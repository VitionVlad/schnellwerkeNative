use super::math::{mat4::Mat4, vec3::Vec3};

#[allow(dead_code)]
pub fn check_for_intersection(x1: f32, x2: f32, y1: f32, y2: f32) -> bool{
    return x1 <= y2 && y1 <= x2;
}

#[allow(dead_code)]
pub fn getpoints(v: Vec<f32>) -> Vec<Vec3>{
    let mut v1 = Vec3::newdefined(v[0], v[1], v[2]);
    let mut v2 = Vec3::newdefined(v[0], v[1], v[2]);
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
#[derive(Copy, Clone)]
pub struct PhysicsObject{
    pub acceleration: Vec3,
    pub speed: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub is_static: bool,
    pub is_interacting: bool,
    pub elasticity: f32,
    pub gravity: bool,
    pub air_friction: f32,
    pub pos: Vec3,
    pub rot: Vec3,
    pub scale: Vec3,
    pub mat: Mat4,
    pub solid: bool,
    pub mass: f32,
    pub retur: bool,
    pub enable_rotation: bool,
    oldpos: Vec3,
    oldrot: Vec3,
    oldscale: Vec3,
    pub savedp1: Vec3,
    pub savedp2: Vec3,
}

impl PhysicsObject{
    #[allow(dead_code)]
    pub fn new(v: Vec<Vec3>, is_static: bool) -> PhysicsObject{
        PhysicsObject{
            acceleration: Vec3::new(),
            speed: Vec3::new(),
            v1: v[0],
            v2: v[1],
            is_static: is_static,
            is_interacting: false,
            elasticity: 0.0f32,
            gravity: true,
            air_friction: 0.9f32,
            pos: Vec3::new(),
            rot: Vec3::new(),
            scale: Vec3::newdefined(1f32, 1f32, 1f32),
            mat: Mat4::new(),
            solid: true,
            mass: 0.01f32,
            retur: false,
            enable_rotation: true,
            oldpos: Vec3::new(),
            oldrot: Vec3::new(),
            oldscale: Vec3::new(),
            savedp1: v[0],
            savedp2: v[1],
        }
    }
    #[allow(dead_code)]
    fn mat4mat4mulop(m1: Mat4, m2: Mat4) -> Mat4 {
        let mut t: Mat4 = Mat4::new();
        t.mat[0] = m1.mat[0] * m2.mat[0] + m1.mat[1] * m2.mat[4] + m1.mat[2] * m2.mat[8] + m1.mat[3] * m2.mat[12];
        t.mat[1] = m1.mat[0] * m2.mat[1] + m1.mat[1] * m2.mat[5] + m1.mat[2] * m2.mat[9] + m1.mat[3] * m2.mat[13];
        t.mat[2] = m1.mat[0] * m2.mat[2] + m1.mat[1] * m2.mat[6] + m1.mat[2] * m2.mat[10] +m1.mat[3] * m2.mat[14];
        t.mat[3] = m1.mat[0] * m2.mat[3] + m1.mat[1] * m2.mat[7] + m1.mat[2] * m2.mat[11] +m1.mat[3] * m2.mat[15];

        t.mat[4] = m1.mat[4] * m2.mat[0] + m1.mat[5] * m2.mat[4] + m1.mat[6] * m2.mat[8] + m1.mat[7] * m2.mat[12];
        t.mat[5] = m1.mat[4] * m2.mat[1] + m1.mat[5] * m2.mat[5] + m1.mat[6] * m2.mat[9] + m1.mat[7] * m2.mat[13];
        t.mat[6] = m1.mat[4] * m2.mat[2] + m1.mat[5] * m2.mat[6] + m1.mat[6] * m2.mat[10] + m1.mat[7] * m2.mat[14];
        t.mat[7] = m1.mat[4] * m2.mat[3] + m1.mat[5] * m2.mat[7] + m1.mat[6] * m2.mat[11] + m1.mat[7] * m2.mat[15];

        t.mat[8] = m1.mat[8] * m2.mat[0] + m1.mat[9] * m2.mat[4] + m1.mat[10] * m2.mat[8] + m1.mat[11] * m2.mat[12];
        t.mat[9] = m1.mat[8] * m2.mat[1] + m1.mat[9] * m2.mat[5] + m1.mat[10] * m2.mat[9] + m1.mat[11] * m2.mat[13];
        t.mat[10] = m1.mat[8] * m2.mat[2] + m1.mat[9] * m2.mat[6] + m1.mat[10] * m2.mat[10] + m1.mat[11] * m2.mat[14];
        t.mat[11] = m1.mat[8] * m2.mat[3] + m1.mat[9] * m2.mat[7] + m1.mat[10] * m2.mat[11] + m1.mat[11] * m2.mat[15];
        return t;
    }
    #[allow(dead_code)]
    fn mat4vec3mulop(m1: Mat4, vec: Vec3) -> Vec3 {
        Vec3 { 
            x: vec.x * m1.mat[0] + vec.y * m1.mat[1] + vec.z * m1.mat[2] + m1.mat[3], 
            y: vec.x * m1.mat[4] + vec.y * m1.mat[5] + vec.z * m1.mat[6] + m1.mat[7], 
            z: vec.x * m1.mat[8] + vec.y * m1.mat[9] + vec.z * m1.mat[10] + m1.mat[11], 
        }
    }
    #[allow(dead_code)]
    fn getbgp(v: Vec<Vec3>) -> Vec3 {
        let mut f = Vec3::newdefined(v[0].x, v[0].y, v[0].z);
        for i in 0..v.len(){
            if v[i].x > f.x{
                f.x = v[i].x;
            }
            if v[i].y > f.y{
                f.y = v[i].y;
            }
            if v[i].z > f.z{
                f.z = v[i].z;
            }
        }
        return f;
    }
    #[allow(dead_code)]
    fn getbsp(v: Vec<Vec3>) -> Vec3 {
        let mut f = Vec3::newdefined(v[0].x, v[0].y, v[0].z);
        for i in 0..v.len(){
            if v[i].x < f.x{
                f.x = v[i].x;
            }
            if v[i].y < f.y{
                f.y = v[i].y;
            }
            if v[i].z < f.z{
                f.z = v[i].z;
            }
        }
        return f;
    }
    #[allow(dead_code)]
    pub fn exec(&mut self){
        if !self.is_static{
            if self.retur{
                self.retur = !self.retur;
                self.pos = self.oldpos;
                self.rot = self.oldrot;
                self.scale = self.oldscale;
            }
            self.oldpos = self.pos;
            self.oldrot = self.rot;
            self.oldscale = self.scale;
            self.speed.sum(self.acceleration);
            self.speed.x *= self.air_friction;
            self.speed.y *= self.air_friction;
            self.speed.z *= self.air_friction;
            self.acceleration.x = 0.0;
            self.acceleration.y = 0.0;
            self.acceleration.z = 0.0;
            self.pos.sum(self.speed);
            if self.gravity{
                self.acceleration.y = -self.mass;
            }
            let mut mmat = Mat4::new();
            mmat.trans(self.pos);
            let mut t: Mat4 = Mat4::new();
            if self.enable_rotation {
                t.yrot(self.rot.y);
                mmat = Self::mat4mat4mulop(mmat, t);
                t = Mat4::new();
                t.xrot(self.rot.x);
                mmat = Self::mat4mat4mulop(mmat, t);
                t = Mat4::new();
                t.zrot(self.rot.z);
                mmat = Self::mat4mat4mulop(mmat, t);
                t = Mat4::new();
            }
            t.scale(self.scale);
            mmat = Self::mat4mat4mulop(mmat, t);
            self.mat = mmat;
            let c1 = [
                Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v1.x, self.v1.y, self.v1.z)),
                Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v1.x, self.v2.y, self.v1.z)),
                Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v2.x, self.v2.y, self.v1.z)),
                Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v2.x, self.v1.y, self.v1.z)),
                Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v2.x, self.v2.y, self.v2.z)),
                Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v1.x, self.v2.y, self.v2.z)),
                Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v1.x, self.v1.y, self.v2.z)),
                Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v2.x, self.v1.y, self.v2.z)),
            ];
            self.savedp1 = Self::getbgp(c1.to_vec());
            self.savedp2 = Self::getbsp(c1.to_vec());
        }else{
            if self.pos.x != self.oldpos.x || self.pos.y != self.oldpos.y || self.pos.z != self.oldpos.z || self.rot.x != self.oldrot.x || self.rot.y != self.oldrot.y || self.rot.z != self.oldrot.z || self.scale.x != self.oldscale.x || self.scale.y != self.oldscale.y || self.scale.z != self.oldscale.z{
                let mut mmat = Mat4::new();
                mmat.trans(self.pos);
                let mut t: Mat4 = Mat4::new();
                if self.enable_rotation {
                    t.yrot(self.rot.y);
                    mmat = Self::mat4mat4mulop(mmat, t);
                    t = Mat4::new();
                    t.xrot(self.rot.x);
                    mmat = Self::mat4mat4mulop(mmat, t);
                    t = Mat4::new();
                    t.zrot(self.rot.z);
                    mmat = Self::mat4mat4mulop(mmat, t);
                    t = Mat4::new();
                }
                t.scale(self.scale);
                mmat = Self::mat4mat4mulop(mmat, t);
                self.mat = mmat;
                self.oldpos = self.pos;
                self.oldrot = self.rot;
                self.oldscale = self.scale;
                let c1 = [
                    Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v1.x, self.v1.y, self.v1.z)),
                    Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v1.x, self.v2.y, self.v1.z)),
                    Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v2.x, self.v2.y, self.v1.z)),
                    Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v2.x, self.v1.y, self.v1.z)),
                    Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v2.x, self.v2.y, self.v2.z)),
                    Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v1.x, self.v2.y, self.v2.z)),
                    Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v1.x, self.v1.y, self.v2.z)),
                    Self::mat4vec3mulop(self.mat, Vec3::newdefined(self.v2.x, self.v1.y, self.v2.z)),
                ];
                self.savedp1 = Self::getbgp(c1.to_vec());
                self.savedp2 = Self::getbsp(c1.to_vec());
            }
        }
    }
    #[allow(dead_code)]
    pub fn reset_states(&mut self){
        self.is_interacting = false;
    }
    #[allow(dead_code)]
    pub fn interact_with_other_object(&mut self, ph2: PhysicsObject){
        if check_for_intersection(ph2.savedp2.y, ph2.savedp1.y, self.savedp2.y, self.savedp1.y) && 
            check_for_intersection(ph2.savedp2.x, ph2.savedp1.x, self.savedp2.x, self.savedp1.x) && 
            check_for_intersection(ph2.savedp2.z, ph2.savedp1.z, self.savedp2.z, self.savedp1.z) {
            self.is_interacting = true;
            if self.solid && !self.is_static && ph2.solid{
                self.acceleration.y = 0f32;
                self.speed.y = -self.speed.y * self.elasticity;
                if self.savedp2.y + 1f32 <= ph2.savedp1.y{
                    self.acceleration.x = 0f32;
                    self.speed.x = -self.speed.x * self.elasticity;
                    self.acceleration.z = 0f32;
                    self.speed.z = -self.speed.z * self.elasticity;
                    self.retur = true;
                }else{
                    self.pos.y += ph2.savedp1.y - self.savedp2.y - 0.001f32;
                }
            }
        }
    }
}