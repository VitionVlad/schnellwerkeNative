use crate::engine::math::{ vec3::Vec3, vec4::Vec4 };

pub struct Gobject{
    pub mesh: usize,
    pub name: String,
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec4,
}

pub struct Gmaterial{
    pub double_sided: bool,
    pub name: String,
    pub texture_indices: Vec<usize>,
}

pub struct Gmesh{
    pub name: String,
    pub attributes: Vec<(usize, u32)>,
    pub enable_indices: bool,
    pub indices: usize,
    pub material: usize,
}

pub struct Gtexture{
    pub image: usize,
}

pub struct Gimage{
    pub name: String,
    pub uri: String,
}

pub struct Gacc{
    pub bufferviwe: usize,
    pub component_type: u32,
    pub count: usize,
    pub tp: String
}

pub struct Gbfv{
    pub buffer: usize,
    pub blenght: usize,
    pub boffset: usize,
    pub target: usize,
}

pub struct Gbf{
    pub bl: usize,
    pub uri: String,
}

pub struct Scene{
    pub nodes: Vec<usize>,
    pub objects: Vec<Gobject>,
    pub materials: Vec<Gmaterial>,
    pub meshes: Vec<Gmesh>,
    pub textures: Vec<Gtexture>,
    pub images: Vec<Gimage>,
    pub accesories: Vec<Gacc>,
    pub bufferview: Vec<Gbfv>,
    pub buffers: Vec<Gbf>
}

pub struct Gltf{
    pub scenes: Vec<Scene>,
}

impl Gltf {
    //pub fn parse()
}