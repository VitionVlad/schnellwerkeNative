#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::math::{vec3::Vec3, vec2::Vec2, mat4::Mat4};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Axis { X, Y, Z }

#[allow(dead_code)]
#[derive(Clone)]
pub struct Manifold {
    pub axis: Vec3,
    pub penetration: f32,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Obb {
    pub center: Vec3,
    pub half_extents: Vec3,
    pub axes: [Vec3; 3],
}

fn cross3(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 { x: a.y*b.z - a.z*b.y, y: a.z*b.x - a.x*b.z, z: a.x*b.y - a.y*b.x }
}
fn dot3(a: Vec3, b: Vec3) -> f32 { a.x*b.x + a.y*b.y + a.z*b.z }
fn length3(v: Vec3) -> f32 { dot3(v, v).sqrt() }

pub fn sat_test(a: &Obb, b: &Obb) -> Option<Manifold> {
    let t = Vec3 { x: b.center.x - a.center.x, y: b.center.y - a.center.y, z: b.center.z - a.center.z };

    let mut axes: Vec<Vec3> = Vec::with_capacity(15);
    axes.extend_from_slice(&a.axes);
    axes.extend_from_slice(&b.axes);
    for ai in &a.axes {
        for bi in &b.axes {
            let c = cross3(*ai, *bi);
            let len = length3(c);
            if len > 1e-5 {
                axes.push(Vec3 { x: c.x / len, y: c.y / len, z: c.z / len });
            }
        }
    }

    let mut best_axis = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut best_overlap = f32::MAX;

    for axis in axes {
        let ra = a.half_extents.x * dot3(a.axes[0], axis).abs()
                + a.half_extents.y * dot3(a.axes[1], axis).abs()
                + a.half_extents.z * dot3(a.axes[2], axis).abs();
        let rb = b.half_extents.x * dot3(b.axes[0], axis).abs()
                + b.half_extents.y * dot3(b.axes[1], axis).abs()
                + b.half_extents.z * dot3(b.axes[2], axis).abs();

        let separation = dot3(t, axis).abs();
        let overlap = ra + rb - separation;

        if overlap < 0.0 {
            return None; // this axis separates them — no collision, bail out early
        }
        if overlap < best_overlap {
            best_overlap = overlap;
            best_axis = if dot3(t, axis) < 0.0 {
                Vec3 { x: -axis.x, y: -axis.y, z: -axis.z }
            } else {
                axis
            };
        }
    }

    Some(Manifold { axis: best_axis, penetration: best_overlap })
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
    pub obb: Obb,
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
            obb: Obb { center: Vec3::new(), half_extents: Vec3::new(), axes: [Vec3::new(); 3] }
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
            obb: Obb { center: Vec3::new(), half_extents: Vec3::new(), axes: [Vec3::new(); 3] }
        }
    }
    pub fn to_obb(&self) -> Obb {
        let c0 = self.corners[0];
        let sub = |p: Vec3| Vec3 { x: p.x - c0.x, y: p.y - c0.y, z: p.z - c0.z };
        let norm = |v: Vec3| { let l = length3(v); (Vec3 { x: v.x/l, y: v.y/l, z: v.z/l }, l) };

        let (ax, lx) = norm(sub(self.corners[3])); // v2.x edge
        let (ay, ly) = norm(sub(self.corners[1])); // v2.y edge
        let (az, lz) = norm(sub(self.corners[6])); // v2.z edge

        Obb {
            center: Vec3 {
                x: (self.corners[0].x + self.corners[4].x) * 0.5,
                y: (self.corners[0].y + self.corners[4].y) * 0.5,
                z: (self.corners[0].z + self.corners[4].z) * 0.5,
            },
            half_extents: Vec3 { x: lx * 0.5, y: ly * 0.5, z: lz * 0.5 },
            axes: [ax, ay, az],
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
        self.obb = self.to_obb();
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
        sat_test(&self.obb, &other.obb)
    }
    fn footprint(&self) -> [[Vec2; 2]; 6] {
        let c = &self.corners;
        let flat = |v: Vec3| Vec2 { x: v.x, y: v.z };
        [
            [flat(c[0]), flat(c[7])],
            [flat(c[1]), flat(c[4])],
            [flat(c[0]), flat(c[5])],
            [flat(c[3]), flat(c[4])],
            [flat(c[0]), flat(c[2])],
            [flat(c[6]), flat(c[4])],
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