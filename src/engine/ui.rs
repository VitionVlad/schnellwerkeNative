#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::render::render::{MaterialShaders, Texture, Vertexes};

use super::{engine::Engine, image::Image, material::Material, math::{vec2::Vec2, vec3::Vec3}, model::Model, object::Object, plane::PLANEUI};

#[derive(Clone)]
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
    pub ubo_index: usize,
}

impl UIplane {
    pub fn new(eng: &mut Engine, mat: Material, image: Image) -> UIplane{
        let model = Model::new(&eng, PLANEUI.to_vec());
        UIplane { 
            object: Object::new(eng, model, mat, image, super::render::render::MeshUsage::LightingPass, true),
            clickzone: Clickzone { pos1: Vec2::newdefined(0.0, 0.0), pos2: Vec2::newdefined(0.0, 0.0) },
            signal: false,
            allow_when_mouse_locked: false,
            ubo_index: 50,
        }
    }
    pub fn new_blank() -> UIplane{
        UIplane { 
            object: Object::new_blank(),
            clickzone: Clickzone { pos1: Vec2::newdefined(0.0, 0.0), pos2: Vec2::newdefined(0.0, 0.0) },
            signal: false,
            allow_when_mouse_locked: false,
            ubo_index: 50,
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
            ubo_index: 50,
        }
    }
    pub fn exec(&mut self, eng: &mut Engine) -> bool{
        self.clickzone.pos1.x = self.object.physic_object.pos.x;
        self.clickzone.pos1.y = self.object.physic_object.pos.y;
        self.clickzone.pos2.x = self.object.physic_object.pos.x + self.object.physic_object.scale.x;
        self.clickzone.pos2.y = self.object.physic_object.pos.y + self.object.physic_object.scale.y;
        let btst = self.clickzone.check(Vec2::newdefined(eng.control.xpos as f32, eng.control.ypos as f32));
        if self.signal{
            if btst && (self.allow_when_mouse_locked || (!self.allow_when_mouse_locked && !eng.control.mouse_lock)) && self.object.draw{
                self.object.mesh.ubo[self.ubo_index] = 1.0;
            }else{
                self.object.mesh.ubo[self.ubo_index] = 0.0;
            }
        }
        self.object.exec(eng);
        return btst;
    }
}

#[derive(Clone)]
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
    pub per_symbol: bool,
    pub allow_when_mouse_locked: bool,
    pub draw: bool,
    pub symbol_pressed: u8,
    pub symbol_index: usize,
    pub ubo_index: usize,
    blank: bool,
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
            per_symbol: true,
            allow_when_mouse_locked: false,
            blank: false,
            symbol_pressed: b' ',
            symbol_index: 0,
            ubo_index: 48,
            draw: true,
        }
    }
    pub fn new_blank() -> UItext{
        UItext{
            plane: Model { vertexbuf: Vertexes{ modelid: 0 }, points: [Vec3::new(), Vec3::new()] },
            font: Image { textures: Texture{ texid: 0 } },
            symbols: "".as_bytes().to_vec(),
            planes: vec![],
            symbol_number: 0,
            material: Material { material_shaders: MaterialShaders{ materialid: 0 } },
            size: Vec2::newdefined(20.0, 40.0),
            pos: Vec3::new(),
            clickzone: Clickzone { pos1: Vec2::newdefined(0.0, 0.0), pos2: Vec2::newdefined(0.0, 0.0) },
            signal: false,
            per_symbol: true,
            allow_when_mouse_locked: false,
            blank: true,
            symbol_pressed: b' ',
            symbol_index: 0,
            ubo_index: 48,
            draw: false,
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
            per_symbol: true,
            allow_when_mouse_locked: false,
            blank: false,
            symbol_pressed: b' ',
            symbol_index: 0,
            ubo_index: 48,
            draw: true,
        }
    }
    pub fn exec(&mut self, eng: &mut Engine, text: &str) -> bool{
        if !self.blank{
            let bt = text.as_bytes();
            let mut mx: u32 = 1;
            let mut my: u32 = 1;
            let mut cx: u32 = 0;
            for i in 0..bt.len(){
                cx+=1;
                if bt[i] == b'\n'{
                    my+=1;
                    if cx-1 >= mx {
                        mx = cx-1;
                    }
                    cx = 1;
                }
            }
            if cx-1 >= mx {
                mx = cx-1;
            }
            self.clickzone.pos1.x = self.pos.x;
            self.clickzone.pos1.y = self.pos.y;
            self.clickzone.pos2.x = self.pos.x + self.size.x*(mx as f32 + 1.0);
            self.clickzone.pos2.y = self.pos.y + self.size.y*(my as f32);
            let mut btst = self.clickzone.check(Vec2::newdefined(eng.control.xpos as f32, eng.control.ypos as f32));
            let mut lbtst = btst;
            if self.planes.len() < bt.len() {
                for i in  self.planes.len()..bt.len(){
                    self.planes.push(Object::new(eng, self.plane, self.material, self.font, super::render::render::MeshUsage::LightingPass, true));
                }
            }
            for i in  0..self.planes.len(){
                self.planes[i].mesh.draw = false;
                self.planes[i].mesh.exec();
            }
            let mut posy: f32 = self.pos.y;
            let mut bp: usize = 0;
            if self.draw{
                for i in 0..bt.len(){
                    for j in 0..self.symbols.len(){
                        if bt[i] == self.symbols[j] {
                            self.planes[i].mesh.draw = true;
                            self.planes[i].mesh.ubo[self.ubo_index] = self.symbol_number as f32;
                            self.planes[i].mesh.ubo[self.ubo_index+1] = j as f32;
                            self.planes[i].physic_object.scale.x = self.size.x;
                            self.planes[i].physic_object.scale.y = self.size.y;
                            self.planes[i].physic_object.scale.z = 1.0;
                            self.planes[i].physic_object.pos.x = self.pos.x + ((i - bp) as f32)*self.size.x;
                            self.planes[i].physic_object.pos.y = posy;
                            self.planes[i].physic_object.pos.z = self.pos.z;

                            if self.per_symbol{
                                self.clickzone.pos1.x = self.planes[i].physic_object.pos.x;
                                self.clickzone.pos1.y = self.planes[i].physic_object.pos.y;
                                self.clickzone.pos2.x = self.planes[i].physic_object.pos.x + self.size.x;
                                self.clickzone.pos2.y = self.planes[i].physic_object.pos.y + self.size.y;
                                lbtst = self.clickzone.check(Vec2::newdefined(eng.control.xpos as f32, eng.control.ypos as f32));
                                if lbtst{
                                    self.symbol_pressed = self.symbols[j];
                                    self.symbol_index = i;
                                    btst = true;
                                }
                            }
                            if self.signal{
                                if lbtst && (self.allow_when_mouse_locked || (!self.allow_when_mouse_locked && !eng.control.mouse_lock)){
                                    self.planes[i].mesh.ubo[self.ubo_index+2] = 1.0;
                                }else{
                                    self.planes[i].mesh.ubo[self.ubo_index+2] = 0.0;
                                }
                            }

                            self.planes[i].exec(eng);
                            break;
                        }
                        if bt[i] == b'\n' {
                            posy+=self.size.y;
                            bp = i+1;
                            break;
                        }
                    }
                }
            }
            return btst;
        }
        return false;
    }
}