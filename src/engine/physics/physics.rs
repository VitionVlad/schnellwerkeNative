#![allow(dead_code)]
#![allow(unused_variables)]
use crate::engine::math::vec2::Vec2;

use crate::engine::math::vec4::Vec4;
use crate::engine::math::{mat4::Mat4, vec3::Vec3};

use crate::engine::physics::collision::*;

const GRAVITATIONAL_ACCELERATION: f32 = 9.81f32;

const TIE_EPS: f32 = 0.001;

const ANGULAR_DAMPING: f32 = 2.0;

const SETTLE_VEL_EPS: f32 = 0.01;

#[allow(dead_code)]
pub fn check_for_intersection(x1: f32, x2: f32, y1: f32, y2: f32) -> bool{
    return x1 <= y2 && y1 <= x2;
}

pub fn distance(v1: Vec3, v2: Vec3) -> f32{
  f32::sqrt((v2.x - v1.x).powi(2) + (v2.z - v1.z).powi(2))
}

#[allow(dead_code)]
pub fn getpoints(v: Vec<f32>) -> Vec<Vec3>{
    let mut v1 = Vec3{ x: v[0], y: v[1], z: v[2]};
    let mut v2 = Vec3{ x: v[0], y: v[1], z: v[2]};
    for i in (0..v.len()/8*3).step_by(3){
        if v[i] > v1.x {
            v1.x = v[i];
        }
        if v[i+1] > v1.y {
            v1.y = v[i+1];
        }
        if v[i+2] > v1.z {
            v1.z = v[i+2];
        }
        if v[i] < v2.x {
            v2.x = v[i];
        }
        if v[i+1] < v2.y {
            v2.y = v[i+1];
        }
        if v[i+2] < v2.z {
            v2.z = v[i+2];
        }
    }
    return vec![v1, v2];
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct PhysicsObject{
    pub collider: Collider,
    pub acceleration: Vec3,
    pub speed: Vec3,
    pub is_static: bool,
    pub is_interacting: bool,
    pub elasticity: f32,
    pub gravity: bool,
    pub air_friction: f32,
    pub pos: Vec3,
    pub rot: Vec4,
    pub angular_velocity: Vec3,
    pub angular_acceleration: Vec3,
    pub scale: Vec3,
    pub mat: Mat4,
    pub solid: bool,
    pub mass: f32,
    pub enable_rotation: bool,
    pub step_height: f32,
    oldpos: Vec3,
    oldrot: Vec4,
    oldscale: Vec3,
    intersectionp: Vec2,
    pub hit: bool,
    pub pin_pos: bool,
    standing_on: bool,
    pub compute_torque: bool,
}

impl PhysicsObject{
    #[allow(dead_code)]
    pub fn new(v: Vec<Vec3>, is_static: bool) -> PhysicsObject{
        PhysicsObject{
            collider: Collider::new(v[1], v[0]),
            acceleration: Vec3::new(),
            speed: Vec3::new(),
            is_static: is_static,
            is_interacting: false,
            elasticity: 0.0f32,
            gravity: true,
            air_friction: 0.005f32,
            pos: Vec3::new(),
            rot: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            angular_velocity: Vec3::new(),
            angular_acceleration: Vec3::new(),
            scale: Vec3{ x: 1f32, y: 1f32, z: 1f32},
            mat: Mat4::new(),
            solid: true,
            mass: 25.0f32,
            enable_rotation: true,
            step_height: 0.5f32,
            oldpos: Vec3::new(),
            oldrot: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            oldscale: Vec3::new(),
            intersectionp: Vec2::new(),
            hit: false,
            pin_pos: false,
            standing_on: false,
            compute_torque: false,
        }
    }
    #[allow(dead_code)]
    pub fn new_mesh(v: Vec<Vec3>, is_static: bool, raw_vertices: Vec<Vec3>) -> PhysicsObject{
        PhysicsObject{
            collider: Collider::new_mesh(v[1], v[0], raw_vertices),
            acceleration: Vec3::new(),
            speed: Vec3::new(),
            is_static: is_static,
            is_interacting: false,
            elasticity: 0.0f32,
            gravity: true,
            air_friction: 0.005f32,
            pos: Vec3::new(),
            rot: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            angular_velocity: Vec3::new(),
            angular_acceleration: Vec3::new(),
            scale: Vec3{ x: 1f32, y: 1f32, z: 1f32},
            mat: Mat4::new(),
            solid: true,
            mass: 25.0f32,
            enable_rotation: true,
            step_height: 0.0f32,
            oldpos: Vec3::new(),
            oldrot: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            oldscale: Vec3::new(),
            intersectionp: Vec2::new(),
            hit: false,
            pin_pos: false,
            standing_on: false,
            compute_torque: true,
        }
    }
    #[allow(dead_code)]
    fn mat4vec3mulop(m1: Mat4, vec: Vec3) -> Vec3 {
        Vec3 { 
            x: vec.x * m1.mat[0] + vec.y * m1.mat[1] + vec.z * m1.mat[2] + m1.mat[3], 
            y: vec.x * m1.mat[4] + vec.y * m1.mat[5] + vec.z * m1.mat[6] + m1.mat[7], 
            z: vec.x * m1.mat[8] + vec.y * m1.mat[9] + vec.z * m1.mat[10] + m1.mat[11], 
        }
    }
    fn rotated_up(q: Vec4) -> Vec3 {
        let magnitude_squared = q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w;
        if magnitude_squared <= f32::EPSILON {
            return Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        }

        Vec3 {
            x: 2.0 * (q.x * q.y + q.z * q.w) / magnitude_squared,
            y: (1.0 - 2.0 * (q.x * q.x + q.z * q.z) / magnitude_squared),
            z: 2.0 * (q.y * q.z - q.x * q.w) / magnitude_squared,
        }
    }
    fn in_range(v1: f32, v2: f32, p1: f32) -> bool{
        return  p1 >= v1 && p1 <= v2;
    }
    #[allow(dead_code)]
    pub fn exec(&mut self, logic_frametime: f32){
        if !self.is_static{
            self.hit = false;
            self.oldpos = self.pos;
            self.oldrot = self.rot;
            self.oldscale = self.scale;

            if self.gravity{
                self.acceleration.y += -GRAVITATIONAL_ACCELERATION;
            }

            let decay = self.air_friction.powf(logic_frametime);

            if !self.pin_pos{
                self.speed.x += self.acceleration.x*logic_frametime;
                self.speed.y += self.acceleration.y*logic_frametime;
                self.speed.z += self.acceleration.z*logic_frametime;
                self.acceleration.x = 0.0;
                self.acceleration.y = 0.0;
                self.acceleration.z = 0.0;

                self.speed.x *= decay;
                self.speed.y *= decay;
                self.speed.z *= decay;

                self.pos.x += self.speed.x*logic_frametime;
                self.pos.y += self.speed.y*logic_frametime;
                self.pos.z += self.speed.z*logic_frametime;
            }

            self.angular_velocity.x += self.angular_acceleration.x*logic_frametime;
            self.angular_velocity.y += self.angular_acceleration.y*logic_frametime;
            self.angular_velocity.z += self.angular_acceleration.z*logic_frametime;

            self.angular_acceleration.x = 0.0;
            self.angular_acceleration.y = 0.0;
            self.angular_acceleration.z = 0.0;

            self.angular_velocity.x *= decay;
            self.angular_velocity.y *= decay;
            self.angular_velocity.z *= decay;

            let (wx, wy, wz) = (-self.angular_velocity.x, -self.angular_velocity.y, -self.angular_velocity.z);
            let (qx, qy, qz, qw) = (self.rot.x, self.rot.y, self.rot.z, self.rot.w);

            let dqx =  wx*qw + wy*qz - wz*qy;
            let dqy = -wx*qz + wy*qw + wz*qx;
            let dqz =  wx*qy - wy*qx + wz*qw;
            let dqw = -(wx*qx + wy*qy + wz*qz);

            self.rot.x += 0.5*dqx*logic_frametime;
            self.rot.y += 0.5*dqy*logic_frametime;
            self.rot.z += 0.5*dqz*logic_frametime;
            self.rot.w += 0.5*dqw*logic_frametime;

            self.rot.normalize();

            let mut mmat = Mat4::new();
            mmat.trans(self.pos);
            let mut t: Mat4 = Mat4::new();
            if self.enable_rotation {
                t.quat_rot(self.rot);
                mmat *= t;
                t = Mat4::new();
            }
            t.scale(self.scale);
            mmat *= t;
            self.mat = mmat;
            self.collider.update(self.mat);
        }else{
            if self.pos.x != self.oldpos.x || self.pos.y != self.oldpos.y || self.pos.z != self.oldpos.z || self.rot.x != self.oldrot.x || self.rot.y != self.oldrot.y || self.rot.z != self.oldrot.z || self.scale.x != self.oldscale.x || self.scale.y != self.oldscale.y || self.scale.z != self.oldscale.z{
                let mut mmat = Mat4::new();
                mmat.trans(self.pos);
                let mut t: Mat4 = Mat4::new();
                if self.enable_rotation {
                    t.quat_rot(self.rot);
                    mmat *= t;
                    t = Mat4::new();
                }
                t.scale(self.scale);
                mmat *= t;
                self.mat = mmat;
                self.oldpos = self.pos;
                self.oldrot = self.rot;
                self.oldscale = self.scale;
                self.mat = mmat;
                self.collider.update(self.mat);
            }
        }
    }
    fn cross(v1: Vec2, v2: Vec2) -> f32 {
        v1.x * v2.y - v1.y * v2.x
    }
    #[allow(dead_code)]
    pub fn reset_states(&mut self){
        self.is_interacting = false;
    }
    #[allow(dead_code)]
    pub fn interact_with_other_object(&mut self, ph2: PhysicsObject){
        self.standing_on = false;
        if self.pin_pos {
            return;
        }
        if !self.collider.broad_phase(&ph2.collider) {
            return;
        }

        self.is_interacting = true;

        if !(self.solid && !self.is_static && ph2.solid) {
            return;
        }

        self.acceleration.y = 0.0;
        self.speed.y = -self.speed.y * self.elasticity;

        if let Some(m) = self.collider.narrow_phase(&ph2.collider) {
            let n = Vec3 { x: -m.axis.x, y: -m.axis.y, z: -m.axis.z };
            let self_bottom = self.collider.aabb_min.y;
            let obstacle_top = ph2.collider.aabb_max.y;
            let is_side_hit = n.y.abs() < std::f32::consts::FRAC_1_SQRT_2;
            if is_side_hit && obstacle_top - self_bottom <= self.step_height {
                self.pos.y += obstacle_top - self_bottom + 0.001;
            } else {
                self.pos.x -= m.axis.x * (m.penetration + 0.001);
                self.pos.y -= m.axis.y * (m.penetration + 0.001);
                self.pos.z -= m.axis.z * (m.penetration + 0.001);
            
                let v_along = self.speed.x * n.x + self.speed.y * n.y + self.speed.z * n.z;
                if v_along < 0.0 {
                    let j = (1.0 + self.elasticity) * v_along;
                    self.speed.x -= n.x * j;
                    self.speed.y -= n.y * j;
                    self.speed.z -= n.z * j;
                    //if self.compute_torque {
                    //    let center = Vec3 {
                    //        x: (self.collider.corners[0].x + self.collider.corners[4].x) * 0.5,
                    //        y: (self.collider.corners[0].y + self.collider.corners[4].y) * 0.5,
                    //        z: (self.collider.corners[0].z + self.collider.corners[4].z) * 0.5,
                    //    };
                    //
                    //    let depth = |c: &Vec3| -(c.x * n.x + c.y * n.y + c.z * n.z);
                    //    let deepest = self.collider.corners.iter().map(depth).fold(f32::MIN, f32::max);
                    //
                    //    const TIE_EPS: f32 = 0.001;
                    //    let mut contact = Vec3::new();
                    //    let mut tied = 0;
                    //    for c in self.collider.corners.iter() {
                    //        if deepest - depth(c) < TIE_EPS {
                    //            contact += *c;
                    //            tied += 1;
                    //        }
                    //    }
                    //    contact /= Vec3 { x: tied as f32, y: tied as f32, z: tied as f32 };
                    //
                    //    let r = Vec3 { x: contact.x - center.x, y: contact.y - center.y, z: contact.z - center.z };
                    //    let cross_rn = Vec3 {
                    //        x: r.y * n.z - r.z * n.y,
                    //        y: r.z * n.x - r.x * n.z,
                    //        z: r.x * n.y - r.y * n.x,
                    //    };
                    //
                    //    let he = Vec3 {
                    //        x: (self.collider.local_max.x - self.collider.local_min.x) * 0.5,
                    //        y: (self.collider.local_max.y - self.collider.local_min.y) * 0.5,
                    //        z: (self.collider.local_max.z - self.collider.local_min.z) * 0.5,
                    //    };
                    //    let i_x = (he.y * he.y + he.z * he.z) / 3.0;
                    //    let i_y = (he.x * he.x + he.z * he.z) / 3.0;
                    //    let i_z = (he.x * he.x + he.y * he.y) / 3.0;
                    //
                    //    self.angular_velocity.x -= j * cross_rn.x / i_x;
                    //    self.angular_velocity.y -= j * cross_rn.y / i_y;
                    //    self.angular_velocity.z -= j * cross_rn.z / i_z;
                    //}
                }
            
                let a_along = self.acceleration.x * n.x + self.acceleration.y * n.y + self.acceleration.z * n.z;
                if a_along < 0.0 {
                    self.acceleration.x -= n.x * a_along;
                    self.acceleration.y -= n.y * a_along;
                    self.acceleration.z -= n.z * a_along;
                }

                if n.y > std::f32::consts::FRAC_1_SQRT_2 {
                    self.standing_on = true;

                    if self.compute_torque{
                        let sn = self.rot.quat_direction().normalize();
                        let on = ph2.rot.quat_direction().normalize();
                        let torque = sn.cross(on);
                        //self.angular_acceleration.x += torque.x*100.0;
                        //self.angular_acceleration.z += torque.z*100.0;
                    }
                }
            }
        }
    }
}