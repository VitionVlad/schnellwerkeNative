#![allow(dead_code)]
#![allow(unused_variables)]

use super::{engine::Engine, loader::imageasset::ImageAsset, render::render::Texture};

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
    pub fn new_color(eng: &Engine, color: [i8; 4]) -> Image{
        Image{
            textures: Texture::new(eng.render, 1, 1, 1, color.to_vec()),
        }
    }
    pub fn new_from_files(eng: &Engine, paths: Vec<String>) -> Image{
        let mut size: [u32; 3] = [0, 0, paths.len() as u32];
        let mut data: Vec<i8> = vec![];
        for i in 0..paths.len(){
            let spl = paths[i].as_bytes();
            if spl[spl.len()-1] == b'a' && spl[spl.len()-2] == b'g' && spl[spl.len()-3] == b't'{
                let mut ia = ImageAsset::load_tga(&paths[i]);
                size[0] = ia.size[0];
                size[1] = ia.size[1];
                data.append(&mut ia.data);
            }
            if spl[spl.len()-1] == b'f' && spl[spl.len()-2] == b'f' && spl[spl.len()-3] == b'i' && spl[spl.len()-4] == b't'{
                let mut ia = ImageAsset::load_tiff(&paths[i]);
                size[0] = ia.size[0];
                size[1] = ia.size[1];
                data.append(&mut ia.data);
            }
        }
        Image{
            textures: Texture::new(eng.render, size[0], size[1], size[2], data),
        }
    }
}