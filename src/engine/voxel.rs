#![allow(dead_code)]
#![allow(unused_variables)]

use std::u8;

use crate::engine::{loader::{jsonparser::JsonF, rw::{readfs, writefs}}, math::{ivec3::Ivec3, vec3::Vec3}};

#[derive(Clone, Copy)]
pub struct Rledata{
    pub data: u8,
    pub location: u64,
    pub length: u64,
}

#[derive(Clone, Copy)]
pub struct Voxel{
    pos: Ivec3,
    color: [u8; 4],
}

#[derive(Clone)]
pub struct SVO{
    is_leaf: bool,
    color: [u8; 4],
    children: Vec<SVO>,
}

const LEAF_MARKER: u8 = 0;
const BRANCH_MARKER: u8 = u8::MAX;

impl SVO {
    pub fn build_tree_recursive(voxels: Vec<Voxel>, boxmin: Ivec3, boxmax: Ivec3, current_level: u32) -> SVO{
        if current_level == 0 {
            for v in &voxels {
                if v.pos.x >= boxmin.x && v.pos.x <= boxmax.x &&
                   v.pos.y >= boxmin.y && v.pos.y <= boxmax.y &&
                   v.pos.z >= boxmin.z && v.pos.z <= boxmax.z {
                    println!("Leaf voxel found at ({}, {}, {}) with color {:?}", v.pos.x, v.pos.y, v.pos.z, v.color);
                    return SVO { is_leaf: true, color: v.color, children: Vec::new() };
                }
            }
        }
        for v in &voxels {
            if v.pos.x >= boxmin.x && v.pos.x <= boxmax.x &&
               v.pos.y >= boxmin.y && v.pos.y <= boxmax.y &&
               v.pos.z >= boxmin.z && v.pos.z <= boxmax.z {
                println!("Non-leaf voxel found at ({}, {}, {}) with color {:?}", v.pos.x, v.pos.y, v.pos.z, v.color);
                let new_svo_boxes = [
                    Self::build_tree_recursive(voxels.clone(), boxmin, Ivec3 { x: (boxmin.x + boxmax.x) / 2, y: (boxmin.y + boxmax.y) / 2, z: (boxmin.z + boxmax.z) / 2 }, current_level - 1),
                    Self::build_tree_recursive(voxels.clone(), Ivec3 { x: (boxmin.x + boxmax.x) / 2, y: boxmin.y, z: boxmin.z }, Ivec3 { x: boxmax.x, y: (boxmin.y + boxmax.y) / 2, z: (boxmin.z + boxmax.z) / 2 }, current_level - 1),
                    Self::build_tree_recursive(voxels.clone(), Ivec3 { x: boxmin.x, y: (boxmin.y + boxmax.y) / 2, z: boxmin.z }, Ivec3 { x: (boxmin.x + boxmax.x) / 2, y: boxmax.y, z: (boxmin.z + boxmax.z) / 2 }, current_level - 1),
                    Self::build_tree_recursive(voxels.clone(), Ivec3 { x: (boxmin.x + boxmax.x) / 2, y: (boxmin.y + boxmax.y) / 2, z: boxmin.z }, Ivec3 { x: boxmax.x, y: boxmax.y, z: (boxmin.z + boxmax.z) / 2 }, current_level - 1),
                    Self::build_tree_recursive(voxels.clone(), Ivec3 { x: boxmin.x, y: boxmin.y, z: (boxmin.z + boxmax.z) / 2 }, Ivec3 { x: (boxmin.x + boxmax.x) / 2, y: (boxmin.y + boxmax.y) / 2, z: boxmax.z }, current_level - 1),
                    Self::build_tree_recursive(voxels.clone(), Ivec3 { x: (boxmin.x + boxmax.x) / 2, y: boxmin.y, z: (boxmin.z + boxmax.z) / 2 }, Ivec3 { x: boxmax.x, y: (boxmin.y + boxmax.y) / 2, z: boxmax.z }, current_level - 1),
                    Self::build_tree_recursive(voxels.clone(), Ivec3 { x: boxmin.x, y: (boxmin.y + boxmax.y) / 2, z: (boxmin.z + boxmax.z) / 2 }, Ivec3 { x: (boxmin.x + boxmax.x) / 2, y: boxmax.y, z: boxmax.z }, current_level - 1),
                    Self::build_tree_recursive(voxels.clone(), Ivec3 { x: (boxmin.x + boxmax.x) / 2, y: (boxmin.y + boxmax.y) / 2, z: (boxmin.z + boxmax.z) / 2 }, boxmax, current_level - 1),
                ];
                return SVO { is_leaf: false, color: [0, 0, 0, 0], children: new_svo_boxes.to_vec() };
            }
        }
        println!("Empty voxel region at boxmin ({}, {}, {}) and boxmax ({}, {}, {})", boxmin.x, boxmin.y, boxmin.z, boxmax.x, boxmax.y, boxmax.z);
        SVO { is_leaf: true, color: [0, 0, 0, 0], children: Vec::new() }
    }
    pub fn estimate_branch_size(&self) -> usize {
        if self.is_leaf {
            1 + 4
        } else {
            let mut size = 1 + 4;
            for child in &self.children {
                size += child.estimate_branch_size();
            }
            size
        }
    }
    pub fn pack_recursive(&self, packed_data: &mut Vec<u8>) {
        if self.is_leaf {
            packed_data.push(LEAF_MARKER); // Leaf node indicator
            packed_data.extend_from_slice(&self.color);
        } else {
            packed_data.push(BRANCH_MARKER); // Non-leaf node indicator
            let sz = self.estimate_branch_size().to_le();
            packed_data.push(sz as u8);
            packed_data.push((sz >> 8) as u8);
            packed_data.push((sz >> 16) as u8);
            packed_data.push((sz >> 24) as u8);
            for child in &self.children {
                child.pack_recursive(packed_data);
            }
        }
    }
    pub fn extract_3d_texture(&self, levels: u32) -> (Vec<u8>, [u32; 3]) {
        let size = 2u32.pow(levels);
        let dimensions = [size, size, size];
        let total_voxels = (size as usize).pow(3);
        
        let mut texture_data = vec![0u8; total_voxels * 4];
        
        self.fill_3d_texture_recursive(
            &mut texture_data,
            dimensions,
            Ivec3 { x: 0, y: 0, z: 0 },
            Ivec3 { x: size as i32 - 1, y: size as i32 - 1, z: size as i32 - 1 },
            levels,
        );
        
        (texture_data, dimensions)
    }
    fn fill_3d_texture_recursive(&self, texture_data: &mut Vec<u8>, dimensions: [u32; 3], box_min: Ivec3, box_max: Ivec3, current_level: u32,) {
        if self.is_leaf {
            Self::fill_region_with_color(texture_data, dimensions, box_min, box_max, self.color,);
        } else if current_level > 0 && self.children.len() == 8 {
            let mid_x = (box_min.x + box_max.x) / 2;
            let mid_y = (box_min.y + box_max.y) / 2;
            let mid_z = (box_min.z + box_max.z) / 2;
            let octants = [
                (Ivec3 { x: box_min.x, y: box_min.y, z: box_min.z }, Ivec3 { x: mid_x, y: mid_y, z: mid_z }),
                (Ivec3 { x: mid_x + 1, y: box_min.y, z: box_min.z }, Ivec3 { x: box_max.x, y: mid_y, z: mid_z }),
                (Ivec3 { x: box_min.x, y: mid_y + 1, z: box_min.z }, Ivec3 { x: mid_x, y: box_max.y, z: mid_z }),
                (Ivec3 { x: mid_x + 1, y: mid_y + 1, z: box_min.z }, Ivec3 { x: box_max.x, y: box_max.y, z: mid_z }),
                (Ivec3 { x: box_min.x, y: box_min.y, z: mid_z + 1 }, Ivec3 { x: mid_x, y: mid_y, z: box_max.z }),
                (Ivec3 { x: mid_x + 1, y: box_min.y, z: mid_z + 1 }, Ivec3 { x: box_max.x, y: mid_y, z: box_max.z }),
                (Ivec3 { x: box_min.x, y: mid_y + 1, z: mid_z + 1 }, Ivec3 { x: mid_x, y: box_max.y, z: box_max.z }),
                (Ivec3 { x: mid_x + 1, y: mid_y + 1, z: mid_z + 1 }, Ivec3 { x: box_max.x, y: box_max.y, z: box_max.z }),
            ];

            for (i, &(octant_min, octant_max)) in octants.iter().enumerate() {
                if i < self.children.len() {
                    self.children[i].fill_3d_texture_recursive(
                        texture_data,
                        dimensions,
                        octant_min,
                        octant_max,
                        current_level - 1,
                    );
                }
            }
        }
    }
    fn fill_region_with_color(texture_data: &mut Vec<u8>, dimensions: [u32; 3], box_min: Ivec3, box_max: Ivec3, color: [u8; 4]) {
        for z in box_min.z..=box_max.z {
            for y in box_min.y..=box_max.y {
                for x in box_min.x..=box_max.x {
                    let index = Self::compute_texture_index(x as u32, y as u32, z as u32, dimensions);
                    if index + 3 < texture_data.len() {
                        texture_data[index] = color[0];
                        texture_data[index + 1] = color[1];
                        texture_data[index + 2] = color[2];
                        texture_data[index + 3] = color[3];
                    }
                }
            }
        }
    }
    pub fn get_texture_at_position(&self, pos: Ivec3, box_min: Ivec3, box_max: Ivec3, current_level: u32,) -> [u8; 4] {
        if self.is_leaf {
            return self.color;
        }

        if current_level == 0 {
            return [0, 0, 0, 0];
        }

        let mid_x = (box_min.x + box_max.x) / 2;
        let mid_y = (box_min.y + box_max.y) / 2;
        let mid_z = (box_min.z + box_max.z) / 2;

        let octant_index = Self::get_octant_index(pos, mid_x, mid_y, mid_z);

        if octant_index < self.children.len() {
            let (octant_min, octant_max) = Self::get_octant_bounds(
                octant_index,
                box_min,
                box_max,
                mid_x,
                mid_y,
                mid_z,
            );
            self.children[octant_index].get_texture_at_position(pos, octant_min, octant_max, current_level - 1)
        } else {
            [0, 0, 0, 0]
        }
    }
    fn compute_texture_index(x: u32, y: u32, z: u32, dimensions: [u32; 3]) -> usize {
        ((z * dimensions[1] * dimensions[0]) + (y * dimensions[0]) + x) as usize * 4
    }
    fn get_octant_index(pos: Ivec3, mid_x: i32, mid_y: i32, mid_z: i32) -> usize {
        let mut index = 0;
        if pos.x > mid_x { index |= 1; }
        if pos.y > mid_y { index |= 2; }
        if pos.z > mid_z { index |= 4; }
        index
    }
    fn get_octant_bounds(octant_index: usize, box_min: Ivec3, box_max: Ivec3, mid_x: i32, mid_y: i32, mid_z: i32,) -> (Ivec3, Ivec3) {
        let min_x = if octant_index & 1 == 0 { box_min.x } else { mid_x + 1 };
        let max_x = if octant_index & 1 == 0 { mid_x } else { box_max.x };
        let min_y = if octant_index & 2 == 0 { box_min.y } else { mid_y + 1 };
        let max_y = if octant_index & 2 == 0 { mid_y } else { box_max.y };
        let min_z = if octant_index & 4 == 0 { box_min.z } else { mid_z + 1 };
        let max_z = if octant_index & 4 == 0 { mid_z } else { box_max.z };

        (
            Ivec3 { x: min_x, y: min_y, z: min_z },
            Ivec3 { x: max_x, y: max_y, z: max_z },
        )
    }
    pub fn extract_3d_texture_from_packed(packed_data: &[u8], levels: u32,) -> (Vec<u8>, [u32; 3]) {
        let size = 2u32.pow(levels);
        let dimensions = [size, size, size];
        let total_voxels = (size as usize).pow(3);
        
        let mut texture_data = vec![0u8; total_voxels * 4];
        let mut cursor = 0;
        
        Self::fill_3d_texture_from_packed_recursive(
            packed_data,
            &mut cursor,
            &mut texture_data,
            dimensions,
            Ivec3 { x: 0, y: 0, z: 0 },
            Ivec3 { x: size as i32 - 1, y: size as i32 - 1, z: size as i32 - 1 },
            levels,
        );
        
        (texture_data, dimensions)
    }
    fn fill_3d_texture_from_packed_recursive(packed_data: &[u8], cursor: &mut usize, texture_data: &mut Vec<u8>, dimensions: [u32; 3], box_min: Ivec3, box_max: Ivec3, current_level: u32,) {
        if *cursor >= packed_data.len() {
            return;
        }

        let marker = packed_data[*cursor];
        *cursor += 1;

        if marker == LEAF_MARKER {
            // Read 4 bytes of RGBA color
            if *cursor + 4 <= packed_data.len() {
                let color = [
                    packed_data[*cursor],
                    packed_data[*cursor + 1],
                    packed_data[*cursor + 2],
                    packed_data[*cursor + 3],
                ];
                *cursor += 4;

                // Fill entire region with this leaf color
                Self::fill_region_with_color(texture_data, dimensions, box_min, box_max, color);
            }
        } else if marker == BRANCH_MARKER && current_level > 0 {
            // Read 4-byte size in little-endian format (we mainly skip past it)
            if *cursor + 4 <= packed_data.len() {
                let _branch_size = u32::from_le_bytes([
                    packed_data[*cursor],
                    packed_data[*cursor + 1],
                    packed_data[*cursor + 2],
                    packed_data[*cursor + 3],
                ]);
                *cursor += 4;

                // Subdivide into 8 octants
                let mid_x = (box_min.x + box_max.x) / 2;
                let mid_y = (box_min.y + box_max.y) / 2;
                let mid_z = (box_min.z + box_max.z) / 2;

                let octants = [
                    (Ivec3 { x: box_min.x, y: box_min.y, z: box_min.z }, Ivec3 { x: mid_x, y: mid_y, z: mid_z }),
                    (Ivec3 { x: mid_x + 1, y: box_min.y, z: box_min.z }, Ivec3 { x: box_max.x, y: mid_y, z: mid_z }),
                    (Ivec3 { x: box_min.x, y: mid_y + 1, z: box_min.z }, Ivec3 { x: mid_x, y: box_max.y, z: mid_z }),
                    (Ivec3 { x: mid_x + 1, y: mid_y + 1, z: box_min.z }, Ivec3 { x: box_max.x, y: box_max.y, z: mid_z }),
                    (Ivec3 { x: box_min.x, y: box_min.y, z: mid_z + 1 }, Ivec3 { x: mid_x, y: mid_y, z: box_max.z }),
                    (Ivec3 { x: mid_x + 1, y: box_min.y, z: mid_z + 1 }, Ivec3 { x: box_max.x, y: mid_y, z: box_max.z }),
                    (Ivec3 { x: box_min.x, y: mid_y + 1, z: mid_z + 1 }, Ivec3 { x: mid_x, y: box_max.y, z: box_max.z }),
                    (Ivec3 { x: mid_x + 1, y: mid_y + 1, z: mid_z + 1 }, Ivec3 { x: box_max.x, y: box_max.y, z: box_max.z }),
                ];

                // Process all 8 children
                for (octant_min, octant_max) in &octants {
                    Self::fill_3d_texture_from_packed_recursive(
                        packed_data,
                        cursor,
                        texture_data,
                        dimensions,
                        *octant_min,
                        *octant_max,
                        current_level - 1,
                    );
                }
            }
        }
    }
    pub fn extract_3d_texture_mipmaps_from_packed(packed_data: &[u8], max_levels: u32) -> Vec<(Vec<u8>, [u32; 3])> {
        let mut mipmaps = Vec::new();
        
        for level in 0..=max_levels {
            let (texture, dims) = Self::extract_3d_texture_from_packed(packed_data, level);
            mipmaps.push((texture, dims));
        }
        
        mipmaps
    }
}

