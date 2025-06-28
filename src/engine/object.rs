#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::math::{vec3::Vec3, vec4::Vec4};

use super::{engine::Engine, image::Image, material::Material, math::mat4::Mat4, model::Model, physics::PhysicsObject, render::render::{Mesh, MeshUsage}};

#[derive(Copy, Clone)]
pub struct Object{
    pub mesh: Mesh,
    pub physic_object: PhysicsObject,
    usage: MeshUsage,
    eng_ph_id: usize,
    blank: bool,
}

impl Object {
    pub fn new(engine: &mut Engine, model: Model, material: Material, image: Image, usage: MeshUsage, is_static: bool) -> Object{
        let ph = PhysicsObject::new(model.points.to_vec(), is_static);
        let id = engine.obj_ph.len();
        if usage == MeshUsage::DefferedPass || usage == MeshUsage::ShadowAndDefferedPass{
            engine.obj_ph.push(ph);
        }
        Object { 
            mesh: Mesh::new(engine.render, model.vertexbuf, material.material_shaders, image.textures, usage),
            physic_object: ph,
            usage: usage,
            eng_ph_id: id,
            blank: false,
        }
    }
    pub fn new_blank() -> Object{
        Object { 
            mesh: Mesh { meshid: 0, ubo: [0.0; 20], draw: true, draw_shadow: true, keep_shadow: false, render_all_cameras: true, exclude_selected_camera: false, camera_number: 0 },
            physic_object: PhysicsObject::new(vec![Vec3::new(), Vec3::new()], true),
            usage: MeshUsage::ShadowAndDefferedPass,
            eng_ph_id: 0,
            blank: true,
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
        if !self.blank{
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
            if self.usage == MeshUsage::DefferedPass || self.usage == MeshUsage::ShadowAndDefferedPass{
                let th = self.physic_object.clone();
                self.physic_object = eng.obj_ph[self.eng_ph_id];
                eng.obj_ph[self.eng_ph_id] = th;

                let mut mt = eng.cameras[eng.primary_camera as usize].get_projection(eng.render.resolution_x as f32/eng.render.resolution_y as f32);
                mt.transpose();

                let c1 = [
                    mt.vec4mul(ubm.vec4mul(Vec4::newdefined(self.physic_object.v1.x, self.physic_object.v1.y, self.physic_object.v1.z, 1.0))),
                    mt.vec4mul(ubm.vec4mul(Vec4::newdefined(self.physic_object.v1.x, self.physic_object.v2.y, self.physic_object.v1.z, 1.0))),
                    mt.vec4mul(ubm.vec4mul(Vec4::newdefined(self.physic_object.v2.x, self.physic_object.v2.y, self.physic_object.v1.z, 1.0))),
                    mt.vec4mul(ubm.vec4mul(Vec4::newdefined(self.physic_object.v2.x, self.physic_object.v1.y, self.physic_object.v1.z, 1.0))),
                    mt.vec4mul(ubm.vec4mul(Vec4::newdefined(self.physic_object.v2.x, self.physic_object.v2.y, self.physic_object.v2.z, 1.0))),
                    mt.vec4mul(ubm.vec4mul(Vec4::newdefined(self.physic_object.v1.x, self.physic_object.v2.y, self.physic_object.v2.z, 1.0))),
                    mt.vec4mul(ubm.vec4mul(Vec4::newdefined(self.physic_object.v1.x, self.physic_object.v1.y, self.physic_object.v2.z, 1.0))),
                    mt.vec4mul(ubm.vec4mul(Vec4::newdefined(self.physic_object.v2.x, self.physic_object.v1.y, self.physic_object.v2.z, 1.0))),
                ];

                if c1[0].z < 0.0 && c1[1].z < 0.0 && c1[2].z < 0.0 && c1[3].z < 0.0 && c1[4].z < 0.0 && c1[5].z < 0.0 && c1[6].z < 0.0 && c1[7].z < 0.0{
                    self.mesh.draw = false;
                    self.mesh.keep_shadow = true;
                }else{
                    self.mesh.draw = true;
                    self.mesh.keep_shadow = true;
                }
            }
            ubm.transpose();
            for i in 0..16{
                self.mesh.ubo[i] = ubm.mat[i];
            }
            self.mesh.exec();
        }
    }
}