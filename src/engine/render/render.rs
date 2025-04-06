unsafe extern "C"{
    fn new() -> cty::uint32_t;
    fn loopcont(eh: cty::uint32_t) -> cty::uint32_t;
    fn destroy(eh: cty::uint32_t);
    fn newmaterial(eh: cty::uint32_t, vert: *mut cty::uint32_t, frag: *mut cty::uint32_t, svert: cty::uint32_t, sfrag: cty::uint32_t, cullmode: cty::uint32_t) -> cty::uint32_t;
}


#[derive(Copy, Clone)]
pub struct Render{
    pub euclid: u32,
}

impl Render{
    pub fn new() -> Render{
        Render { 
            euclid: unsafe {
                new()
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