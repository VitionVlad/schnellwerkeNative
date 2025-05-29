#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::File, io::{BufRead, BufReader}};

use super::mtlasset::MtlAsset;

pub struct ModelAsset{
    pub vertices: Vec<Vec<f32>>,
    pub mtl: MtlAsset,
}

impl ModelAsset{
    pub fn load_obj(path: &str) -> ModelAsset{
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut vert: Vec<[f32; 3]> = vec![];
        let mut uv: Vec<[f32; 2]> = vec![];
        let mut norm: Vec<[f32; 3]> = vec![];

        let mut ivert: Vec<u32> = vec![];
        let mut iuv: Vec<u32> = vec![];
        let mut inorm: Vec<u32> = vec![];

        let mut fnvrt: Vec<f32> = vec![];
        let mut fnobj: Vec<Vec<f32>> = vec![];
        
        let mut objcnt = 0usize;
        let mut objbegind: Vec<[usize; 3]> = vec![];

        let mut mtl: MtlAsset = MtlAsset { matinfo: vec![] };
        for line in reader.lines() {
            let va = line.unwrap_or_default();
            if va.clone().chars().next().unwrap_or_default() == '#' {
                continue;
            }
            let spl: Vec<&str> = va.split(' ').collect();
            if spl[0] == "mtllib"{
                let pspl: Vec<&str> = path.split('/').collect();
                let mut mtlp: String = "".to_string();
                for i in 0..pspl.len()-1{
                    mtlp += &pspl[i].to_string();
                    mtlp += "/";
                }
                mtlp += &spl[1].to_string();
                println!("ModelLoader: Loading mtl by path: {}", mtlp);
                mtl = MtlAsset::load_mtl(&mtlp);
                continue;
            }
            if va.clone().as_bytes()[0] == b'o' && va.clone().as_bytes()[1] == b' '{
                objcnt += 1usize;
                objbegind.push([ivert.len(), iuv.len(), inorm.len()]);
                continue;
            }
            if va.clone().as_bytes()[0] == b'v' && va.clone().as_bytes()[1] == b' '{
                let spl: Vec<&str> = va.split(' ').collect();
                let pos: [f32; 3] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap(), spl[3].parse::<f32>().unwrap()];
                vert.push(pos);
                println!("ModelLoader: obj vert = ({}, {}, {})", pos[0], pos[1], pos[2]);
                continue;
            }
            if va.clone().as_bytes()[0] == b'v' && va.clone().as_bytes()[1] == b't'{
                let spl: Vec<&str> = va.split(' ').collect();
                let uvc: [f32; 2] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap()];
                uv.push(uvc);
                println!("ModelLoader: obj uv = ({}, {})", uvc[0], uvc[1]);
                continue;
            }
            if va.clone().as_bytes()[0] == b'v' && va.clone().as_bytes()[1] == b'n'{
                let spl: Vec<&str> = va.split(' ').collect();
                let normal: [f32; 3] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap(), spl[3].parse::<f32>().unwrap()];
                norm.push(normal);
                println!("ModelLoader: obj normal = ({}, {}, {})", normal[0], normal[1], normal[2]);
                continue;
            }
            if va.clone().as_bytes()[0] == b'f' && va.clone().as_bytes()[1] == b' '{
                let spl: Vec<&str> = va.split(' ').collect();
                let spl3: [Vec<&str>; 3] = [spl[1].split('/').collect(), spl[2].split('/').collect(), spl[3].split('/').collect()];
                let posi: [u32; 3] = [spl3[0][0].parse::<u32>().unwrap(), spl3[1][0].parse::<u32>().unwrap(), spl3[2][0].parse::<u32>().unwrap()];
                let uvi: [u32; 3] = [spl3[0][1].parse::<u32>().unwrap(), spl3[1][1].parse::<u32>().unwrap(), spl3[2][1].parse::<u32>().unwrap()];
                let normali: [u32; 3] = [spl3[0][2].parse::<u32>().unwrap(), spl3[1][2].parse::<u32>().unwrap(), spl3[2][2].parse::<u32>().unwrap()];
                inorm.push(normali[0]);
                inorm.push(normali[1]);
                inorm.push(normali[2]);

                ivert.push(posi[0]);
                ivert.push(posi[1]);
                ivert.push(posi[2]);

                iuv.push(uvi[0]);
                iuv.push(uvi[1]);
                iuv.push(uvi[2]);
                println!("ModelLoader: obj face = p({}, {}, {}) u({}, {}, {}) n({}, {}, {})", posi[0], posi[1], posi[2], uvi[0], uvi[1], uvi[2], normali[0], normali[1], normali[2]);
                continue;
            }
        }
        objbegind.push([ivert.len(), iuv.len(), inorm.len()]);
        for j in 0..objcnt{
            for i in objbegind[j][0]..objbegind[j+1][0]{
                fnvrt.push(vert[ivert[i] as usize - 1][0]);
                fnvrt.push(vert[ivert[i] as usize - 1][1]);
                fnvrt.push(vert[ivert[i] as usize - 1][2]);
            }
            for i in objbegind[j][1]..objbegind[j+1][1]{
                fnvrt.push(uv[iuv[i] as usize - 1][0]);
                fnvrt.push(uv[iuv[i] as usize - 1][1]);
            }
            for i in objbegind[j][2]..objbegind[j+1][2]{
                fnvrt.push(norm[inorm[i] as usize - 1][0]);
                fnvrt.push(norm[inorm[i] as usize - 1][1]);
                fnvrt.push(norm[inorm[i] as usize - 1][2]);
            }
            fnobj.push(fnvrt.clone());
            fnvrt = vec![];
        }
        ModelAsset { 
            vertices: fnobj, 
            mtl: mtl,
        }
    }
}