pub struct VoxelScene{
    pub data: Vec<u8>,
    pub voxels: Vec<Voxel>,
    pub levels: u32,
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
    fn compress_data_impl(data: &[u8]) -> (Vec<Rledata>, Vec<u8>) {
        let mut entries = Vec::new();
        let mut tail = Vec::new();
        let mut i = 0usize;

        while i < data.len() {
            let current = data[i];
            let mut run_len = 1usize;
            while i + run_len < data.len() && data[i + run_len] == current {
                run_len += 1;
            }

            if run_len > 17 {
                entries.push(Rledata {
                    data: current,
                    location: i as u64,
                    length: run_len as u64,
                });
                i += run_len;
            } else {
                tail.extend_from_slice(&data[i..(i + run_len)]);
                i += run_len;
            }
        }

        (entries, tail)
    }
    fn decompress_data_impl(entries: &[Rledata], tail: &[u8], total_len: usize) -> Vec<u8> {
        let mut result = vec![0u8; total_len];
        let mut sorted_entries = entries.to_vec();
        sorted_entries.sort_by_key(|entry| entry.location);
        let mut raw_index = 0usize;
        let mut cursor = 0usize;
        for e in &sorted_entries {
            let start = e.location as usize;
            if start > cursor {
                let n = start - cursor;
                result[cursor..start].copy_from_slice(&tail[raw_index..raw_index + n]);
                raw_index += n;
            }
            let end = start + e.length as usize;
            result[start..end].fill(e.data);
            cursor = end;
        }
        if cursor < total_len {
            let n = total_len - cursor;
            result[cursor..total_len].copy_from_slice(&tail[raw_index..raw_index + n]);
        }

        result
    }
    pub fn new_blank() -> VoxelScene{
        VoxelScene { data: vec![], voxels: Vec::new(), levels: 0, voxel_size: 1.0, origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, size: [0, 0, 0] }
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
                upper_boundaries[0] = x.round() as i32 + 1;
            }
            if x < lower_boundaries[0] as f32{
                lower_boundaries[0] = x.round() as i32 - 1;
            }
            if y > upper_boundaries[1] as f32{
                upper_boundaries[1] = y.round() as i32 + 1;
            }
            if y < lower_boundaries[1] as f32{
                lower_boundaries[1] = y.round() as i32 - 1;
            }
            if z > upper_boundaries[2] as f32{
                upper_boundaries[2] = z.round() as i32 + 1;
            }
            if z < lower_boundaries[2] as f32{
                lower_boundaries[2] = z.round() as i32 - 1;
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
    pub fn set(&mut self, x: u32, y: u32, z: u32, color: [u8; 4], vx: bool) {
        if x < self.size[0] && y < self.size[1] && z < self.size[2] {
            if vx {
                self.voxels.push(Voxel { pos: Ivec3 { x: x as i32, y: y as i32, z: z as i32 }, color });
            }else{
                self.data[((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize)*4] = color[0];
                self.data[((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize)*4 + 1] = color[1];
                self.data[((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize)*4 + 2] = color[2];
                self.data[((x + y * self.size[0] + z * self.size[1] * self.size[0]) as usize)*4 + 3] = color[3];
            }
        }
    }
    pub fn voxelize_triangle(&mut self, v0: Vec3, v1: Vec3, v2: Vec3, color: [u8; 4], voxel_set: bool) {
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
                        self.set(vx as u32, vy as u32, vz as u32, color, voxel_set);
                    }
                }
            }
        }
    }
    pub fn voxelize_triangles(&mut self, vertices: Vec<f32>, tricolor: Vec<u8>, voxel_set: bool) {
        for i in (0..vertices.len()).step_by(9) {
            self.voxelize_triangle(
                Vec3 { x: vertices[i], y: vertices[i+1], z: vertices[i+2] }, 
                Vec3 { x: vertices[i+3], y: vertices[i+4], z: vertices[i+5] }, 
                Vec3 { x: vertices[i+6], y: vertices[i+7], z: vertices[i+8] },
                [tricolor[(i/9)*4], tricolor[(i/9)*4+1], tricolor[(i/9)*4+2], tricolor[(i/9)*4+3]],
                voxel_set
            );
        }
    }
    pub fn new(voxel_representation: Vec<u8>, voxel_size: f32, size: [u32; 3], origin: Vec3, levels: u32) -> VoxelScene{
        VoxelScene { data: voxel_representation, voxels: Vec::new(), levels: levels, voxel_size: voxel_size, origin: origin, size }
    }
    pub fn from_vertices(vertices: Vec<f32>, voxel_size: f32, tricolor: Vec<u8>) -> VoxelScene{
        let bond = Self::get_boundaries(vertices.clone());
        let size = [
            ((bond.1[0] - bond.0[0]).abs() as f32/voxel_size) as u32, 
            ((bond.1[1] - bond.0[1]).abs() as f32/voxel_size) as u32, 
            ((bond.1[2] - bond.0[2]).abs() as f32/voxel_size) as u32];
        let mut scn = Self::new(vec![], voxel_size, size, Vec3 { x: bond.0[0] as f32, y: bond.0[1] as f32, z: bond.0[2] as f32 }, 0);
        scn.data.resize(((size[0]*size[1]*size[2]*4) as f32/voxel_size.powi(3)) as usize, 0);
        scn.voxelize_triangles(vertices, tricolor, false);
        scn
    }
    pub fn from_vertices_svo(vertices: Vec<f32>, tricolor: Vec<u8>, levels: u32) -> VoxelScene{
        let bond = Self::get_boundaries(vertices.clone());
        let dsize = [
            (bond.1[0] - bond.0[0]).abs() as f32 as u32, 
            (bond.1[1] - bond.0[1]).abs() as f32 as u32, 
            (bond.1[2] - bond.0[2]).abs() as f32 as u32];
        let size = [
            2u32.pow(levels), 
            2u32.pow(levels), 
            2u32.pow(levels)];
        let voxel_size = *dsize.iter().max().unwrap_or(&1) as f32 / size[0] as f32;
        let mut scn = Self::new(vec![], voxel_size, size, Vec3 { x: bond.0[0] as f32, y: bond.0[1] as f32, z: bond.0[2] as f32 }, levels);
        //scn.data.resize(((size[0]*size[1]*size[2]*4) as f32) as usize, 0);
        scn.voxelize_triangles(vertices, tricolor, true);
        println!("Voxels generated: {}", scn.voxels.len());
        let svo = SVO::build_tree_recursive(scn.voxels.clone(), Ivec3 { x: 0, y: 0, z: 0 }, Ivec3 { x: size[0] as i32, y: size[1] as i32, z: size[2] as i32 }, levels);
        println!("SVO built with {} children", svo.children.len());
        //scn.data = svo.pack_recursive(packed_data);
        svo.pack_recursive(&mut scn.data);
        println!("SVO packed with {} bytes", scn.data.len());
        scn
    }
    pub fn from_file(path: &str) -> VoxelScene{
        let cont = readfs(path);
        let jsonend = u32::from_ne_bytes([cont[0], cont[1], cont[2], cont[3]]);
        let rjson = cont[4..jsonend as usize].to_vec();
        let mut size = [0u32, 0u32, 0u32];
        let mut origin = Vec3{ x: 0.0, y: 0.0, z: 0.0};
        let mut voxelsize = 1.0f32;
        let mut compression_active = false;
        let json = JsonF::from_text(&String::from_utf8(rjson).unwrap());
        let mut lvls = 0u32;
        let mut ln = 0u32;
        for i in 0..json.other_nodes.len() {
            match json.other_nodes[i].name.as_str() {
                "voxel_size" => voxelsize = json.other_nodes[i].numeral_val as f32,
                "size_x" => size[0] = json.other_nodes[i].numeral_val as u32,
                "size_y" => size[1] = json.other_nodes[i].numeral_val as u32,
                "size_z" => size[2] = json.other_nodes[i].numeral_val as u32,
                "origin_x" => origin.x = json.other_nodes[i].numeral_val as f32,
                "origin_y" => origin.y = json.other_nodes[i].numeral_val as f32,
                "origin_z" => origin.z = json.other_nodes[i].numeral_val as f32,
                "compression_active" => compression_active = json.other_nodes[i].bolean,
                "levels" =>lvls = json.other_nodes[i].numeral_val as u32,
                "data_size" => ln = json.other_nodes[i].numeral_val as u32,
                _ => {}
            }
        }
        let payload = cont[(jsonend) as usize..cont.len()].to_vec();
        let data = if compression_active {
            if payload.len() < 8 {
                vec![]
            } else {
                let count = u64::from_ne_bytes(payload[0..8].try_into().unwrap()) as usize;
                let mut cursor = 8usize;
                let mut entries = Vec::new();
                for _ in 0..count {
                    if cursor + 17 > payload.len() {
                        break;
                    }
                    let mut data_byte = [0u8; 1];
                    data_byte.copy_from_slice(&payload[cursor..(cursor + 1)]);
                    let mut location_bytes = [0u8; 8];
                    location_bytes.copy_from_slice(&payload[(cursor + 1)..(cursor + 9)]);
                    let mut length_bytes = [0u8; 8];
                    length_bytes.copy_from_slice(&payload[(cursor + 9)..(cursor + 17)]);
                    entries.push(Rledata {
                        data: data_byte[0],
                        location: u64::from_ne_bytes(location_bytes),
                        length: u64::from_ne_bytes(length_bytes),
                    });
                    cursor += 17;
                }
                let tail = payload[cursor..payload.len()].to_vec();
                let expected_len = ((size[0] as f32) * (size[1] as f32) * (size[2] as f32) / voxelsize.powi(3)) as usize * 4;
                if ln == 0{
                    ln = expected_len as u32;
                }
                Self::decompress_data_impl(&entries, &tail, ln as usize)
            }
        } else {
            payload
        };
        VoxelScene::new(data, voxelsize, size, origin, lvls)
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
        s.push_str(&format!("  \"levels\": {},\n", self.levels));
        s.push_str(&format!("  \"data_size\": {},\n", self.data.len()));
        let (entries, tail) = Self::compress_data_impl(&self.data);
        let compression_active = !entries.is_empty();
        s.push_str(&format!("  \"compression_active\": {}\n", if compression_active { "true" } else { "false" }));
        s.push_str("}");
        let beggoff = ((s.len()+4) as u32).to_ne_bytes();
        let mut rwd = beggoff.clone().to_vec();
        rwd.append(&mut s.into_bytes());

        let (entries, tail) = Self::compress_data_impl(&self.data);
        let compression_active = !entries.is_empty();
        if compression_active {
            let mut payload = Vec::new();
            payload.extend_from_slice(&(entries.len() as u64).to_ne_bytes());
            for entry in &entries {
                let mut entry_bytes = [0u8; 17];
                entry_bytes[0] = entry.data;
                entry_bytes[1..9].copy_from_slice(&entry.location.to_ne_bytes());
                entry_bytes[9..17].copy_from_slice(&entry.length.to_ne_bytes());
                payload.extend_from_slice(&entry_bytes);
            }
            payload.extend_from_slice(&tail);
            rwd.append(&mut payload);
        } else {
            rwd.append(&mut self.data.clone());
        }

        writefs(path, rwd);
    }
}