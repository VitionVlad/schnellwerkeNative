use crate::engine::{loader::jsonparser::JsonF, math::{ vec3::Vec3, vec4::Vec4 }};

#[derive(Clone)]
pub enum GLtypes{
    SignedByte = 5120,	
    UnsignedByte = 5121,	
    SignedShort = 5122,
    UnsignedShort = 5123,	
    UnsignedInt = 5125,
    Float = 5126,
}

#[derive(Clone)]
pub struct Gobject{
    pub mesh: usize,
    pub name: String,
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec4,
}

#[derive(Clone)]
pub struct Gmaterial{
    pub double_sided: bool,
    pub name: String,
    pub texture_indices: Vec<usize>,
}

#[derive(Clone)]
pub struct Gmesh{
    pub name: String,
    pub attributes: Vec<usize>,
    pub attributesu: Vec<String>,
    pub enable_indices: bool,
    pub indices: usize,
    pub material: usize,
}

#[derive(Copy, Clone)]
pub struct Gtexture{
    pub image: usize,
}

#[derive(Clone)]
pub struct Gimage{
    pub name: String,
    pub tip: String,
    pub uri: String,
}

#[derive(Clone)]
pub struct Gacc{
    pub bufferview: usize,
    pub component_type: u32,
    pub count: usize,
    pub tp: String
}

#[derive(Copy, Clone)]
pub struct Gbfv{
    pub buffer: usize,
    pub blenght: usize,
    pub boffset: usize,
    pub target: usize,
}

#[derive(Clone)]
pub struct Gbf{
    pub bl: usize,
    pub uri: String,
}

#[derive(Clone)]
pub struct Gscene{
    pub name: String,
    pub nodes: Vec<usize>,
}

#[derive(Clone)]
pub struct Gltf{
    pub scene: usize,
    pub scenes: Vec<Gscene>,
    pub objects: Vec<Gobject>,
    pub materials: Vec<Gmaterial>,
    pub meshes: Vec<Gmesh>,
    pub textures: Vec<Gtexture>,
    pub images: Vec<Gimage>,
    pub accesories: Vec<Gacc>,
    pub bufferview: Vec<Gbfv>,
    pub buffers: Vec<Gbf>
}

