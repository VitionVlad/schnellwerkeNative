#![allow(dead_code)]
#![allow(unused_variables)]
use std::ops::Mul;

use crate::engine::math::vec2::Vec2;

use crate::engine::math::{mat4::Mat4, vec3::Vec3};

use crate::engine::physics::collision::*;

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
    pub rot: Vec3,
    pub angular_velocity: Vec3,
    pub angular_acceleration: Vec3,
    pub scale: Vec3,
    pub mat: Mat4,
    pub solid: bool,
    pub mass: f32,
    pub enable_rotation: bool,
    pub step_height: f32,
    oldpos: Vec3,
    oldrot: Vec3,
    oldscale: Vec3,
    intersectionp: Vec2,
    pub hit: bool,
    pub pin_pos: bool,
    ethalonrot: Vec3,
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
            rot: Vec3::new(),
            angular_velocity: Vec3::new(),
            angular_acceleration: Vec3::new(),
            scale: Vec3{ x: 1f32, y: 1f32, z: 1f32},
            mat: Mat4::new(),
            solid: true,
            mass: 25.0f32,
            enable_rotation: true,
            step_height: 0.5f32,
            oldpos: Vec3::new(),
            oldrot: Vec3::new(),
            oldscale: Vec3::new(),
            intersectionp: Vec2::new(),
            hit: false,
            pin_pos: false,
            ethalonrot: Vec3::new(),
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
            rot: Vec3::new(),
            angular_velocity: Vec3::new(),
            angular_acceleration: Vec3::new(),
            scale: Vec3{ x: 1f32, y: 1f32, z: 1f32},
            mat: Mat4::new(),
            solid: true,
            mass: 25.0f32,
            enable_rotation: true,
            step_height: 0.5f32,
            oldpos: Vec3::new(),
            oldrot: Vec3::new(),
            oldscale: Vec3::new(),
            intersectionp: Vec2::new(),
            hit: false,
            pin_pos: false,
            ethalonrot: Vec3::new(),
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

            let mut lowest_corner = self.collider.corners[0];
            let mut center = Vec3::new();

            for i in 0..8{
                center += self.collider.corners[i];
                if lowest_corner.y > self.collider.corners[i].y{
                    lowest_corner = self.collider.corners[i];
                }
            }

            center /= Vec3{ x: 8.0, y: 8.0, z: 8.0};

            if self.gravity{
                self.acceleration.y += -self.mass;
                if self.compute_torque{
                    let xsin = (self.rot.x + self.ethalonrot.x.asin()).sin();
                    let zsin = (self.rot.z + self.ethalonrot.z.asin()).sin();
                    if !Self::in_range(0.99, 1.0, xsin.abs())
                    && !Self::in_range(0.0, 0.001, xsin.abs()){
                        self.angular_acceleration.x += (center.z-lowest_corner.z)*self.mass;   
                    }
                    if !Self::in_range(0.99, 1.0, zsin.abs())
                    && !Self::in_range(0.0, 0.001, zsin.abs()){
                        self.angular_acceleration.z += (center.x-lowest_corner.x)*self.mass;    
                    }
                }
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

            self.rot.x += self.angular_velocity.x*logic_frametime;
            self.rot.y += self.angular_velocity.y*logic_frametime;
            self.rot.z += self.angular_velocity.z*logic_frametime;

            let mut mmat = Mat4::new();
            mmat.trans(self.pos);
            let mut t: Mat4 = Mat4::new();
            if self.enable_rotation {
                t.xrot(self.rot.x);
                mmat = mmat.mul(t);
                t = Mat4::new();
                t.yrot(self.rot.y);
                mmat =  mmat.mul(t);
                t = Mat4::new();
                t.zrot(self.rot.z);
                mmat =  mmat.mul(t);
                t = Mat4::new();
            }
            t.scale(self.scale);
            mmat =  mmat.mul(t);
            self.mat = mmat;
            self.collider.update(self.mat);
        }else{
            if self.pos.x != self.oldpos.x || self.pos.y != self.oldpos.y || self.pos.z != self.oldpos.z || self.rot.x != self.oldrot.x || self.rot.y != self.oldrot.y || self.rot.z != self.oldrot.z || self.scale.x != self.oldscale.x || self.scale.y != self.oldscale.y || self.scale.z != self.oldscale.z{
                let mut mmat = Mat4::new();
                mmat.trans(self.pos);
                let mut t: Mat4 = Mat4::new();
                if self.enable_rotation {
                    t.xrot(self.rot.x);
                    mmat =  mmat.mul(t);
                    t = Mat4::new();
                    t.yrot(self.rot.y);
                    mmat =  mmat.mul(t);
                    t = Mat4::new();
                    t.zrot(self.rot.z);
                    mmat =  mmat.mul(t);
                    t = Mat4::new();
                }
                t.scale(self.scale);
                mmat =  mmat.mul(t);
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
    fn calclninter(&mut self, l1p1: Vec2, l1p2: Vec2, l2p1: Vec2, l2p2: Vec2){
        self.hit = false;

        let d1 = Vec2{ x: l1p2.x - l1p1.x, y: l1p2.y - l1p1.y}; 
        let d2 = Vec2{ x: l2p2.x - l2p1.x, y: l2p2.y - l2p1.y}; 
        let d3 = Vec2{ x: l2p1.x - l1p1.x, y: l2p1.y - l1p1.y}; 

        let denom = Self::cross(d1, d2);

        const EPS: f32 = 1e-9;
        if denom.abs() < EPS {
            return;
        }

        let t = Self::cross(d3, d2) / denom;
        let s = Self::cross(d3, d1) / denom;

        if t >= 0.0 && t <= 1.0 && s >= 0.0 && s <= 1.0 {
            self.intersectionp = Vec2 {
                x: l1p1.x + t * d1.x,
                y: l1p1.y + t * d1.y,
            };
            self.hit = true;
        }
    }
    #[allow(dead_code)]
    pub fn interact_with_other_object(&mut self, ph2: PhysicsObject){
        self.standing_on = false;
        self.ethalonrot = Vec3 { 
            x: 0.0, 
            y: 0.0, 
            z: 0.0 
        };
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

        if self.collider.aabb_min.y + self.step_height <= ph2.collider.aabb_max.y {
            if let Some(m) = self.collider.narrow_phase(&ph2.collider) {
                match m.axis {
                    Axis::X => {
                        self.pos.x += m.sign * (m.penetration + 0.001);
                        if self.speed.x * m.sign < 0.0 {
                            self.speed.x = -self.speed.x * self.elasticity;
                        }
                        self.acceleration.x = 0.0;
                    }
                    Axis::Y => {
                        self.pos.y += m.sign * (m.penetration + 0.001);
                        if self.speed.y * m.sign < 0.0 {
                            self.speed.y = -self.speed.y * self.elasticity;
                        }
                        self.acceleration.y = 0.0;
                        self.ethalonrot = Vec3 { 
                            x: ph2.rot.x.sin(), 
                            y: 0.0, 
                            z: ph2.rot.z.sin() 
                        };
                        self.standing_on = true;
                    }
                    Axis::Z => {
                        self.pos.z += m.sign * (m.penetration + 0.001);
                        if self.speed.z * m.sign < 0.0 {
                            self.speed.z = -self.speed.z * self.elasticity;
                        }
                        self.acceleration.z = 0.0;
                    }
                }
            }
        } else {
            self.pos.y += ph2.collider.aabb_max.y - self.collider.aabb_min.y - 0.001;
            self.ethalonrot = Vec3 { 
                x: ph2.rot.x.sin(), 
                y: 0.0, 
                z: ph2.rot.z.sin() 
            };
            self.standing_on = true;
        }
    }
}