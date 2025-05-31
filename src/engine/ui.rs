#![allow(dead_code)]
#![allow(unused_variables)]

use super::{engine::Engine, image::Image, material::Material, math::{vec2::Vec2, vec3::Vec3}, model::Model, object::Object, plane::PLANE};

pub struct UIplane{
    pub object: Object
}

impl UIplane {
    pub fn new(eng: &mut Engine, mat: Material, image: Image) -> UIplane{
        let model = Model::new(&eng, PLANE.to_vec());
        UIplane { 
            object: Object::new(eng, model, mat, image, super::render::render::MeshUsage::LightingPass, true)
        }
    }
    pub fn new_from_file(eng: &mut Engine, mat: Material, paths: Vec<String>) -> UIplane{
        let image = Image::new_from_files(eng, paths);
        let model = Model::new(&eng, PLANE.to_vec());
        UIplane { 
            object: Object::new(eng, model, mat, image, super::render::render::MeshUsage::LightingPass, true)
        }
    }
    pub fn exec(&mut self, eng: &mut Engine){
        self.object.exec(eng);
    }
}

pub struct UItext{
    plane: Model,
    pub font: Image,
    pub symbols: Vec<u8>,
    pub planes: Vec<Object>,
    pub symbol_number: u32,
    pub material: Material,
    pub size: Vec2,
    pub pos: Vec3,
}

impl UItext {
    pub fn new(eng: &mut Engine, mat: Material, image: Image, symbols: &str) -> UItext{
        UItext{
            plane: Model::new(&eng, PLANE.to_vec()),
            font: image,
            symbols: symbols.as_bytes().to_vec(),
            planes: vec![],
            symbol_number: symbols.len() as u32,
            material: mat,
            size: Vec2::newdefined(40.0, 80.0),
            pos: Vec3::new(),
        }
    }
    pub fn exec(&mut self, eng: &mut Engine, text: &str){
        let bt = text.as_bytes();
        if self.planes.len() <= bt.len() {
            for i in  self.planes.len()..bt.len(){
                self.planes.push(Object::new(eng, self.plane, self.material, self.font, super::render::render::MeshUsage::LightingPass, true));
            }
        }
        for i in 0..bt.len(){
            for j in 0..self.symbols.len(){
                if bt[i] == self.symbols[j] {
                    self.planes[i].mesh.ubo[16] = self.symbol_number as f32;
                    self.planes[i].mesh.ubo[17] = j as f32;
                    self.planes[i].physic_object.scale.x = self.size.x;
                    self.planes[i].physic_object.scale.y = self.size.y;
                    self.planes[i].physic_object.scale.z = 1.0;
                    self.planes[i].physic_object.pos.x = self.pos.x + (i as f32)*self.size.x*2.0;
                    self.planes[i].physic_object.pos.y = self.pos.y;
                    self.planes[i].physic_object.pos.z = self.pos.z;
                    self.planes[i].exec(eng);
                }
            }
        }
    }
}