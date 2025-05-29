#![allow(dead_code)]
#![allow(unused_variables)]

use super::{engine::Engine, image::Image, material::Material, math::mat4::Mat4, model::Model, physics::PhysicsObject, render::render::{Mesh, MeshUsage}};

#[derive(Copy, Clone)]
pub struct Object{
    pub mesh: Mesh,
    pub physic_object: PhysicsObject,
    usage: MeshUsage,
    eng_ph_id: usize,
}

impl Object {
    pub fn new(engine: &mut Engine, model: Model, material: Material, image: Image, usage: MeshUsage, is_static: bool) -> Object{
        let ph = PhysicsObject::new(model.points.to_vec(), is_static);
        let id = engine.obj_ph.len();
        engine.obj_ph.push(ph);
        engine.obj_us.push(usage);
        Object { 
            mesh: Mesh::new(engine.render, model.vertexbuf, material.material_shaders, image.textures, usage),
            physic_object: ph,
            usage: usage,
            eng_ph_id: id,
        }
    }
    pub fn execph(&mut self, eng: &mut Engine){
        if self.usage == MeshUsage::DefferedPass || self.usage == MeshUsage::ShadowAndDefferedPass {
            self.physic_object.reset_states();
            self.physic_object.exec();
            for i in 0..u32::min(eng.used_camera_count, 10){
                eng.cameras[i as usize].physic_object.interact_with_other_object(self.physic_object);
            }
        }
    }
    pub fn exec(&mut self, eng: &mut Engine){
        let th = self.physic_object.clone();
        self.physic_object = eng.obj_ph[self.eng_ph_id];
        eng.obj_ph[self.eng_ph_id] = th;
        let mut ubm = Mat4::new();
        ubm.trans(self.physic_object.pos);
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
        t.scale(self.physic_object.scale);
        ubm.mul(&t);
        ubm.transpose();
        for i in 0.. self.mesh.ubo.len(){
            self.mesh.ubo[i] = ubm.mat[i];
        }
        self.mesh.exec();
    }
}