#![allow(dead_code)]
#![allow(unused_variables)]

use super::{engine::Engine, image::Image, loader::modelasset::ModelAsset, material::Material, model::Model, object::Object};

pub struct Scene{
    pub objects: Vec<Object>,
}

impl Scene{
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
            objects: fobj 
        }
    }
    pub fn exec(&mut self, eng: &mut Engine){
        for i in 0..self.objects.len(){
            self.objects[i].exec(eng);
        }
    }
}