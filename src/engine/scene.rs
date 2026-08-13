#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::{loader::{glscene::Glscene, imageasset::ImageAsset, rw::checkfs}, math::{mat4::Mat4, vec2::Vec2, vec3::Vec3, vec4::Vec4}, render::render::TextureFormat, voxel::VoxelScene};

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
    pub fn load_from_obj(eng: &mut Engine, path: &str, material: Material, voxelize: bool, voxel_size: f32) -> Scene{
        let mut pakpath = path.to_string();
        let pke = pakpath.len();
        pakpath.remove(pke-1);
        pakpath.remove(pke-2);
        pakpath.remove(pke-3);
        pakpath += "pak3";

        let pak3e = checkfs(&pakpath);
        let mut totvrt = vec![];
        let mut texsv = vec![];
        let mut tricolor = vec![];

        let obj = ModelAsset::load_obj(path);
        let mut mdst: Vec<Model> = vec![];
        let mut mdtx: Vec<Image> = vec![];
        for i in 0..obj.mtl.matinfo.len(){
            mdtx.push(Image::new_from_files(&eng, obj.mtl.matinfo[i].clone()));
        }
        if voxelize && !pak3e{
            let lastelem= obj.mtl.matinfo.len()-1;
            for i in 0..obj.mtl.matinfo.len(){
                //mdtx.push(Image::new_from_files(&eng, obj.mtl.matinfo[i].clone()));
                if i == 0 || lastelem == i{
                    let texdt = ImageAsset::other_load(&obj.mtl.matinfo[i][0]);
                    texsv.push(texdt.clone());
                }
                if i != obj.mtl.matinfo.len()-1 && (obj.mtl.matnam[i] != obj.mtl.matnam[i+1]){
                    let texdt = ImageAsset::other_load(&obj.mtl.matinfo[i+1][0]);
                    texsv.push(texdt.clone());
                }
            }
        }

        for i in 0..obj.vertices.len(){
            mdst.push(Model::new(&eng, obj.vertices[i].clone()));
            if !pak3e && voxelize{
                let mut mat = 0usize;
                for j in 0..mdtx.len(){
                    if obj.mtl.matnam[j] == obj.matnam[i]{
                        mat = j;
                        break;
                    }
                }

                let vlen = (obj.vertices[i].len()/8)*3;
                totvrt.append(&mut obj.vertices[i][0..vlen].to_vec().clone());
                for j in (0..(vlen/3)*2).step_by(6){
                    let uvind = j+vlen;

                    let uv1 = Vec2{ x: obj.vertices[i][uvind], y: obj.vertices[i][uvind+1]};
                    let uv2 = Vec2{ x: obj.vertices[i][uvind+2], y: obj.vertices[i][uvind+3]};
                    let uv3 = Vec2{ x: obj.vertices[i][uvind+4], y: obj.vertices[i][uvind+5]};

                    let mut meduv = Vec2{ x: (uv1.x+uv2.x+uv3.x)/3.0, y: (uv1.y+uv2.y+uv3.y)/3.0};

                    if meduv.x > 1.0 || meduv.x < 0.0{
                        meduv.x -= meduv.x.floor() as f32;
                    }
                    if meduv.y > 1.0 || meduv.y < 0.0{
                        meduv.y -= meduv.y.floor() as f32;
                    }
                    let sz = texsv[mat].size;

                    meduv.x *= sz[0] as f32;
                    meduv.y *= sz[1] as f32;

                    tricolor.push(texsv[mat].data[(meduv.x as usize+meduv.y as usize*sz[0] as usize)*4]);
                    tricolor.push(texsv[mat].data[(meduv.x as usize+meduv.y as usize*sz[0] as usize)*4+1]);
                    tricolor.push(texsv[mat].data[(meduv.x as usize+meduv.y as usize*sz[0] as usize)*4+2]);
                    tricolor.push(texsv[mat].data[(meduv.x as usize+meduv.y as usize*sz[0] as usize)*4+3]);
                }
            }
        }

        let mut vrp = VoxelScene::new_blank();

        if voxelize{
            if pak3e{
                vrp = VoxelScene::from_file(&pakpath);
            }else{
                vrp = VoxelScene::from_vertices(totvrt, voxel_size, tricolor);
                vrp.save_file(&pakpath);
            }
        }

        let mut fobj: Vec<Object> = vec![];
        for i in 0..mdst.len(){
            for j in 0..mdtx.len(){
                if obj.mtl.matnam[j] == obj.matnam[i]{
                    fobj.push(Object::new(eng, mdst[i], material, vec![mdtx[j]], super::render::render::MeshUsage::ShadowAndDefferedPass, true, obj.obn[i].clone()));
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
            voxel_representation: vrp,
        }
    }
    pub fn load_from_gltf(eng: &mut Engine, path: &str, material: Material, voxelize: bool, voxel_size: f32) -> Scene{
        let mut scn = Scene::new_blank();

        let gltfsc;

        let mut ldmt = vec![];
        let mut pakpath = path.to_string();

        let mut texsv = vec![];

        if Glscene::is_glb(path){
            let pke = pakpath.len();
            pakpath.remove(pke-1);
            pakpath.remove(pke-2);
            pakpath.remove(pke-3);
            pakpath.remove(pke-4);
            pakpath += ".pak3";
            let pak3e = checkfs(&pakpath);
            gltfsc = Glscene::readglb(path);

            for i in 0..gltfsc.material_data.len(){
              let mut totdata = vec![];
              for j in 0..gltfsc.material_data[i].len(){
                totdata.extend_from_slice(&gltfsc.material_data[i][j].data);
              }
              if voxelize && !pak3e{
                texsv.push(gltfsc.material_data[i][0].clone());
              }
              ldmt.push(Image::new(eng, [gltfsc.material_data[i][0].size[0], gltfsc.material_data[i][0].size[1], gltfsc.material_data[i].len() as u32], totdata, false, TextureFormat::R8g8b8a8Unorm, true));
            }
        }else{
            let pke = pakpath.len();
            pakpath.remove(pke-1);
            pakpath.remove(pke-2);
            pakpath.remove(pke-3);
            pakpath.remove(pke-4);
            pakpath.remove(pke-5);
            pakpath += ".pak3";
            let pak3e = checkfs(&pakpath);
            gltfsc = Glscene::read_gltf_json(path);

            for i in 0..gltfsc.material_uri.len(){
              let mut totdata = vec![];
              for j in 0..gltfsc.material_uri[i].len(){
                totdata.push(gltfsc.material_uri[i][j].clone());
              }
              if voxelize && !pak3e{
                let texdt = ImageAsset::other_load(&gltfsc.material_uri[i][0]);
                texsv.push(texdt.clone());
              }
              ldmt.push(Image::new_from_files(&eng, totdata)); 
            }
        }

        //println!("pak path: {}", pakpath);
        let pak3e = checkfs(&pakpath);
        //println!("pak check result: {}", pak3e);

        let mut totvrt = vec![];
        let mut tricolor = vec![];

        for i in 0..gltfsc.objs.len(){
            let tobj = Model::new(eng, gltfsc.objs[i].vertices.clone());
            scn.objects.push(Object::new(eng, tobj, material, vec![ldmt[gltfsc.objs[i].material]], super::render::render::MeshUsage::ShadowAndDefferedPass, true, gltfsc.objs[i].name.clone()));
            let lobj = scn.objects.len()-1;
            scn.objects[lobj].physic_object.pos = gltfsc.objs[i].position;
            scn.objects[lobj].physic_object.scale = gltfsc.objs[i].scale;
            scn.objects[lobj].physic_object.rot = gltfsc.objs[i].rot;

            if !pak3e && voxelize{
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
                for j in (0..(reqlen/3)*2).step_by(6){
                    let uvind = j+reqlen;

                    let uv1 = Vec2{ x: gltfsc.objs[i].vertices[uvind], y: gltfsc.objs[i].vertices[uvind+1]};
                    let uv2 = Vec2{ x: gltfsc.objs[i].vertices[uvind+2], y: gltfsc.objs[i].vertices[uvind+3]};
                    let uv3 = Vec2{ x: gltfsc.objs[i].vertices[uvind+4], y: gltfsc.objs[i].vertices[uvind+5]};

                    let mut meduv = Vec2{ x: (uv1.x+uv2.x+uv3.x)/3.0, y: (uv1.y+uv2.y+uv3.y)/3.0};

                    if meduv.x > 1.0 || meduv.x < 0.0{
                        meduv.x -= meduv.x.floor() as f32;
                    }
                    if meduv.y > 1.0 || meduv.y < 0.0{
                        meduv.y -= meduv.y.floor() as f32;
                    }
                    let sz = texsv[gltfsc.objs[i].material].size;

                    meduv.x *= sz[0] as f32;
                    meduv.y *= sz[1] as f32;

                    let mut imed = [meduv.x as u32, meduv.y as u32];

                    if imed[0] >= sz[0]{
                        imed[0] = sz[0] - 1;
                    }

                    if imed[1] >= sz[1]{
                        imed[1] = sz[1] - 1;
                    }

                    tricolor.push(texsv[gltfsc.objs[i].material].data[(imed[0] as usize+imed[1] as usize*sz[0] as usize)*4]);
                    tricolor.push(texsv[gltfsc.objs[i].material].data[(imed[0] as usize+imed[1] as usize*sz[0] as usize)*4+1]);
                    tricolor.push(texsv[gltfsc.objs[i].material].data[(imed[0] as usize+imed[1] as usize*sz[0] as usize)*4+2]);
                    tricolor.push(texsv[gltfsc.objs[i].material].data[(imed[0] as usize+imed[1] as usize*sz[0] as usize)*4+3]);
                }
            }
        }
        println!("total vertices: {}, total colors: {}", totvrt.len(), tricolor.len());

        //let bd = VoxelScene::get_boundaries(totvrt.clone());
        if voxelize{
            if pak3e{
                scn.voxel_representation = VoxelScene::from_file(&pakpath);
            }else{
                scn.voxel_representation = VoxelScene::from_vertices_svo(totvrt.clone(), tricolor, 5);
                scn.voxel_representation.save_file(&pakpath);
            }
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