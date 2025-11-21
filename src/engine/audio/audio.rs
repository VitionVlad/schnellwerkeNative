#![allow(dead_code)]
#![allow(unused_variables)]

use std::ffi::CString;

unsafe extern "C"{
    fn newmozart() -> cty::uint32_t;
    fn mozartsetvolume(mhi: cty::uint32_t, vol: cty::c_float);
    fn newsound(mhi: cty::uint32_t, path: *const cty::c_char) -> cty::uint32_t;
    fn soundplay(msn: cty::uint32_t, pan: cty::c_float, vol: cty::c_float);
    fn soundstop(msn: cty::uint32_t);
    fn soundsetloop(msn: cty::uint32_t, val: cty::uint8_t);
    fn soundsetpos(msn: cty::uint32_t, val: cty::c_float);
    fn destroymozart(mhi: cty::uint32_t);
}

#[derive(Copy, Clone)]
pub struct AudioEngine{
    pub index: u32,
    pub vol: f32,
    pub spacial: bool,
}

impl AudioEngine{
    pub fn new() -> AudioEngine{
        AudioEngine{
            index: unsafe{ newmozart() },
            vol: 1.0f32,
            spacial: true,
        }
    }
    pub fn exec(&mut self){
        unsafe{ mozartsetvolume(self.index, self.vol) };
    }
    pub fn destroy(&mut self){
        unsafe{ destroymozart(self.index) };
    }
}

pub struct Sound{
    index: u32,
    pub vol: f32,
    pub pan: f32,
    pub loopsound: bool,
}

impl Sound{
    pub fn new(ae: AudioEngine, path: &str) -> Sound{
        Sound { 
            index: unsafe {
                newsound(ae.index, CString::new(path).unwrap().as_ptr())
            }, 
            vol: 1.0, 
            pan: 0.0,
            loopsound: true,
        }
    }
    pub fn play(&mut self){
        unsafe {
            soundsetloop(self.index, self.loopsound as u8);
            soundplay(self.index, self.pan, self.vol);
        }
    }
    pub fn set_new_pos(&mut self, newpos: f32){
        unsafe {
            soundsetpos(self.index, newpos);
        }
    }
    pub fn stop(&mut self){
        unsafe {
            soundstop(self.index);
        }
    }
}