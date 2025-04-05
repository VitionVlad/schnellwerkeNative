unsafe extern "C"{
    fn new() -> cty::c_uint;
    fn loopcont(eh: cty::c_uint) -> cty::c_uint;
    fn destroy(eh: cty::c_uint);
}

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