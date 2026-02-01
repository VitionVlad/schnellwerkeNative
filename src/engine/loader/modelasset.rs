#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::File, io::{BufRead, BufReader}};

use crate::engine::math::{vec2::Vec2, vec3::Vec3, vec4::Vec4};

use super::mtlasset::MtlAsset;

#[derive(Clone)]
#[derive(PartialEq)]
enum Rdbft{
  SCALAR,
  VEC2,
  VEC3,
  VEC4
}

#[derive(Clone)]
#[derive(PartialEq)]
enum Aus{
  POSITION,
  NORMAL,
  UV,
  INDICES,
  OTHER,
}

struct Rdbf{
  tp: Rdbft,
  mu: Aus,
  scalar: Vec<u32>,
  vec2: Vec<Vec2>,
  vec3: Vec<Vec3>,
  vec4: Vec<Vec4>,
}

fn quat_to_euler(q: Vec4) -> Vec3 {
    let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
    let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
    let roll = sinr_cosp.atan2(cosr_cosp);

    let sinp = 2.0 * (q.w * q.y - q.z * q.x);
    let pitch = if sinp.abs() >= 1.0 {
        std::f32::consts::FRAC_PI_2.copysign(sinp)
    } else {
        sinp.asin()
    };

    let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
    let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
    let yaw = siny_cosp.atan2(cosy_cosp);

    Vec3 { x: roll, y: pitch, z: yaw }
}

pub struct ModelAsset{
    pub vertices: Vec<Vec<f32>>,
    pub matnam: Vec<String>,
    pub objpos: Vec<Vec3>,
    pub objrot: Vec<Vec3>,
    pub objscale: Vec<Vec3>,
    pub obn: Vec<String>,
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

        let mut mtl: MtlAsset = MtlAsset { matinfo: vec![], matnam: vec![] };

        let mut mtsl: Vec<String> = vec![];
        let mut obn: Vec<String> = vec![];
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
                mtl = MtlAsset::load_mtl(&mtlp);
                continue;
            }
            if spl[0] == "usemtl"{
                mtsl.push(spl[1].to_owned());
                continue;
            }
            if va.clone().as_bytes()[0] == b'o' && va.clone().as_bytes()[1] == b' '{
                obn.push(spl[1].to_string());
                objcnt += 1usize;
                objbegind.push([ivert.len(), iuv.len(), inorm.len()]);
                continue;
            }
            if va.clone().as_bytes()[0] == b'v' && va.clone().as_bytes()[1] == b' '{
                let spl: Vec<&str> = va.split(' ').collect();
                let pos: [f32; 3] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap(), spl[3].parse::<f32>().unwrap()];
                vert.push(pos);
                continue;
            }
            if va.clone().as_bytes()[0] == b'v' && va.clone().as_bytes()[1] == b't'{
                let spl: Vec<&str> = va.split(' ').collect();
                let uvc: [f32; 2] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap()];
                uv.push(uvc);
                continue;
            }
            if va.clone().as_bytes()[0] == b'v' && va.clone().as_bytes()[1] == b'n'{
                let spl: Vec<&str> = va.split(' ').collect();
                let normal: [f32; 3] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap(), spl[3].parse::<f32>().unwrap()];
                norm.push(normal);
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
            matnam: mtsl,
            obn: obn,
            objpos: vec![],
            objrot: vec![],
            objscale: vec![],
            mtl: mtl,
        }
    }
}