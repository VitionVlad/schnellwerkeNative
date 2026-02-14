#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::{loader::glscene::Glscene, math::vec3::Vec3};

use super::{engine::Engine, image::Image, loader::modelasset::ModelAsset, material::Material, model::Model, object::Object};

pub struct Scene{
    pub objects: Vec<Object>,
    pub images: Vec<Image>,
    pub models: Vec<Model>,
    pub use_global_values: bool,
    pub pos: Vec3,
    pub scale: Vec3,
    pub rot: Vec3,
    pub render_all_cameras: bool,
    pub exclude_selected_camera: bool,
    pub camera_number: i8,
}

impl Scene{
    pub fn new_blank() -> Scene{
        Scene { 
            objects: vec![],
            images: vec![], 
            models: vec![], 
            use_global_values: false, 
            pos: Vec3::new(), 
            scale: Vec3::new(), 
            rot: Vec3::new(), 
            render_all_cameras: true, 
            exclude_selected_camera: false, 
            camera_number: 0 
        }
    }
    pub fn load_from_obj(eng: &mut Engine, path: &str, material: Material) -> Scene{
        let obj = ModelAsset::load_obj(path);
        let mut mdst: Vec<Model> = vec![];
        let mut mdtx: Vec<Image> = vec![];
        for i in 0..obj.mtl.matinfo.len(){
            mdtx.push(Image::new_from_files(&eng, obj.mtl.matinfo[i].clone()));
        }
        for i in 0..obj.vertices.len(){
            mdst.push(Model::new(&eng, obj.vertices[i].clone()));
        }
        let mut fobj: Vec<Object> = vec![];
        for i in 0..mdst.len(){
            for j in 0..mdtx.len(){
                if obj.mtl.matnam[j] == obj.matnam[i]{
                    fobj.push(Object::new(eng, mdst[i], material, mdtx[j], super::render::render::MeshUsage::ShadowAndDefferedPass, true, obj.obn[i].clone()));
                    break;
                }
            }
        }
        Scene { 
            objects: fobj,
            images: mdtx,
            models: mdst,
            use_global_values: true,
            pos: Vec3::new(),
            rot: Vec3::new(),
            scale: Vec3{ x: 1.0f32, y: 1.0f32, z: 1.0f32},
            render_all_cameras: true,
            exclude_selected_camera: false,
            camera_number: 0,
        }
    }
    pub fn load_from_gltf(eng: &mut Engine, path: &str, material: Material) -> Scene{
        let mut scn = Scene::new_blank();

        let gltfsc;

        let mut ldmt = vec![];

        if Glscene::is_glb(path){
            gltfsc = Glscene::readglb(path);

            for i in 0..gltfsc.material_data.len(){
              let mut totdata = vec![];
              for j in 0..gltfsc.material_data[i].len(){
                totdata.extend_from_slice(&gltfsc.material_data[i][j].data);
              }
              ldmt.push(Image::new(eng, [gltfsc.material_data[i][0].size[0], gltfsc.material_data[i][0].size[1], gltfsc.material_data[i].len() as u32], totdata));
            }
        }else{
            gltfsc = Glscene::read_gltf_json(path);

            for i in 0..gltfsc.material_uri.len(){
              let mut totdata = vec![];
              for j in 0..gltfsc.material_uri[i].len(){
                totdata.push(gltfsc.material_uri[i][j].clone());
              }
              ldmt.push(Image::new_from_files(&eng, totdata));
            }
        }

        for i in 0..gltfsc.objs.len(){
          let tobj = Model::new(eng, gltfsc.objs[i].vertices.clone());
          scn.objects.push(Object::new(eng, tobj, material, ldmt[gltfsc.objs[i].material], super::render::render::MeshUsage::ShadowAndDefferedPass, true, gltfsc.objs[i].name.clone()));
          let lobj = scn.objects.len()-1;
          scn.objects[lobj].physic_object.pos = gltfsc.objs[i].position;
          scn.objects[lobj].physic_object.scale = gltfsc.objs[i].scale;
          scn.objects[lobj].physic_object.rot = gltfsc.objs[i].rot;
        }

        scn.use_global_values = false;
        scn
    }
    pub fn exec(&mut self, eng: &mut Engine){
        for i in 0..self.objects.len(){
            if self.use_global_values{
                self.objects[i].physic_object.pos += self.pos;
                self.objects[i].physic_object.rot += self.rot;
                self.objects[i].physic_object.scale += self.scale;
                self.objects[i].mesh.render_all_cameras = self.render_all_cameras;
                self.objects[i].mesh.exclude_selected_camera = self.exclude_selected_camera;
                self.objects[i].mesh.camera_number = self.camera_number;
            }
            self.objects[i].exec(eng);
        }
    }
}