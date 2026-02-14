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
  pub fn perobj(pgltf: Gltf, bfvp: Vec<Vec<u8>>) -> Vec<Globject>{
    let mut objvec = vec![];

    let mut sbf: Vec<Rdbf> = vec![];

    for i in 0..pgltf.accesories.len(){
      let bfvi = pgltf.accesories[i].bufferview;
      if pgltf.accesories[i].tp == "SCALAR" && pgltf.accesories[i].component_type != GLtypes::Float{
        let mut lbf = vec![];
        match pgltf.accesories[i].component_type {
          GLtypes::SignedByte => {
            for j in 0..bfvp[bfvi].len(){
              lbf.push(i8::from_le_bytes([bfvp[bfvi][j]]) as u32);
            }
          },
          GLtypes::UnsignedByte => {
            for j in 0..bfvp[bfvi].len(){
              lbf.push(u8::from_le_bytes([bfvp[bfvi][j]]) as u32);
            }
          },
          GLtypes::SignedShort => {
            for j in (0..bfvp[bfvi].len()).step_by(2){
              lbf.push(i16::from_le_bytes([bfvp[bfvi][j], bfvp[bfvi][j+1]]) as u32);
            }
          },
          GLtypes::UnsignedShort => {
            for j in (0..bfvp[bfvi].len()).step_by(2){
              lbf.push(u16::from_le_bytes([bfvp[bfvi][j], bfvp[bfvi][j+1]]) as u32);
            }
          },
          GLtypes::UnsignedInt => {
            //for j in (0..bfvp[bfvi].len()).step_by(4){
            //  lbf.push(u32::from_le_bytes([bfvp[bfvi][j], bfvp[bfvi][j+1], bfvp[bfvi][j+2], bfvp[bfvi][j+3]]) as u32);
            //}
            unsafe {
              lbf = std::slice::from_raw_parts(
                bfvp[bfvi][0..bfvp[bfvi].len()].as_ptr() as *const u32,
                bfvp[bfvi].len() / 4,
              ).to_vec();
            }
          },
          _ => {}
        }
        sbf.push(Rdbf { tp: Rdbft::SCALAR, mu: Aus::OTHER, scalar: vec![], vec2: vec![], vec3: vec![], vec4: vec![], indices: lbf });
      }else if pgltf.accesories[i].tp == "SCALAR" && pgltf.accesories[i].component_type == GLtypes::Float{
        let mut lbf = vec![];
        //for j in (0..bfvp[bfvi].len()).step_by(4){
        //  lbf.push(f32::from_le_bytes([bfvp[bfvi][j], bfvp[bfvi][j+1], bfvp[bfvi][j+2], bfvp[bfvi][j+3]]));
        //}
        unsafe {
          lbf = std::slice::from_raw_parts(
            bfvp[bfvi][0..bfvp[bfvi].len()].as_ptr() as *const f32,
            bfvp[bfvi].len() / 4,
          ).to_vec();
        }
        sbf.push(Rdbf { tp: Rdbft::SCALAR, mu: Aus::OTHER, scalar: lbf, vec2: vec![], vec3: vec![], vec4: vec![], indices: vec![] });
      }else if pgltf.accesories[i].tp == "VEC2"{
        let mut lbf = vec![];
        //for j in (0..bfvp[i].len()).step_by(8){
        //  lbf.push(Vec2{ 
        //    x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
        //    y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]])
        //  });
        //}
        unsafe {
          lbf = std::slice::from_raw_parts(
            bfvp[bfvi][0..bfvp[bfvi].len()].as_ptr() as *const Vec2,
            bfvp[bfvi].len() / 8,
          ).to_vec();
        }
        sbf.push(Rdbf { tp: Rdbft::VEC2, mu: Aus::OTHER, scalar: vec![], vec2: lbf, vec3: vec![], vec4: vec![], indices: vec![] });
      }else if pgltf.accesories[i].tp == "VEC3"{
        let mut lbf = vec![];
        //for j in (0..bfvp[i].len()).step_by(12){
        //  lbf.push(Vec3{ 
        //    x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
        //    y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]]),
        //    z: f32::from_le_bytes([bfvp[i][j+8], bfvp[i][j+9], bfvp[i][j+10], bfvp[i][j+11]])
        //  });
        //}
        unsafe {
          lbf = std::slice::from_raw_parts(
            bfvp[bfvi][0..bfvp[bfvi].len()].as_ptr() as *const Vec3,
            bfvp[bfvi].len() / 12,
          ).to_vec();
        }
        sbf.push(Rdbf { tp: Rdbft::VEC3, mu: Aus::OTHER, scalar: vec![], vec2: vec![], vec3: lbf, vec4: vec![], indices: vec![] });
      }else if pgltf.accesories[i].tp == "VEC4"{
        let mut lbf = vec![];
        //for j in (0..bfvp[i].len()).step_by(16){
        //  lbf.push(Vec4{ 
        //    x: f32::from_le_bytes([bfvp[i][j], bfvp[i][j+1], bfvp[i][j+2], bfvp[i][j+3]]), 
        //    y: f32::from_le_bytes([bfvp[i][j+4], bfvp[i][j+5], bfvp[i][j+6], bfvp[i][j+7]]),
        //    z: f32::from_le_bytes([bfvp[i][j+8], bfvp[i][j+9], bfvp[i][j+10], bfvp[i][j+11]]),
        //    w: f32::from_le_bytes([bfvp[i][j+12], bfvp[i][j+13], bfvp[i][j+14], bfvp[i][j+15]])
        //  });
        //}
        unsafe {
          lbf = std::slice::from_raw_parts(
            bfvp[bfvi][0..bfvp[bfvi].len()].as_ptr() as *const Vec4,
            bfvp[bfvi].len() / 16,
          ).to_vec();
        }
        sbf.push(Rdbf { tp: Rdbft::VEC4, mu: Aus::OTHER, scalar: vec![], vec2: vec![], vec3: vec![], vec4: lbf, indices: vec![] });
      }
    }

    for gobj in pgltf.objects{
      let mesh = pgltf.meshes[gobj.mesh].clone();
      let mut acc = vec![];
      let mut accu = vec![];

      for i in 0..mesh.attributes.len(){
        acc.push(mesh.attributes[i]);
        if mesh.attributesu[i] == "POSITION"{
          sbf[mesh.attributes[i]].mu = Aus::INDICES;
          accu.push(Aus::POSITION);
        }else if mesh.attributesu[i] == "NORMAL"{
          sbf[mesh.attributes[i]].mu = Aus::INDICES;
          accu.push(Aus::NORMAL);
        }else if mesh.attributesu[i] == "TEXCOORD_0"{
          sbf[mesh.attributes[i]].mu = Aus::INDICES;
          accu.push(Aus::UV);
        }else{
          accu.push(Aus::OTHER);
        }
      }
      if mesh.enable_indices{
        acc.push(mesh.indices);
        sbf[mesh.indices].mu = Aus::INDICES;
        accu.push(Aus::INDICES);
      }

      let mut fvert = vec![];
      let mut fuv = vec![];
      let mut fnorm = vec![];
      let mut fvrt = vec![];

      let mut pi = 0usize;
      let mut ni = 0usize;
      let mut uvi = 0usize;
      let mut ii = 0usize;

      for i in 0..accu.len(){
        if accu[i] == Aus::INDICES{
          ii = acc[i];
        }else if accu[i] == Aus::POSITION{
          pi = acc[i];
        }else if accu[i] == Aus::UV{
          uvi = acc[i];
        }else if accu[i] == Aus::NORMAL{
          ni = acc[i];
        }
      }

      for i in 0..sbf[ii].indices.len(){
        fvert.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].x);
        fvert.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].y);
        fvert.push(sbf[pi].vec3[sbf[ii].indices[i] as usize].z);

        fuv.push(sbf[uvi].vec2[sbf[ii].indices[i] as usize].x);
        fuv.push(sbf[uvi].vec2[sbf[ii].indices[i] as usize].y);

        fnorm.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].x);
        fnorm.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].y);
        fnorm.push(sbf[ni].vec3[sbf[ii].indices[i] as usize].z);
      }

      fvrt.append(&mut fvert);
      fvrt.append(&mut fuv);
      fvrt.append(&mut fnorm);

      objvec.push(Globject{
        name: gobj.name,
        vertices: fvrt,
        position: Vec3 { x: gobj.position.x, y: gobj.position.y, z: gobj.position.z },
        scale: Vec3 { x: gobj.scale.x, y: gobj.scale.y, z: gobj.scale.z },
        rot: quat_to_euler(gobj.rotation),
        material: mesh.material,
      });
    }

    objvec
  }
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

    let mut bfvp = vec![];
    for i in 0..pgltf.bufferview.len(){
      let to = pgltf.bufferview[i].boffset+pgltf.bufferview[i].blenght;
      bfvp.push(rwbf[pgltf.bufferview[i].buffer][pgltf.bufferview[i].boffset..to].to_vec());
    }

    let objvec = Self::perobj(pgltf, bfvp);

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
      //for j in ti..(ti+cl as usize){
      //  chunkd.data.push(rglb[j]);
      //}
      chunkd.data.extend(&rglb[ti..(ti+cl as usize)]);
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


    for i in 0..pgltf.materials.len(){
      let mut data = vec![];
      for j in 0..pgltf.materials[i].texture_indices.len(){
        let view = pgltf.bufferview[pgltf.images[pgltf.textures[pgltf.materials[i].texture_indices[j]].image].buffer_view];
        let rwimg = chunksrd[bini].data[view.boffset..(view.boffset+view.blenght)].to_vec();
        data.push(ImageAsset::other_parse(rwimg));
      }
      matimg.push(data);
    }

    let mut bfvp = vec![];
    for i in 0..pgltf.bufferview.len(){
      let to = pgltf.bufferview[i].boffset+pgltf.bufferview[i].blenght;
      bfvp.push(chunksrd[bini].data[pgltf.bufferview[i].boffset..to].to_vec());
    }

    let objvec = Self::perobj(pgltf, bfvp);

    Glscene { 
      objs: objvec,
      material_uri: vec![],
      material_data: matimg,
      images_bin: true,
    }
  }
  pub fn is_glb(path: &str) -> bool{
    let rglb = fs::read(path).unwrap();
    if rglb[0] == b'g' && rglb[1] == b'l' && rglb[2] == b'T' && rglb[3] == b'F'{
      return true;
    }
    false
  }
}