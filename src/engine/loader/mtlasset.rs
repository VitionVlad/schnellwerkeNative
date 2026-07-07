#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::loader::rw::readfs;

pub struct MtlAsset{
    pub matinfo: Vec<Vec<String>>,
    pub matnam: Vec<String>,
}

impl MtlAsset{
    pub fn load_mtl(path: &str) -> MtlAsset{
        let file = String::from_utf8(readfs(path)).unwrap();
        let reader: Vec<&str> = file.split('\n').collect();
        let mut fmat: Vec<Vec<String>> = vec![];
        let mut fnv: Vec<String> = vec![];
        for i in 0..reader.len() {
            let va = reader[i];
            let spl: Vec<&str> = va.split(' ').collect();
            if spl[0] == "newmtl"{
                fnv.push(spl[1].to_string());
                fmat.push(vec![]);
            }
            if spl[0] == "map_Ka" || 
                spl[0] == "map_Kd" || 
                spl[0] == "map_Ks" || 
                spl[0] == "map_Ns" || 
                spl[0] == "map_d" || 
                spl[0] == "map_refl" || 
                spl[0] == "map_Bump" || 
                spl[0] == "Pr/map_Pr" || 
                spl[0] == "Pm/map_Pm" || 
                spl[0] == "Ps/map_Ps" || 
                spl[0] == "Ke/map_Ke" {
                let index = fmat.len()-1;

                let pspl: Vec<&str> = path.split('/').collect();
                let mut mtlp: String = "".to_string();
                for i in 0..pspl.len()-1{
                    mtlp += &pspl[i].to_string();
                    mtlp += "/";
                }
                mtlp += &spl[spl.len()-1].to_string();

                fmat[index].push(mtlp);
            }
        }
        MtlAsset { 
            matinfo: fmat, 
            matnam: fnv,
        }
    }
}