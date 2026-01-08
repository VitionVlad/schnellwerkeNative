use crate::engine::math::{ vec3::Vec3, vec4::Vec4 };

pub struct Gobject{
    mesh: usize,
    name: String,
    position: Vec3,
    scale: Vec3,
    rotation: Vec4,
}

pub struct Gmaterial{
    double_sided: bool,
    name: String,
    texture_indices: Vec<usize>,
}

pub struct Gmesh{
    name: String,
    attributes: Vec<(usize, u32)>,
    enable_indices: bool,
    indices: usize,
    material: usize,
}

pub struct Gtexture{
    image: usize,
}

pub struct Gimage{
    image: usize,
}