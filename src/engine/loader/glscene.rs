use std::fs;

use crate::engine::{loader::{gltf::{GLtypes, Gltf}, imageasset::ImageAsset, jsonparser::JsonF}, math::{vec2::Vec2, vec3::Vec3, vec4::Vec4}};

#[derive(Clone, PartialEq)]
enum Rdbft{
  SCALAR,
  VEC2,
  VEC3,
  VEC4
}

#[derive(Clone, PartialEq)]
enum Aus{
  POSITION,
  NORMAL,
  UV,
  INDICES,
  OTHER,
}

#[derive(Clone, PartialEq)]
enum ChunkType{
  JSON,
  BIN
}

struct GlChunk{
  chunk_type: ChunkType,
  data: Vec<u8>,  
}

struct Rdbf{
  pub tp: Rdbft,
  pub mu: Aus,
  pub scalar: Vec<f32>,
  pub indices: Vec<u32>,
  pub vec2: Vec<Vec2>,
  pub vec3: Vec<Vec3>,
  pub vec4: Vec<Vec4>,
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
pub struct Globject{
  pub name: String,
  pub vertices: Vec<f32>,
  pub position: Vec3,
  pub scale: Vec3,
  pub rot: Vec3,
  pub material: usize,
}

pub struct Glscene{
  pub objs: Vec<Globject>,
  pub material_uri: Vec<Vec<String>>,
  pub material_data: Vec<Vec<ImageAsset>>,
  pub images_bin: bool,
}

impl Glscene{
  pub fn read_gltf_json(path: &str) -> Glscene{
    let seppath: Vec<&str> = path.split("/").collect();
    let mut prefix = "".to_string(); 

    for i in 0..seppath.len()-1{
      prefix += seppath[i];
      prefix += "/";
    }

    let jgltf = JsonF::load_from_file(path);
    let pgltf = Gltf::parse_gltf(jgltf);

    let mut matimg = vec![];
    let mut objvec = vec![];
    let mut rwbf = vec![];

    for i in 0..pgltf.materials.len(){
      let mut uris = vec![];
      for j in 0..pgltf.materials[i].texture_indices.len(){
        let str = format!("{}{}", prefix, pgltf.images[pgltf.textures[pgltf.materials[i].texture_indices[j]].image].uri.clone());
        uris.push(str.clone());
      }
      matimg.push(uris);
    }

    for i in 0..pgltf.buffers.len(){
      rwbf.push(fs::read(format!("{}{}", prefix, pgltf.buffers[i].uri.clone())).unwrap());
    }

    for gobj in pgltf.objects{
      let mesh = pgltf.meshes[gobj.mesh].clone();
      let mut acc = vec![];
      let mut accu = vec![];
      let mut bfv = vec![];

      let mut sbf: Vec<Rdbf> = vec![];

      for i in 0..mesh.attributes.len(){
        acc.push(pgltf.accesories[mesh.attributes[i]].clone());
        if mesh.attributesu[i] == "POSITION"{
          accu.push(Aus::POSITION);
        }else if mesh.attributesu[i] == "NORMAL"{
          accu.push(Aus::NORMAL);
        }else if mesh.attributesu[i] == "TEXCOORD_0"{
          accu.push(Aus::UV);
        }else{
          accu.push(Aus::OTHER);
        }
      }
      if mesh.enable_indices{
        acc.push(pgltf.accesories[mesh.indices].clone());
        accu.push(Aus::INDICES);
      }

      for i in 0..acc.len(){
        bfv.push(pgltf.bufferview[acc[i].bufferview]);
      }

      let mut bfvp = vec![];
      for i in 0..bfv.len(){
        let mut lbf = vec![];
        let to = bfv[i].boffset+bfv[i].blenght;
        for j in bfv[i].boffset..to{
          lbf.push(rwbf[bfv[i].buffer][j]);
        }
        bfvp.push(lbf);
      }

      for i in 0..acc.len(){
        if acc[i].tp == "SCALAR" && accu[i] == Aus::INDICES{
          let mut lbf = vec![];
          match acc[i].component_type {
            GLtypes::SignedByte => {
              for j in 0..bfvp[i].len(){
                lbf.push(i8::from_le_bytes([bfvp[i][j]]) as u32);
              }
            },
            GLtypes::UnsignedByte => {
              for j in 0..bfvp[i].len(){
                lbf.push(u8::from_le_bytes([bfvp[i][j]]) as u32);
              }
            },
            GLtypes::SignedShort => {
              for j in (0..bfvp[i].len()).step_by(2){
                lbf.push(i16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as u32);
              }
            },
            GLtypes::UnsignedShort => {
              for j in (0..bfvp[i].len()).step_by(2){
                lbf.push(u16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as u32);
              }
            },
            GLtypes::UnsignedInt => {
              for j in (0..bfvp[i].len()).step_by(4){
                lbf.push(u32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]) as u32);
              }
            },
            _ => {}
          }
          sbf.push(Rdbf { tp: Rdbft::SCALAR, mu: accu[i].clone(), scalar: vec![], vec2: vec![], vec3: vec![], vec4: vec![], indices: lbf });
        }else if acc[i].tp == "SCALAR" && accu[i] != Aus::INDICES{
          let mut lbf = vec![];
          match acc[i].component_type {
            GLtypes::SignedByte => {
              for j in 0..bfvp[i].len(){
                lbf.push(i8::from_le_bytes([bfvp[i][j]]) as f32);
              }
            },
            GLtypes::UnsignedByte => {
              for j in 0..bfvp[i].len(){
                lbf.push(u8::from_le_bytes([bfvp[i][j]]) as f32);
              }
            },
            GLtypes::SignedShort => {
              for j in (0..bfvp[i].len()).step_by(2){
                lbf.push(i16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as f32);
              }
            },
            GLtypes::UnsignedShort => {
              for j in (0..bfvp[i].len()).step_by(2){
                lbf.push(u16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as f32);
              }
            },
            GLtypes::UnsignedInt => {
              for j in (0..bfvp[i].len()).step_by(4){
                lbf.push(u32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]) as f32);
              }
            },
            GLtypes::Float => {
              for j in (0..bfvp[i].len()).step_by(4){
                lbf.push(f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]));
              }
            },
          }
          sbf.push(Rdbf { tp: Rdbft::SCALAR, mu: accu[i].clone(), scalar: lbf, vec2: vec![], vec3: vec![], vec4: vec![], indices: vec![] });
        }else if acc[i].tp == "VEC2"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(8){
            lbf.push(Vec2{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC2, mu: accu[i].clone(), scalar: vec![], vec2: lbf, vec3: vec![], vec4: vec![], indices: vec![] });
        }else if acc[i].tp == "VEC3"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(12){
            lbf.push(Vec3{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]]),
              z: f32::from_le_bytes([bfvp[i][j+8], bfvp[i][j+9], bfvp[i][j+10], bfvp[i][j+11]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC3, mu: accu[i].clone(), scalar: vec![], vec2: vec![], vec3: lbf, vec4: vec![], indices: vec![] });
        }else if acc[i].tp == "VEC4"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(12){
            lbf.push(Vec4{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]]),
              z: f32::from_le_bytes([bfvp[i][j+8], bfvp[i][j+9], bfvp[i][j+10], bfvp[i][j+11]]),
              w: f32::from_le_bytes([bfvp[i][j+12], bfvp[i][j+13], bfvp[i][j+14], bfvp[i][j+15]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC4, mu: accu[i].clone(), scalar: vec![], vec2: vec![], vec3: vec![], vec4: lbf, indices: vec![] });
        }
      }

      let mut fvrt = vec![];

      let mut pi = 0usize;
      let mut ni = 0usize;
      let mut uvi = 0usize;
      let mut ii = 0usize;

      for i in 0..sbf.len(){
        if sbf[i].mu == Aus::INDICES{
          ii = i;
        }else if sbf[i].mu == Aus::POSITION{
          pi = i;
        }else if sbf[i].mu == Aus::UV{
          uvi = i;
        }else if sbf[i].mu == Aus::NORMAL{
          ni = i;
        }
      }

      for i in 0..sbf[ii].indices.len(){
        fvrt.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].x);
        fvrt.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].y);
        fvrt.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].z);
      }

      for i in 0..sbf[ii].indices.len(){
        fvrt.push(sbf[uvi].vec2[sbf[ii].indices[i] as usize].x);
        fvrt.push(sbf[uvi].vec2[sbf[ii].indices[i] as usize].y);
      }

      for i in 0..sbf[ii].indices.len(){
        fvrt.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].x);
        fvrt.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].y);
        fvrt.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].z);
      }

      objvec.push(Globject{
        name: gobj.name,
        vertices: fvrt,
        position: Vec3 { x: gobj.position.x, y: gobj.position.y, z: gobj.position.z },
        scale: Vec3 { x: gobj.scale.x, y: gobj.scale.y, z: gobj.scale.z },
        rot: quat_to_euler(gobj.rotation),
        material: mesh.material,
      });
    }

    Glscene { 
      objs: objvec,
      material_uri: matimg,
      material_data: vec![],
      images_bin: false,
    }
  }
  pub fn readglb(path: &str) -> Glscene{
    let rglb = fs::read(path).unwrap();

    let mut i = 12usize;
    let mut chunksrd = vec![];

    let mut bini = 1usize;
    let mut jsoni = 0usize;

    while i < rglb.len(){
      let mut chunkd = GlChunk{ chunk_type: ChunkType::BIN, data: vec![] };
      let cl = u32::from_le_bytes([rglb[i], rglb[i+1], rglb[i+2], rglb[i+3]]);
      if rglb[i+4] == b'J' && rglb[i+5] == b'S' && rglb[i+6] == b'O' && rglb[i+7] == b'N'{
        chunkd.chunk_type = ChunkType::JSON;
      }
      i += 8;
      let ti = i;
      for j in ti..(ti+cl as usize){
        chunkd.data.push(rglb[j]);
      }
      chunksrd.push(chunkd);
      i += cl as usize;
    }

    for i in 0..chunksrd.len(){
      if chunksrd[i].chunk_type == ChunkType::BIN{
        bini = i;
      }else if chunksrd[i].chunk_type == ChunkType::JSON{
        jsoni = i;
      }
    }

    let jgltf = JsonF::from_text(&String::from_utf8(chunksrd[jsoni].data.clone()).unwrap());
    let pgltf = Gltf::parse_gltf(jgltf);

    let mut matimg = vec![];
    let mut objvec = vec![];

    for i in 0..pgltf.materials.len(){
      let mut data = vec![];
      for j in 0..pgltf.materials[i].texture_indices.len(){
        let view = pgltf.bufferview[pgltf.images[pgltf.textures[pgltf.materials[i].texture_indices[j]].image].buffer_view];
        let rwimg = chunksrd[bini].data[view.boffset..(view.boffset+view.blenght)].to_vec();
        data.push(ImageAsset::other_parse(rwimg));
      }
      matimg.push(data);
    }

    for gobj in pgltf.objects{
      let mesh = pgltf.meshes[gobj.mesh].clone();
      let mut acc = vec![];
      let mut accu = vec![];
      let mut bfv = vec![];

      let mut sbf: Vec<Rdbf> = vec![];

      for i in 0..mesh.attributes.len(){
        acc.push(pgltf.accesories[mesh.attributes[i]].clone());
        if mesh.attributesu[i] == "POSITION"{
          accu.push(Aus::POSITION);
        }else if mesh.attributesu[i] == "NORMAL"{
          accu.push(Aus::NORMAL);
        }else if mesh.attributesu[i] == "TEXCOORD_0"{
          accu.push(Aus::UV);
        }else{
          accu.push(Aus::OTHER);
        }
      }
      if mesh.enable_indices{
        acc.push(pgltf.accesories[mesh.indices].clone());
        accu.push(Aus::INDICES);
      }

      for i in 0..acc.len(){
        bfv.push(pgltf.bufferview[acc[i].bufferview]);
      }

      let mut bfvp = vec![];
      for i in 0..bfv.len(){
        let mut lbf = vec![];
        let to = bfv[i].boffset+bfv[i].blenght;
        for j in bfv[i].boffset..to{
          lbf.push(chunksrd[bini].data[j]);
        }
        bfvp.push(lbf);
      }

      for i in 0..acc.len(){
        if acc[i].tp == "SCALAR" && accu[i] == Aus::INDICES{
          let mut lbf = vec![];
          match acc[i].component_type {
            GLtypes::SignedByte => {
              for j in 0..bfvp[i].len(){
                lbf.push(i8::from_le_bytes([bfvp[i][j]]) as u32);
              }
            },
            GLtypes::UnsignedByte => {
              for j in 0..bfvp[i].len(){
                lbf.push(u8::from_le_bytes([bfvp[i][j]]) as u32);
              }
            },
            GLtypes::SignedShort => {
              for j in (0..bfvp[i].len()).step_by(2){
                lbf.push(i16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as u32);
              }
            },
            GLtypes::UnsignedShort => {
              for j in (0..bfvp[i].len()).step_by(2){
                lbf.push(u16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as u32);
              }
            },
            GLtypes::UnsignedInt => {
              for j in (0..bfvp[i].len()).step_by(4){
                lbf.push(u32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]) as u32);
              }
            },
            _ => {}
          }
          sbf.push(Rdbf { tp: Rdbft::SCALAR, mu: accu[i].clone(), scalar: vec![], vec2: vec![], vec3: vec![], vec4: vec![], indices: lbf });
        }else if acc[i].tp == "SCALAR" && accu[i] != Aus::INDICES{
          let mut lbf = vec![];
          match acc[i].component_type {
            GLtypes::SignedByte => {
              for j in 0..bfvp[i].len(){
                lbf.push(i8::from_le_bytes([bfvp[i][j]]) as f32);
              }
            },
            GLtypes::UnsignedByte => {
              for j in 0..bfvp[i].len(){
                lbf.push(u8::from_le_bytes([bfvp[i][j]]) as f32);
              }
            },
            GLtypes::SignedShort => {
              for j in (0..bfvp[i].len()).step_by(2){
                lbf.push(i16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as f32);
              }
            },
            GLtypes::UnsignedShort => {
              for j in (0..bfvp[i].len()).step_by(2){
                lbf.push(u16::from_le_bytes([bfvp[i][j], bfvp[i][j+1]]) as f32);
              }
            },
            GLtypes::UnsignedInt => {
              for j in (0..bfvp[i].len()).step_by(4){
                lbf.push(u32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]) as f32);
              }
            },
            GLtypes::Float => {
              for j in (0..bfvp[i].len()).step_by(4){
                lbf.push(f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]));
              }
            },
          }
          sbf.push(Rdbf { tp: Rdbft::SCALAR, mu: accu[i].clone(), scalar: lbf, vec2: vec![], vec3: vec![], vec4: vec![], indices: vec![] });
        }else if acc[i].tp == "VEC2"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(8){
            lbf.push(Vec2{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC2, mu: accu[i].clone(), scalar: vec![], vec2: lbf, vec3: vec![], vec4: vec![], indices: vec![] });
        }else if acc[i].tp == "VEC3"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(12){
            lbf.push(Vec3{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]]),
              z: f32::from_le_bytes([bfvp[i][j+8], bfvp[i][j+9], bfvp[i][j+10], bfvp[i][j+11]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC3, mu: accu[i].clone(), scalar: vec![], vec2: vec![], vec3: lbf, vec4: vec![], indices: vec![] });
        }else if acc[i].tp == "VEC4"{
          let mut lbf = vec![];
          for j in (0..bfvp[i].len()).step_by(12){
            lbf.push(Vec4{ 
              x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
              y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]]),
              z: f32::from_le_bytes([bfvp[i][j+8], bfvp[i][j+9], bfvp[i][j+10], bfvp[i][j+11]]),
              w: f32::from_le_bytes([bfvp[i][j+12], bfvp[i][j+13], bfvp[i][j+14], bfvp[i][j+15]])
            });
          }
          sbf.push(Rdbf { tp: Rdbft::VEC4, mu: accu[i].clone(), scalar: vec![], vec2: vec![], vec3: vec![], vec4: lbf, indices: vec![] });
        }
      }

      let mut fvrt = vec![];

      let mut pi = 0usize;
      let mut ni = 0usize;
      let mut uvi = 0usize;
      let mut ii = 0usize;

      for i in 0..sbf.len(){
        if sbf[i].mu == Aus::INDICES{
          ii = i;
        }else if sbf[i].mu == Aus::POSITION{
          pi = i;
        }else if sbf[i].mu == Aus::UV{
          uvi = i;
        }else if sbf[i].mu == Aus::NORMAL{
          ni = i;
        }
      }

      for i in 0..sbf[ii].indices.len(){
        fvrt.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].x);
        fvrt.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].y);
        fvrt.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].z);
      }

      for i in 0..sbf[ii].indices.len(){
        fvrt.push(sbf[uvi].vec2[sbf[ii].indices[i] as usize].x);
        fvrt.push(sbf[uvi].vec2[sbf[ii].indices[i] as usize].y);
      }

      for i in 0..sbf[ii].indices.len(){
        fvrt.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].x);
        fvrt.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].y);
        fvrt.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].z);
      }

      objvec.push(Globject{
        name: gobj.name,
        vertices: fvrt,
        position: Vec3 { x: gobj.position.x, y: gobj.position.y, z: gobj.position.z },
        scale: Vec3 { x: gobj.scale.x, y: gobj.scale.y, z: gobj.scale.z },
        rot: quat_to_euler(gobj.rotation),
        material: mesh.material,
      });
    }

    Glscene { 
      objs: objvec,
      material_uri: vec![],
      material_data: matimg,
      images_bin: true,
    }
  }
}