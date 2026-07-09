#![allow(dead_code)]
#![allow(unused_variables)]

use std::u8;

use crate::engine::{loader::{jsonparser::JsonF, rw::{readfs, writefs}}, math::vec3::Vec3};

pub struct VoxelScene{
    pub data: Vec<u8>,
    pub size: [u32; 3],
    pub origin: Vec3,
    pub voxel_size: f32,
}

pub fn voxel_overlaps_triangle(voxel_center: Vec3, voxel_size: f32, v0: Vec3, v1: Vec3, v2: Vec3, dominant_axis: usize) -> bool {
    let half = voxel_size * 0.5;

    let (c, t0, t1, t2) = match dominant_axis {
        0 => (
            [voxel_center.y, voxel_center.z],
            [v0.y, v0.z],
            [v1.y, v1.z],
            [v2.y, v2.z],
        ),
        1 => (
            [voxel_center.x, voxel_center.z],
            [v0.x, v0.z],
            [v1.x, v1.z],
            [v2.x, v2.z],
        ),
        _ => (
            [voxel_center.x, voxel_center.y],
            [v0.x, v0.y],
            [v1.x, v1.y],
            [v2.x, v2.y],
        ),
    };

    // 2D separating axis test between the square voxel and projected triangle
    // Tests triangle edges as potential separating axes
    let edges = [
        [t1[0] - t0[0], t1[1] - t0[1]],
        [t2[0] - t1[0], t2[1] - t1[1]],
        [t0[0] - t2[0], t0[1] - t2[1]],
    ];

    for edge in &edges {
        // Normal to this edge (perpendicular in 2D)
        let axis = [-edge[1], edge[0]];

        // Project triangle vertices onto axis
        let p0 = axis[0] * t0[0] + axis[1] * t0[1];
        let p1 = axis[0] * t1[0] + axis[1] * t1[1];
        let p2 = axis[0] * t2[0] + axis[1] * t2[1];

        let tri_min = p0.min(p1).min(p2);
        let tri_max = p0.max(p1).max(p2);

        // Project voxel center onto axis, radius is half-diagonal on this axis
        let vox_proj   = axis[0] * c[0] + axis[1] * c[1];
        let vox_radius = half * (axis[0].abs() + axis[1].abs());

        // Separating axis found — no overlap
        if tri_max < vox_proj - vox_radius || tri_min > vox_proj + vox_radius {
            return false;
        }
    }

    // Also test the two AABB axes (X and Y of projection plane)
    for axis in [[1.0f32, 0.0f32], [0.0, 1.0]] {
        let p0 = axis[0] * t0[0] + axis[1] * t0[1];
        let p1 = axis[0] * t1[0] + axis[1] * t1[1];
        let p2 = axis[0] * t2[0] + axis[1] * t2[1];

        let tri_min    = p0.min(p1).min(p2);
        let tri_max    = p0.max(p1).max(p2);
        let vox_proj   = axis[0] * c[0] + axis[1] * c[1];

        if tri_max < vox_proj - half || tri_min > vox_proj + half {
            return false;
        }
    }

    true
}

