#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::math::{vec3::Vec3, vec4::Vec4};

use super::{engine::Engine, image::Image, material::Material, math::mat4::Mat4, model::Model, physics::PhysicsObject, render::render::{Mesh, MeshUsage}};

#[derive(Copy, Clone)]
pub struct Object{
    pub mesh: Mesh,
    pub physic_object: PhysicsObject,
    pub is_looking_at: bool,
    pub draw: bool,
    pub draw_shadow: bool,
    pub draw_distance: f32,
    pub view_reaction_distance: f32,
    pub render_in_behind: bool,
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
            is_looking_at: false,
            draw: true,
            draw_shadow: true,
            draw_distance: 30f32,
            view_reaction_distance: 2f32,
            render_in_behind: true,
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
            is_looking_at: false,
            draw: true,
            draw_shadow: true,
            draw_distance: 30f32,
            view_reaction_distance: 2f32,
            render_in_behind: true,
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
    fn in_range(v1: f32, v2: f32, p1: f32) -> bool{
        return  p1>= v1 && p1 <= v2;
    }
    #[allow(dead_code)]
    fn getbgp(v: Vec<Vec4>) -> Vec3 {
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
    fn getbsp(v: Vec<Vec4>) -> Vec3 {
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
                if self.physic_object.is_static{
                    eng.obj_ph[self.eng_ph_id] = self.physic_object;
                }else{
                    let th = self.physic_object.clone();
                    self.physic_object = eng.obj_ph[self.eng_ph_id];
                    eng.obj_ph[self.eng_ph_id] = th;
                }

                let mut mt = eng.cameras[eng.primary_camera as usize].get_projection(eng.render.resolution_x as f32/eng.render.resolution_y as f32);
                mt.transpose();

                let mut c1 = [
                    ubm.vec4mul(Vec4::newdefined(self.physic_object.v1.x, self.physic_object.v1.y, self.physic_object.v1.z, 1.0)),
                    ubm.vec4mul(Vec4::newdefined(self.physic_object.v1.x, self.physic_object.v2.y, self.physic_object.v1.z, 1.0)),
                    ubm.vec4mul(Vec4::newdefined(self.physic_object.v2.x, self.physic_object.v2.y, self.physic_object.v1.z, 1.0)),
                    ubm.vec4mul(Vec4::newdefined(self.physic_object.v2.x, self.physic_object.v1.y, self.physic_object.v1.z, 1.0)),
                    ubm.vec4mul(Vec4::newdefined(self.physic_object.v2.x, self.physic_object.v2.y, self.physic_object.v2.z, 1.0)),
                    ubm.vec4mul(Vec4::newdefined(self.physic_object.v1.x, self.physic_object.v2.y, self.physic_object.v2.z, 1.0)),
                    ubm.vec4mul(Vec4::newdefined(self.physic_object.v1.x, self.physic_object.v1.y, self.physic_object.v2.z, 1.0)),
                    ubm.vec4mul(Vec4::newdefined(self.physic_object.v2.x, self.physic_object.v1.y, self.physic_object.v2.z, 1.0)),
                ];

                let bg = Self::getbgp(c1.to_vec());
                let sg = Self::getbsp(c1.to_vec());

                c1 = [
                    mt.vec4mul(c1[0]),
                    mt.vec4mul(c1[1]),
                    mt.vec4mul(c1[2]),
                    mt.vec4mul(c1[3]),
                    mt.vec4mul(c1[4]),
                    mt.vec4mul(c1[5]),
                    mt.vec4mul(c1[6]),
                    mt.vec4mul(c1[7]),
                ];

                let lbg = Self::getbgp(c1.to_vec());
                let lsg = Self::getbsp(c1.to_vec());

                let mut behind = false;
                let cmp = eng.cameras[eng.primary_camera as usize].physic_object.pos;
                let dst1 = f32::sqrt(f32::powi(bg.x-cmp.x, 2)+f32::powi(bg.y-cmp.y, 2)+f32::powi(bg.z-cmp.z, 2));
                let dst2 = f32::sqrt(f32::powi(sg.x-cmp.x, 2)+f32::powi(sg.y-cmp.y, 2)+f32::powi(sg.z-cmp.z, 2));
                let fdst = f32::min(dst1, dst2);
                if (c1[0].z < 0.0 && c1[1].z < 0.0 && c1[2].z < 0.0 && c1[3].z < 0.0 && c1[4].z < 0.0 && c1[5].z < 0.0 && c1[6].z < 0.0 && c1[7].z < 0.0 && self.render_in_behind) || fdst >= self.draw_distance{
                    self.mesh.draw = false;
                    self.mesh.keep_shadow = self.draw_shadow;
                    self.mesh.draw_shadow = self.draw_shadow;
                    behind = true;
                }else{
                    self.mesh.draw = self.draw;
                    self.mesh.keep_shadow = self.draw_shadow;
                    self.mesh.draw_shadow = self.draw_shadow;
                }

                self.is_looking_at = false;

                if Self::in_range(lsg.x, lbg.x, 0.0) && Self::in_range(lsg.y, lbg.y, 0.0) && !behind && fdst <= self.view_reaction_distance{
                    self.is_looking_at = true;
                }
            }else if self.usage == MeshUsage::LightingPass{
                self.mesh.draw = self.draw;
                self.mesh.keep_shadow = false;
                self.mesh.draw_shadow = true;
            }else{
                self.mesh.draw = self.draw;
                self.mesh.keep_shadow = self.draw;
                self.mesh.draw_shadow = self.draw;
            }
            ubm.transpose();
            for i in 0..16{
                self.mesh.ubo[i] = ubm.mat[i];
            }
            self.mesh.exec();
        }
    }
}