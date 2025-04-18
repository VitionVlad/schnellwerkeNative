#![allow(dead_code)]
#![allow(unused_variables)]

unsafe extern "C"{
    fn neweng(shadowMapResolution: cty::uint32_t) -> cty::uint32_t;
    fn destroy(eh: cty::uint32_t);
    fn newmaterial(eh: cty::uint32_t, vert: *mut cty::uint32_t, frag: *mut cty::uint32_t, svert: cty::uint32_t, sfrag: cty::uint32_t, cullmode: cty::uint32_t) -> cty::uint32_t;
    fn newmodel(eh: cty::uint32_t, vert: *mut cty::c_float, uv: *mut cty::c_float, normals: *mut cty::c_float, size: cty::uint32_t) -> cty::uint32_t;
    fn setmeshbuf(eme: cty::uint32_t, i: cty::uint32_t, val: cty::c_float);
    fn newmesh(eh: cty::uint32_t, es: cty::uint32_t, em: cty::uint32_t, te: cty::uint32_t) -> cty::uint32_t;
    fn newtexture(eh: cty::uint32_t, xsize: cty::uint32_t, ysize: cty::uint32_t, zsize: cty::uint32_t, pixels: *mut cty::c_char) -> cty::uint32_t;
    fn loopcont(eh: cty::uint32_t) -> cty::uint32_t;
}

#[derive(Copy, Clone)]
pub struct Render{
    pub euclid: u32,
}

impl Render{
    pub fn new() -> Render{
        Render { 
            euclid: unsafe {
                neweng(1000)
            } 
        }
    }
    pub fn continue_loop(&self) -> bool{
        return unsafe { loopcont(self.euclid) } == 1;
    }
    pub fn destroy(&self){
        unsafe{
            destroy(self.euclid);
        }
    }
}

#[derive(Copy, Clone)]
pub enum CullMode {
    CullModeNone = 0,
    CullModeFrontBit = 0x00000001,
    CullModeBackBit = 0x00000002,
    CullModeFrontAndBack = 0x00000003,
}

#[derive(Copy, Clone)]
pub struct MaterialShaders{
    pub materialid: u32,
}

impl MaterialShaders{
    pub fn new(ren: Render, vert: Vec<u8>, frag: Vec<u8>, cullmode: CullMode) -> MaterialShaders{
        MaterialShaders { 
            materialid: unsafe{
                newmaterial(ren.euclid, vert.as_ptr() as *mut u32, frag.as_ptr() as *mut u32, vert.len() as u32, frag.len() as u32, cullmode as u32)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertexes{
    pub modelid: u32,
}

impl Vertexes{
    pub fn new(ren: Render, vertices: Vec<f32>) -> Vertexes{
        let size = vertices.len()/8;
        let mut v: Vec<f32> = vec![];
        let mut u: Vec<f32> = vec![];
        let mut n: Vec<f32> = vec![];
        for i in 0..size*3 {
            v.push(vertices[i]);
        }
        for i in 0..size*2 {
            u.push(vertices[i+size*3]);
        }
        for i in 0..size*3 {
            n.push(vertices[i+size*5]);
        }
        Vertexes { 
            modelid: unsafe{
                newmodel(ren.euclid, v.as_ptr() as *mut f32, u.as_ptr() as *mut f32, n.as_ptr() as *mut f32, size as u32)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Texture{
    pub texid: u32,
}

impl Texture {
    pub fn new(render: Render, xs: u32, ys: u32, texnm: u32, data: Vec<i8>) -> Texture{
        Texture { 
            texid: unsafe {
                newtexture(render.euclid, xs, ys, texnm, data.as_ptr() as *mut i8)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Mesh{
    pub meshid: u32,
    pub ubo: [f32; 20],
}

impl Mesh{
    pub fn new(ren: Render, model: Vertexes, material: MaterialShaders, texture: Texture) -> Mesh{
        Mesh { 
            meshid: unsafe{
                newmesh(ren.euclid, material.materialid, model.modelid, texture.texid)
            },
            ubo: [1.0; 20]
        }
    }

    pub fn exec(&self){
        for i in 0..20{
            unsafe {
                setmeshbuf(self.meshid, i, self.ubo[i as usize])
            };
        }
    }
}