impl VoxelScene{
    pub fn new_blank() -> VoxelScene{
        VoxelScene { data: vec![], voxel_size: 1.0, origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, size: [0, 0, 0] }
    }
    pub fn get_boundaries(vertices: Vec<f32>) -> ([i32; 3], [i32; 3]){
        let mut upper_boundaries = [vertices[0].round() as i32, vertices[1].round() as i32, vertices[2].round() as i32];
        let mut lower_boundaries = [vertices[0].round() as i32, vertices[1].round() as i32, vertices[2].round() as i32];
        let vl = vertices.len()/3;
        for i in 0..vl{
            let x = vertices[i*3 + 0];
            let y = vertices[i*3 + 1];
            let z = vertices[i*3 + 2];
            if x > upper_boundaries[0] as f32{
                upper_boundaries[0] = x.round() as i32;
            }
            if x < lower_boundaries[0] as f32{
                lower_boundaries[0] = x.round() as i32;
            }
            if y > upper_boundaries[1] as f32{
                upper_boundaries[1] = y.round() as i32;
            }
            if y < lower_boundaries[1] as f32{
                lower_boundaries[1] = y.round() as i32;
            }
            if z > upper_boundaries[2] as f32{
                upper_boundaries[2] = z.round() as i32;
            }
            if z < lower_boundaries[2] as f32{
                lower_boundaries[2] = z.round() as i32;
            }
        }
        (lower_boundaries, upper_boundaries)
    }
    pub fn world_to_voxel(&self, pos: Vec3) -> (isize, isize, isize) {
        let mut local = pos - self.origin;
        local.x /= self.voxel_size;
        local.y /= self.voxel_size;
        local.z /= self.voxel_size;
        (local.x as isize, local.y as isize, local.z as isize)
    }
    pub fn set(&mut self, x: u32, y: u32, z: u32) {
        if x < self.size[0] && y < self.size[1] && z < self.size[2] {
            self.data[((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize)*4] = u8::MAX;
            self.data[((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize)*4 + 1] = u8::MAX;
            self.data[((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize)*4 + 2] = u8::MAX;
            self.data[((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize)*4 + 3] = u8::MAX;
        }
    }
    pub fn voxelize_triangle(&mut self, v0: Vec3, v1: Vec3, v2: Vec3) {
        let edge0  = v1 - v0;
        let edge1  = v2 - v0;
        let normal = edge0.cross(edge1);

        let abs_n  = normal.abs();
        let dominant_axis = if abs_n.x >= abs_n.y && abs_n.x >= abs_n.z {
            0
        } else if abs_n.y >= abs_n.z {
            1
        } else {
            2
        };
        let tri_min = v0.min(v1).min(v2);
        let tri_max = v0.max(v1).max(v2);

        let (min_vx, min_vy, min_vz) = self.world_to_voxel(tri_min);
        let (max_vx, max_vy, max_vz) = self.world_to_voxel(tri_max);
        let x0 = (min_vx).max(0) as usize;
        let y0 = (min_vy).max(0) as usize;
        let z0 = (min_vz).max(0) as usize;
        let x1 = (max_vx + 1).min(self.size[0] as isize - 1) as usize;
        let y1 = (max_vy + 1).min(self.size[1] as isize - 1) as usize;
        let z1 = (max_vz + 1).min(self.size[2] as isize - 1) as usize;
        for vz in z0..=z1 {
            for vy in y0..=y1 {
                for vx in x0..=x1 {
                    let voxel_center = self.origin + Vec3{
                        x: (vx as f32 + 0.5) * self.voxel_size,
                        y: (vy as f32 + 0.5) * self.voxel_size,
                        z: (vz as f32 + 0.5) * self.voxel_size,
                    };

                    if voxel_overlaps_triangle(
                        voxel_center,
                        self.voxel_size,
                        v0, v1, v2,
                        dominant_axis,
                    ) {
                        self.set(vx as u32, vy as u32, vz as u32);
                    }
                }
            }
        }
    }
    pub fn voxelize_triangles(&mut self, vertices: Vec<f32>) {
        for i in (0..vertices.len()).step_by(9) {
            self.voxelize_triangle(
                Vec3 { x: vertices[i], y: vertices[i+1], z: vertices[i+2] }, 
                Vec3 { x: vertices[i+3], y: vertices[i+4], z: vertices[i+5] }, 
                Vec3 { x: vertices[i+6], y: vertices[i+7], z: vertices[i+8] }
            );
        }
    }
    pub fn new(voxel_representation: Vec<u8>, voxel_size: f32, size: [u32; 3], origin: Vec3) -> VoxelScene{
        VoxelScene { data: voxel_representation, voxel_size: voxel_size, origin: origin, size }
    }
    pub fn from_vertices(vertices: Vec<f32>, voxel_size: f32) -> VoxelScene{
        let bond = Self::get_boundaries(vertices.clone());
        let size = [(bond.1[0] - bond.0[0]).abs() as u32, (bond.1[1] - bond.0[1]).abs() as u32, (bond.1[2] - bond.0[2]).abs() as u32];
        let mut scn = Self::new(vec![], voxel_size, size, Vec3 { x: bond.0[0] as f32, y: bond.0[1] as f32, z: bond.0[2] as f32 });
        scn.data.resize(((size[0]*size[1]*size[2]*4) as f32/voxel_size) as usize, 0);
        scn.voxelize_triangles(vertices);
        scn
    }
    pub fn from_file(path: &str) -> VoxelScene{
        let cont = readfs(path);
        let jsonend = u32::from_ne_bytes([cont[0], cont[1], cont[2], cont[3]]);
        let rjson = cont[4..jsonend as usize].to_vec();
        let mut size = [0u32, 0u32, 0u32];
        let mut origin = Vec3{ x: 0.0, y: 0.0, z: 0.0};
        let mut voxelsize = 1.0f32;
        let json = JsonF::from_text(&String::from_utf8(rjson).unwrap());
        for i in 0..json.other_nodes.len() {
            match json.other_nodes[i].name.as_str() {
                "voxel_size" => voxelsize = json.other_nodes[i].numeral_val as f32,
                "size_x" => size[0] = json.other_nodes[i].numeral_val as u32,
                "size_y" => size[1] = json.other_nodes[i].numeral_val as u32,
                "size_z" => size[2] = json.other_nodes[i].numeral_val as u32,
                "origin_x" => origin.x = json.other_nodes[i].numeral_val as f32,
                "origin_y" => origin.y = json.other_nodes[i].numeral_val as f32,
                "origin_z" => origin.z = json.other_nodes[i].numeral_val as f32,
                _ => {}
            }
        }
        let cl = cont.len();
        let data = cont[(jsonend) as usize..cl].to_vec();
        VoxelScene::new(data, voxelsize, size, origin)
    }
    pub fn save_file(&mut self, path: &str){
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str(&format!("  \"voxel_size\": {},\n", self.voxel_size));
        s.push_str(&format!("  \"size_x\": {},\n", self.size[0]));
        s.push_str(&format!("  \"size_y\": {},\n", self.size[1]));
        s.push_str(&format!("  \"size_z\": {},\n", self.size[2]));
        s.push_str(&format!("  \"origin_x\": {},\n", self.origin.x));
        s.push_str(&format!("  \"origin_y\": {},\n", self.origin.y));
        s.push_str(&format!("  \"origin_z\": {},\n", self.origin.z));
        s.push_str("}");
        let beggoff = ((s.len()+4) as u32).to_ne_bytes();
        let mut rwd = beggoff.clone().to_vec();
        rwd.append(&mut s.into_bytes());
        rwd.append(&mut self.data.clone());
        writefs(path, rwd);
    }
}