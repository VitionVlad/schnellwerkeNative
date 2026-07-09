#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::{loader::{glscene::Glscene, rw::checkfs}, math::{mat4::Mat4, vec3::Vec3, vec4::Vec4}, render::render::TextureFormat, voxel::VoxelScene};

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
    pub voxel_representation: VoxelScene,
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
            camera_number: 0,
            voxel_representation: VoxelScene::new_blank(),
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
            voxel_representation: VoxelScene::new_blank(),
        }
    }
    pub fn load_from_gltf(eng: &mut Engine, path: &str, material: Material, voxelize: bool) -> Scene{
        let mut scn = Scene::new_blank();

        let gltfsc;

        let mut ldmt = vec![];
        let mut pakpath = path.to_string();

        if Glscene::is_glb(path){
            let pke = pakpath.len();
            pakpath.remove(pke-1);
            pakpath.remove(pke-2);
            pakpath.remove(pke-3);
            pakpath.remove(pke-4);
            gltfsc = Glscene::readglb(path);

            for i in 0..gltfsc.material_data.len(){
              let mut totdata = vec![];
              for j in 0..gltfsc.material_data[i].len(){
                totdata.extend_from_slice(&gltfsc.material_data[i][j].data);
              }
              ldmt.push(Image::new(eng, [gltfsc.material_data[i][0].size[0], gltfsc.material_data[i][0].size[1], gltfsc.material_data[i].len() as u32], totdata, false, TextureFormat::R8g8b8a8Unorm));
            }
        }else{
            let pke = pakpath.len();
            pakpath.remove(pke-1);
            pakpath.remove(pke-2);
            pakpath.remove(pke-3);
            pakpath.remove(pke-4);
            pakpath.remove(pke-5);
            gltfsc = Glscene::read_gltf_json(path);

            for i in 0..gltfsc.material_uri.len(){
              let mut totdata = vec![];
              for j in 0..gltfsc.material_uri[i].len(){
                totdata.push(gltfsc.material_uri[i][j].clone());
              }
              ldmt.push(Image::new_from_files(&eng, totdata));
            }
        }

        pakpath += ".pak3";
        //println!("pak path: {}", pakpath);
        let pak3e = checkfs(&pakpath);
        //println!("pak check result: {}", pak3e);

        let mut totvrt = vec![];

        for i in 0..gltfsc.objs.len(){
            let tobj = Model::new(eng, gltfsc.objs[i].vertices.clone());
            scn.objects.push(Object::new(eng, tobj, material, ldmt[gltfsc.objs[i].material], super::render::render::MeshUsage::ShadowAndDefferedPass, true, gltfsc.objs[i].name.clone()));
            let lobj = scn.objects.len()-1;
            scn.objects[lobj].physic_object.pos = gltfsc.objs[i].position;
            scn.objects[lobj].physic_object.scale = gltfsc.objs[i].scale;
            scn.objects[lobj].physic_object.rot = gltfsc.objs[i].rot;

            if !pak3e{
                let reqlen = (gltfsc.objs[i].vertices.len()/8)*3;

                for j in (0..reqlen).step_by(3){
                    let v4 = Vec4{ x: gltfsc.objs[i].vertices[j], y: gltfsc.objs[i].vertices[j+1], z: gltfsc.objs[i].vertices[j+2], w: 1.0};

                    let mut lubm;
                    let mut ubm = Mat4::new();
                    ubm.trans(scn.objects[lobj].physic_object.pos);
                    lubm = ubm.clone();
                
                    let mut t: Mat4 = Mat4::new();
                    ubm = Mat4::new();
                    ubm.xrot(scn.objects[lobj].physic_object.rot.x);
                    t.yrot(scn.objects[lobj].physic_object.rot.y);
                    ubm *= t;
                    t = Mat4::new();
                    t.zrot(scn.objects[lobj].physic_object.rot.z);
                    ubm *= t;
                
                    lubm *= ubm;
                
                    ubm = Mat4::new();
                    ubm.scale(scn.objects[lobj].physic_object.scale);
                
                    lubm *= ubm;

                    let res = lubm.vec4mul(v4);

                    totvrt.append(&mut vec![res.x, res.y, res.z]);
                }
            }
        }

        //let bd = VoxelScene::get_boundaries(totvrt.clone());
        if pak3e{
            scn.voxel_representation = VoxelScene::from_file(&pakpath);
        }else{
            scn.voxel_representation = VoxelScene::from_vertices(totvrt, 1.0);
            scn.voxel_representation.save_file(&pakpath);
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