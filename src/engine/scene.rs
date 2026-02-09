#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::math::vec3::Vec3;

use super::{engine::Engine, image::Image, loader::modelasset::ModelAsset, material::Material, model::Model, object::Object};

pub struct Scene{
    pub objects: Vec<Object>,
    pub images: Vec<Image>,
    pub models: Vec<Model>,
    pub use_global_values: bool,
    pub pos: Vec3,
    pub scale: Vec3,
    pub rot: Vec3,
    pub render_all_cameras: bool,
    pub exclude_selected_camera: bool,
    pub camera_number: i8,
}

impl Scene{
    pub fn new_blank() -> Scene{
        Scene { 
            objects: vec![],
            images: vec![], 
            models: vec![], 
            use_global_values: false, 
            pos: Vec3::new(), 
            scale: Vec3::new(), 
            rot: Vec3::new(), 
            render_all_cameras: true, 
            exclude_selected_camera: false, 
            camera_number: 0 
        }
    }
    pub fn load_from_obj(eng: &mut Engine, path: &str, material: Material) -> Scene{
        let obj = ModelAsset::load_obj(path);
        let mut mdst: Vec<Model> = vec![];
        let mut mdtx: Vec<Image> = vec![];
        for i in 0..obj.mtl.matinfo.len(){
            mdtx.push(Image::new_from_files(&eng, obj.mtl.matinfo[i].clone()));
        }
        for i in 0..obj.vertices.len(){
            mdst.push(Model::new(&eng, obj.vertices[i].clone()));
        }
        let mut fobj: Vec<Object> = vec![];
        for i in 0..mdst.len(){
            for j in 0..mdtx.len(){
                if obj.mtl.matnam[j] == obj.matnam[i]{
                    fobj.push(Object::new(eng, mdst[i], material, mdtx[j], super::render::render::MeshUsage::ShadowAndDefferedPass, true));
                    break;
                }
            }
        }
        Scene { 
            objects: fobj,
            images: mdtx,
            models: mdst,
            use_global_values: true,
            pos: Vec3::new(),
            rot: Vec3::new(),
            scale: Vec3{ x: 1.0f32, y: 1.0f32, z: 1.0f32},
            render_all_cameras: true,
            exclude_selected_camera: false,
            camera_number: 0,
        }
    }
    pub fn exec(&mut self, eng: &mut Engine){
        for i in 0..self.objects.len(){
            if self.use_global_values{
                self.objects[i].physic_object.pos += self.pos;
                self.objects[i].physic_object.rot += self.rot;
                self.objects[i].physic_object.scale += self.scale;
                self.objects[i].mesh.render_all_cameras = self.render_all_cameras;
                self.objects[i].mesh.exclude_selected_camera = self.exclude_selected_camera;
                self.objects[i].mesh.camera_number = self.camera_number;
            }
            self.objects[i].exec(eng);
        }
    }
}