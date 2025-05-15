#![allow(dead_code)]
#![allow(unused_variables)]

use super::{engine::Engine, render::render::Texture};

#[derive(Copy, Clone)]
pub struct Image{
    pub textures: Texture,
}

impl Image{
    pub fn new(eng: Engine, size: [u32; 3], data: Vec<i8>) -> Image{
        Image{
            textures: Texture::new(eng.render, size[0], size[1], size[2], data),
        }
    }
}