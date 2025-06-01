#![allow(dead_code)]
#![allow(unused_variables)]

use super::{engine::Engine, image::Image, material::Material, math::{vec2::Vec2, vec3::Vec3}, model::Model, object::Object, plane::PLANEUI};

pub struct Clickzone{
    pos1: Vec2,
    pos2: Vec2,
}

impl Clickzone{
    pub fn check(&self, mouse: Vec2) -> bool{
        if mouse.x >= self.pos1.x && mouse.x <= self.pos2.x && mouse.y >= self.pos1.y && mouse.y <= self.pos2.y{
            return true;
        }
        return false;
    }
}

pub struct UIplane{
    pub object: Object,
    pub clickzone: Clickzone,
    pub signal: bool,
    pub allow_when_mouse_locked: bool,
}

impl UIplane {
    pub fn new(eng: &mut Engine, mat: Material, image: Image) -> UIplane{
        let model = Model::new(&eng, PLANEUI.to_vec());
        UIplane { 
            object: Object::new(eng, model, mat, image, super::render::render::MeshUsage::LightingPass, true),
            clickzone: Clickzone { pos1: Vec2::newdefined(0.0, 0.0), pos2: Vec2::newdefined(0.0, 0.0) },
            signal: false,
            allow_when_mouse_locked: false,
        }
    }
    pub fn new_from_file(eng: &mut Engine, mat: Material, paths: Vec<String>) -> UIplane{
        let image = Image::new_from_files(eng, paths);
        let model = Model::new(&eng, PLANEUI.to_vec());
        UIplane { 
            object: Object::new(eng, model, mat, image, super::render::render::MeshUsage::LightingPass, true),
            clickzone: Clickzone { pos1: Vec2::newdefined(0.0, 0.0), pos2: Vec2::newdefined(0.0, 0.0) },
            signal: false,
            allow_when_mouse_locked: false,
        }
    }
    pub fn exec(&mut self, eng: &mut Engine) -> bool{
        self.clickzone.pos1.x = self.object.physic_object.pos.x;
        self.clickzone.pos1.y = self.object.physic_object.pos.y;
        self.clickzone.pos2.x = self.object.physic_object.pos.x + self.object.physic_object.scale.x;
        self.clickzone.pos2.y = self.object.physic_object.pos.y + self.object.physic_object.scale.y;
        let btst = self.clickzone.check(Vec2::newdefined(eng.control.xpos as f32, eng.control.ypos as f32));
        if self.signal && btst && (self.allow_when_mouse_locked || (!self.allow_when_mouse_locked && !eng.control.mouse_lock)){
            self.object.mesh.ubo[18] = 1.0;
        }else{
            self.object.mesh.ubo[18] = 0.0;
        }
        self.object.exec(eng);
        return btst;
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
    pub clickzone: Clickzone,
    pub signal: bool,
    pub allow_when_mouse_locked: bool,
}

impl UItext {
    pub fn new(eng: &mut Engine, mat: Material, image: Image, symbols: &str) -> UItext{
        UItext{
            plane: Model::new(&eng, PLANEUI.to_vec()),
            font: image,
            symbols: symbols.as_bytes().to_vec(),
            planes: vec![],
            symbol_number: symbols.len() as u32,
            material: mat,
            size: Vec2::newdefined(20.0, 40.0),
            pos: Vec3::new(),
            clickzone: Clickzone { pos1: Vec2::newdefined(0.0, 0.0), pos2: Vec2::newdefined(0.0, 0.0) },
            signal: false,
            allow_when_mouse_locked: false,
        }
    }
    pub fn new_from_file(eng: &mut Engine, mat: Material, image: &str, symbols: &str) -> UItext{
        let img = Image::new_from_files(eng, vec![image.to_string()]);
        UItext{
            plane: Model::new(&eng, PLANEUI.to_vec()),
            font: img,
            symbols: symbols.as_bytes().to_vec(),
            planes: vec![],
            symbol_number: symbols.len() as u32,
            material: mat,
            size: Vec2::newdefined(20.0, 40.0),
            pos: Vec3::new(),
            clickzone: Clickzone { pos1: Vec2::newdefined(0.0, 0.0), pos2: Vec2::newdefined(0.0, 0.0) },
            signal: false,
            allow_when_mouse_locked: false,
        }
    }
    pub fn exec(&mut self, eng: &mut Engine, text: &str) -> bool{
        let bt = text.as_bytes();
        self.clickzone.pos1.x = self.pos.x;
        self.clickzone.pos1.y = self.pos.y;
        self.clickzone.pos2.x = self.pos.x + self.size.x*(bt.len() as f32);
        self.clickzone.pos2.y = self.pos.y + self.size.y;
        let btst = self.clickzone.check(Vec2::newdefined(eng.control.xpos as f32, eng.control.ypos as f32));
        if self.planes.len() < bt.len() {
            for i in  self.planes.len()..bt.len(){
                self.planes.push(Object::new(eng, self.plane, self.material, self.font, super::render::render::MeshUsage::LightingPass, true));
            }
        }
        for i in  0..self.planes.len(){
            self.planes[i].mesh.draw = false;
            self.planes[i].mesh.exec();
        }
        for i in 0..bt.len(){
            for j in 0..self.symbols.len(){
                if bt[i] == self.symbols[j] {
                    self.planes[i].mesh.draw = true;
                    self.planes[i].mesh.ubo[16] = self.symbol_number as f32;
                    self.planes[i].mesh.ubo[17] = j as f32;
                    if self.signal && btst && (self.allow_when_mouse_locked || (!self.allow_when_mouse_locked && !eng.control.mouse_lock)){
                        self.planes[i].mesh.ubo[18] = 1.0;
                    }else{
                        self.planes[i].mesh.ubo[18] = 0.0;
                    }
                    self.planes[i].physic_object.scale.x = self.size.x;
                    self.planes[i].physic_object.scale.y = self.size.y;
                    self.planes[i].physic_object.scale.z = 1.0;
                    self.planes[i].physic_object.pos.x = self.pos.x + (i as f32)*self.size.x;
                    self.planes[i].physic_object.pos.y = self.pos.y;
                    self.planes[i].physic_object.pos.z = self.pos.z;
                    self.planes[i].exec(eng);
                    break;
                }
            }
        }
        return btst;
    }
}