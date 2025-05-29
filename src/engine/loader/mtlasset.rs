#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::File, io::{BufRead, BufReader}};

pub struct MtlAsset{
    pub matinfo: Vec<Vec<String>>,
}

impl MtlAsset{
    pub fn load_mtl(path: &str) -> MtlAsset{
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut fmat: Vec<Vec<String>> = vec![];
        for line in reader.lines(){
            let va= line.unwrap_or_default();
            let spl: Vec<&str> = va.split(' ').collect();
            if spl[0] == "newmtl"{
                //fmat.push(vec![spl[1].to_string()]);
                fmat.push(vec![]);
            }
            if spl[0] == "map_Ka" || 
                spl[0] == "map_Kd" || 
                spl[0] == "map_Ks" || 
                spl[0] == "map_Ns" || 
                spl[0] == "map_d" || 
                spl[0] == "map_bump" || 
                spl[0] == "bump" || 
                spl[0] == "disp" || 
                spl[0] == "decal" ||
                spl[0] == "Pr/map_Pr" || 
                spl[0] == "Pm/map_Pm" || 
                spl[0] == "Ps/map_Ps" || 
                spl[0] == "Pc" || 
                spl[0] == "Pcr" || 
                spl[0] == "Ke/map_Ke" || 
                spl[0] == "aniso" || 
                spl[0] == "anisor" || 
                spl[0] == "norm"{
                let index = fmat.len()-1;

                let pspl: Vec<&str> = path.split('/').collect();
                let mut mtlp: String = "".to_string();
                for i in 0..pspl.len()-1{
                    mtlp += &pspl[i].to_string();
                    mtlp += "/";
                }
                mtlp += &spl[1].to_string();

                println!("MTLLoader: MTL texture by path: {}", mtlp);

                fmat[index].push(mtlp);
            }
        }
        MtlAsset { 
            matinfo: fmat 
        }
    }
}