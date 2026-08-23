#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::math::{vec3::Vec3, vec2::Vec2, mat4::Mat4};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Axis { X, Y, Z }

#[allow(dead_code)]
#[derive(Clone)]
pub struct Manifold {
    pub axis: Axis,
    pub penetration: f32,
    pub sign: f32,
    pub contact_point: Vec2,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Collider {
    pub vertices: Vec<Vec3>,
    pub local_min: Vec3,
    pub local_max: Vec3,
    pub corners: [Vec3; 8],
    pub aabb_min: Vec3,
    pub aabb_max: Vec3,
}

impl Collider {
    pub fn new(local_min: Vec3, local_max: Vec3) -> Self {
        Collider {
            vertices: Vec::new(),
            local_min,
            local_max,
            corners: [Vec3::new(); 8],
            aabb_min: Vec3::new(),
            aabb_max: Vec3::new(),
        }
    }
    pub fn new_mesh(local_min: Vec3, local_max: Vec3, vertices: Vec<Vec3>) -> Self {
        Collider {
            vertices: vertices,
            local_min,
            local_max,
            corners: [Vec3::new(); 8],
            aabb_min: Vec3::new(),
            aabb_max: Vec3::new(),
        }
    }
    pub fn update(&mut self, mat: Mat4) {
        self.corners = [
            Self::transform(mat, Vec3{ x: self.local_min.x, y: self.local_min.y, z: self.local_min.z }),
            Self::transform(mat, Vec3{ x: self.local_min.x, y: self.local_max.y, z: self.local_min.z }),
            Self::transform(mat, Vec3{ x: self.local_max.x, y: self.local_max.y, z: self.local_min.z }),
            Self::transform(mat, Vec3{ x: self.local_max.x, y: self.local_min.y, z: self.local_min.z }),
            Self::transform(mat, Vec3{ x: self.local_max.x, y: self.local_max.y, z: self.local_max.z }),
            Self::transform(mat, Vec3{ x: self.local_min.x, y: self.local_max.y, z: self.local_max.z }),
            Self::transform(mat, Vec3{ x: self.local_min.x, y: self.local_min.y, z: self.local_max.z }),
            Self::transform(mat, Vec3{ x: self.local_max.x, y: self.local_min.y, z: self.local_max.z }),
        ];
        self.aabb_max = Self::bounds_max(&self.corners);
        self.aabb_min = Self::bounds_min(&self.corners);
    }
    fn transform(m: Mat4, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * m.mat[0] + v.y * m.mat[1] + v.z * m.mat[2] + m.mat[3],
            y: v.x * m.mat[4] + v.y * m.mat[5] + v.z * m.mat[6] + m.mat[7],
            z: v.x * m.mat[8] + v.y * m.mat[9] + v.z * m.mat[10] + m.mat[11],
        }
    }
    fn bounds_max(pts: &[Vec3]) -> Vec3 {
        let mut f = pts[0];
        for p in pts.iter() {
            if p.x > f.x { f.x = p.x; }
            if p.y > f.y { f.y = p.y; }
            if p.z > f.z { f.z = p.z; }
        }
        f
    }
    fn bounds_min(pts: &[Vec3]) -> Vec3 {
        let mut f = pts[0];
        for p in pts.iter() {
            if p.x < f.x { f.x = p.x; }
            if p.y < f.y { f.y = p.y; }
            if p.z < f.z { f.z = p.z; }
        }
        f
    }
    pub fn broad_phase(&self, other: &Collider) -> bool {
        Self::overlap(self.aabb_min.x, self.aabb_max.x, other.aabb_min.x, other.aabb_max.x)
            && Self::overlap(self.aabb_min.y, self.aabb_max.y, other.aabb_min.y, other.aabb_max.y)
            && Self::overlap(self.aabb_min.z, self.aabb_max.z, other.aabb_min.z, other.aabb_max.z)
    }
    fn overlap(min_a: f32, max_a: f32, min_b: f32, max_b: f32) -> bool {
        min_a <= max_b && min_b <= max_a
    }
    pub fn narrow_phase(&self, other: &Collider) -> Option<Manifold> {
        let footprint_a = self.footprint();
        let footprint_b = other.footprint();

        let mut contact = None;
        'outer: for edge_a in footprint_a.iter() {
            for edge_b in footprint_b.iter() {
                if let Some(p) = Self::segment_intersection(edge_a[0], edge_a[1], edge_b[0], edge_b[1]) {
                    contact = Some(p);
                    break 'outer;
                }
            }
        }
        let contact_point = contact?;

        let overlap_x_hi = self.aabb_max.x - other.aabb_min.x;
        let overlap_x_lo = other.aabb_max.x - self.aabb_min.x;
        let overlap_y_hi = self.aabb_max.y - other.aabb_min.y;
        let overlap_y_lo = other.aabb_max.y - self.aabb_min.y;
        let overlap_z_hi = self.aabb_max.z - other.aabb_min.z;
        let overlap_z_lo = other.aabb_max.z - self.aabb_min.z;

        let pen_x = overlap_x_hi.min(overlap_x_lo);
        let pen_y = overlap_y_hi.min(overlap_y_lo);
        let pen_z = overlap_z_hi.min(overlap_z_lo);

        let (axis, penetration, sign) = if pen_x <= pen_y && pen_x <= pen_z {
            (Axis::X, pen_x, if overlap_x_hi < overlap_x_lo { -1.0 } else { 1.0 })
        } else if pen_y <= pen_z {
            (Axis::Y, pen_y, if overlap_y_hi < overlap_y_lo { -1.0 } else { 1.0 })
        } else {
            (Axis::Z, pen_z, if overlap_z_hi < overlap_z_lo { -1.0 } else { 1.0 })
        };

        Some(Manifold { axis, penetration, sign, contact_point })
    }
    fn footprint(&self) -> [[Vec2; 2]; 4] {
        let c = &self.corners;
        let flat = |v: Vec3| Vec2 { x: v.x, y: v.z };
        [
            [flat(c[3]), flat(c[0])],
            [flat(c[0]), flat(c[6])],
            [flat(c[6]), flat(c[7])],
            [flat(c[7]), flat(c[3])],
        ]
    }
    fn segment_intersection(l1p1: Vec2, l1p2: Vec2, l2p1: Vec2, l2p2: Vec2) -> Option<Vec2> {
        let d1 = Vec2{ x: l1p2.x - l1p1.x, y: l1p2.y - l1p1.y };
        let d2 = Vec2{ x: l2p2.x - l2p1.x, y: l2p2.y - l2p1.y };
        let d3 = Vec2{ x: l2p1.x - l1p1.x, y: l2p1.y - l1p1.y };

        let denom = Self::cross(d1, d2);
        const EPS: f32 = 1e-9;
        if denom.abs() < EPS {
            return None;
        }

        let t = Self::cross(d3, d2) / denom;
        let s = Self::cross(d3, d1) / denom;

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&s) {
            Some(Vec2 { x: l1p1.x + t * d1.x, y: l1p1.y + t * d1.y })
        } else {
            None
        }
    }
    fn cross(a: Vec2, b: Vec2) -> f32 { a.x * b.y - a.y * b.x }
}