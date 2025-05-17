#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs, i8, vec};

pub struct ImageAsset{
    pub data: Vec<i8>,
    pub size: [u32; 2],
}

impl ImageAsset{
    pub fn load_tga(path: &str) -> ImageAsset{
        let tga = fs::read(path).unwrap();
        let size16: [u32; 2] = [ ((tga[12] as u32) << 16) | ((tga[13] as u32) << 8) , ((tga[14] as u32) << 16) | ((tga[15] as u32) << 8)];
        println!("ImageLoader: Image resolutions = {}x{}", size16[0], size16[1]);
        println!("ImageLoader: file size = {}", tga.len());
        let mut data: Vec<i8> = vec![];
        let sz = size16[0] * size16[1] * 3;
        println!("ImageLoader: Expected image size = {}", sz);
        for i in (0..sz as usize).step_by(3) {
            data.push(tga[i+18] as i8);
            data.push(tga[i+19] as i8);
            data.push(tga[i+20] as i8);
            data.push(i8::MAX);
        }
        println!("ImageLoader: Image size = {}", data.len());
        ImageAsset { 
            data: data, 
            size: [size16[0], size16[1]] 
        }
    }
    pub fn load_tiff(path: &str) -> ImageAsset{
        let tiff = fs::read(path).unwrap();
        let mut size: [u32; 2] = [0, 0];
        let idfoffset: u32 = (tiff[7] as u32) << 24 | (tiff[6] as u32) << 16 | (tiff[5] as u32) << 8 | (tiff[4] as u32);
        let pixelcnt = (idfoffset - 8)/3;
        println!("ImageLoader: TIFF idf offset = {}", idfoffset);
        let argcnt = ((tiff[idfoffset as usize + 1] as u32) << 8) | (tiff[idfoffset as usize] as u32);
        println!("ImageLoader: TIFF idf cnt = {}", argcnt);
        for i in (idfoffset+2..idfoffset+2+argcnt*12).step_by(12){
            let tag = ((tiff[i as usize+1] as u16) << 8) | tiff[i as usize] as u16;
            println!("ImageLoader: TIFF idf tag = {}", tag);
            if tag == 256 {
                size[0] = (tiff[i as usize + 11] as u32) << 24 | (tiff[i as usize + 10] as u32) << 16 | (tiff[i as usize + 9] as u32) << 8 | (tiff[i as usize + 8] as u32);
                size[1] = pixelcnt/size[0];
                println!("ImageLoader: TIFF image resolution = {}x{}", size[0], size[1]);
                break;
            }
        }
        let mut data: Vec<i8> = vec![];
        for i in (8..idfoffset).step_by(3){
            data.push(tiff[i as usize] as i8);
            data.push(tiff[i as usize + 1] as i8);
            data.push(tiff[i as usize + 2] as i8);
            data.push(i8::MAX);
        }
        ImageAsset { 
            data: data, 
            size: size, 
        }
    }
}