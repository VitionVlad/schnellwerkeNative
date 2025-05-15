#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::File, io::{BufRead, BufReader}};

pub struct ImageLoader{
    pub data: Vec<i8>,
    pub size: [u32; 2],
}

impl ImageLoader{
    pub fn load_ppm(path: &str){
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut size: [u32; 2] = [0, 0];
        let mut skipline = false;
        let data: Vec<i8> = Vec::new();
        for line in reader.lines(){
            if skipline {
                skipline = !skipline;
                continue;
            }
            let va = line.unwrap_or_default();
            if va.clone().chars().next().unwrap_or_default() == '#' || va == "P6" {
                continue;
            }
            if size[0] == 0 && size[1] == 0 {
                let sizes: Vec<i32> = va.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
                size[0] = sizes[0] as u32;
                size[1] = sizes[1] as u32;
                if sizes.len() < 3{
                    skipline = true;
                }
                println!("ImageLoader: image size = {}, {}", size[0], size[1]);
                continue;
            }
            println!("ImageLoader: image data = {}", va);
        }
    }
}