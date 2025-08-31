use super::{math::{mat4::Mat4, vec3::Vec3}, physics::PhysicsObject};

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Camera{
    pub physic_object: PhysicsObject,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
    pub is_orthographic: bool,
    pub rotation_colision_calc: bool,
}

impl Camera{
    #[allow(dead_code)]
    pub fn get_projection(&mut self, aspect: f32) -> Mat4{
        self.physic_object.enable_rotation = self.rotation_colision_calc;
        let mut ubm = Mat4::new();
        if !self.is_orthographic{
            ubm.perspective(self.fov, self.zfar, self.znear, aspect);
        }else{
            ubm.orthographic(self.fov, -self.fov, self.fov, -self.fov, self.znear, self.zfar);
        }
        let mut t: Mat4 = Mat4::new();
        t.xrot(self.physic_object.rot.x);
        ubm.mul(&t);
        t = Mat4::new();
        t.yrot(self.physic_object.rot.y);
        ubm.mul(&t);
        t = Mat4::new();
        t.zrot(self.physic_object.rot.z);
        ubm.mul(&t);
        t = Mat4::new();
        t.trans(Vec3::newdefined(-self.physic_object.pos.x, -self.physic_object.pos.y, -self.physic_object.pos.z));
        ubm.mul(&t);
        ubm.transpose();
        return ubm;
    }
}