impl Gltf {
    pub fn parse_gltf(json: JsonF) -> Gltf{
        let mut lgltf = Gltf{ 
            scene: 0, 
            scenes: vec![],
            objects: vec![],
            materials: vec![],
            meshes: vec![],
            textures: vec![],
            images: vec![],
            accesories: vec![],
            bufferview: vec![],
            buffers: vec![]
        };
        for i in 0..json.other_nodes.len(){
            if json.other_nodes[i].name == "scene"{
                lgltf.scene = json.other_nodes[i].numeral_val as usize;
            }
            else if json.other_nodes[i].name == "scenes"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut name = "";
                    let mut nodes = vec![];
                    if json.other_nodes[i].other_nodes[j].name == "name"{
                        name = &json.other_nodes[i].other_nodes[j].strval;
                    }
                    else if json.other_nodes[i].other_nodes[j].name == "nodes"{
                        for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                            nodes.push(json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize);
                        }
                    }
                    lgltf.scenes.push(Gscene{
                        name: name.to_string(),
                        nodes: nodes,
                    });
                }
            }else if json.other_nodes[i].name == "nodes"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut msg = Gobject{ name: "".to_string(), mesh: 0, position: Vec3::new(), scale: Vec3::new(), rotation: Vec4::new() };
                    for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                        let fname = json.other_nodes[i].other_nodes[j].other_nodes[l].name.clone();
                        if fname == "mesh"{
                            msg.mesh = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }else if fname == "name"{
                            msg.name = json.other_nodes[i].other_nodes[j].other_nodes[l].strval.clone();
                        }else if fname == "rotation"{
                            msg.rotation.x = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].numeral_val;
                            msg.rotation.y = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[1].numeral_val;
                            msg.rotation.z = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[2].numeral_val;
                            msg.rotation.w = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[3].numeral_val;
                        }else if fname == "scale"{
                            msg.scale.x = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].numeral_val;
                            msg.scale.y = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[1].numeral_val;
                            msg.scale.z = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[2].numeral_val;
                        }else if fname == "translation"{
                            msg.position.x = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].numeral_val;
                            msg.position.y = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[1].numeral_val;
                            msg.position.z = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[2].numeral_val;
                        }
                    }
                    lgltf.objects.push(msg);
                }
            }else if json.other_nodes[i].name == "materials"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut msg = Gmaterial{ double_sided: false, name: "".to_string(), texture_indices: vec![] };
                    for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                        let fname = json.other_nodes[i].other_nodes[j].other_nodes[l].name.clone();
                        if fname == "doubleSided"{
                            msg.double_sided = json.other_nodes[i].other_nodes[j].other_nodes[l].bolean;
                        }else if fname == "name"{
                            msg.name = json.other_nodes[i].other_nodes[j].other_nodes[l].strval.clone();
                        }else if fname == "pbrMetallicRoughness"{
                            for h in 0..json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes.len(){
                                let lfname = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[h].name.clone();
                                if lfname == "baseColorTexture" || lfname == "metallicRoughnessTexture" {
                                    for p in 0..json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[h].other_nodes.len(){
                                        if json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[h].other_nodes[p].name == "index"{
                                            msg.texture_indices.push(json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[h].other_nodes[p].numeral_val as usize);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    lgltf.materials.push(msg);
                }
            }else if json.other_nodes[i].name == "meshes"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut msg = Gmesh{ name: "".to_string(), attributes: vec![], attributesu: vec![], enable_indices: false, indices: 0, material: 0 };
                    for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                        let fname = json.other_nodes[i].other_nodes[j].other_nodes[l].name.clone();
                        if fname == "primitives"{
                            for k in 0..json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].other_nodes.len(){
                                let kfname = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].other_nodes[k].name.clone();
                                if kfname == "indices"{
                                    msg.indices = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].other_nodes[k].numeral_val as usize;
                                    msg.enable_indices = true;
                                }else if kfname == "material"{
                                    msg.material = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].other_nodes[k].numeral_val as usize;
                                }else if kfname == "attributes"{
                                    for p in 0..json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].other_nodes[k].other_nodes.len(){
                                        let atn = json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].other_nodes[k].other_nodes[p].name.clone();
                                        msg.attributes.push(json.other_nodes[i].other_nodes[j].other_nodes[l].other_nodes[0].other_nodes[k].other_nodes[p].numeral_val as usize);
                                        msg.attributesu.push(atn);
                                    }
                                }
                            }
                        }else if fname == "name"{
                            msg.name = json.other_nodes[i].other_nodes[j].other_nodes[l].strval.clone();
                        }
                    }
                    lgltf.meshes.push(msg);
                }
            }else if json.other_nodes[i].name == "textures"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut msg = Gtexture{ image: 0 };
                    for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                        let fname = json.other_nodes[i].other_nodes[j].other_nodes[l].name.clone();
                        if fname == "source"{
                            msg.image = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }
                    }
                    lgltf.textures.push(msg);
                }
            }else if json.other_nodes[i].name == "images"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut msg = Gimage{ name: "".to_string(), tip: "".to_string(), uri: "".to_string() };
                    for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                        let fname = json.other_nodes[i].other_nodes[j].other_nodes[l].name.clone();
                        if fname == "mimeType"{
                            msg.name = json.other_nodes[i].other_nodes[j].other_nodes[l].strval.clone();
                        }else if fname == "name"{
                            msg.tip = json.other_nodes[i].other_nodes[j].other_nodes[l].strval.clone();
                        }else if fname == "uri"{
                            msg.uri = json.other_nodes[i].other_nodes[j].other_nodes[l].strval.clone();
                        }
                    }
                    lgltf.images.push(msg);
                }
            }else if json.other_nodes[i].name == "accessors"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut msg = Gacc{ bufferview: 0, component_type: 0, count: 0, tp: "".to_string() };
                    for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                        let fname = json.other_nodes[i].other_nodes[j].other_nodes[l].name.clone();
                        if fname == "bufferView"{
                            msg.bufferview = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }else if fname == "componentType"{
                            msg.component_type = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as u32;
                        }else if fname == "count"{
                            msg.count = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }else if fname == "type"{
                            msg.tp = json.other_nodes[i].other_nodes[j].other_nodes[l].strval.clone();
                        }
                    }
                    lgltf.accesories.push(msg);
                }
            }else if json.other_nodes[i].name == "bufferViews"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut msg = Gbfv{ buffer: 0, blenght: 0, boffset: 0, target: 0 };
                    for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                        let fname = json.other_nodes[i].other_nodes[j].other_nodes[l].name.clone();
                        if fname == "buffer"{
                            msg.buffer = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }else if fname == "byteLength"{
                            msg.blenght = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }else if fname == "byteOffset"{
                            msg.boffset = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }else if fname == "target"{
                            msg.target = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }
                    }
                    lgltf.bufferview.push(msg);
                }
            }else if json.other_nodes[i].name == "buffers"{
                for j in 0..json.other_nodes[i].other_nodes.len(){
                    let mut msg = Gbf{ bl: 0, uri: "".to_string() };
                    for l in 0..json.other_nodes[i].other_nodes[j].other_nodes.len(){
                        let fname = json.other_nodes[i].other_nodes[j].other_nodes[l].name.clone();
                        if fname == "byteLength"{
                            msg.bl = json.other_nodes[i].other_nodes[j].other_nodes[l].numeral_val as usize;
                        }else if fname == "uri"{
                            msg.uri = json.other_nodes[i].other_nodes[j].other_nodes[l].strval.clone();
                        }
                    }
                    lgltf.buffers.push(msg);
                }
            }
        }
        lgltf
    }
}