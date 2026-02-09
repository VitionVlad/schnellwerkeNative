#![allow(dead_code)]
#![allow(unused_variables)]

use std::ffi::CString;

use cty::{uint8_t, uint32_t};

unsafe extern "C"{
    fn get_frametime(eh: cty::uint32_t) -> cty::c_float;
    fn get_resx(eh: cty::uint32_t) -> cty::uint32_t;
    fn get_resy(eh: cty::uint32_t) -> cty::uint32_t;
    fn setresolution(eh: cty::uint32_t, xs: cty::uint32_t, ys: cty::uint32_t);
    fn seticon(eh: cty::uint32_t, xs: cty::uint32_t, ys: cty::uint32_t, pixels: *mut cty::c_char);
    fn settitle(eh: cty::uint32_t, title: *const cty::c_char);
    fn setfullscreen(eh: cty::uint32_t);
    fn quitfullscreen(eh: cty::uint32_t);
    fn getKeyPressed(eh: cty::uint32_t, index: cty::uint32_t) -> cty::uint8_t;
    fn getmr(eh: cty::uint32_t) -> cty::uint8_t;
    fn getml(eh: cty::uint32_t) -> cty::uint8_t;
    fn getmm(eh: cty::uint32_t) -> cty::uint8_t;
    fn get_mouse_posx(eh: cty::uint32_t)  -> cty::c_double;
    fn get_mouse_posy(eh: cty::uint32_t)  -> cty::c_double;
    fn req_mouse_lock(eh: cty::uint32_t);
    fn req_mouse_unlock(eh: cty::uint32_t);
    fn get_axis(eh: cty::uint32_t, n: cty::uint8_t) -> cty::c_float;
    fn get_button(eh: cty::uint32_t, n: cty::uint8_t) -> cty::c_uchar;
    fn gamepad_con(eh: cty::uint32_t) -> cty::uint8_t;
    fn gamepad_axisnm(eh: cty::uint32_t) -> cty::uint8_t;
    fn gamepad_buttonnm(eh: cty::uint32_t) -> cty::uint8_t;
    fn modifyshadowdata(eh: cty::uint32_t, ncnt: cty::uint32_t, nres: cty::uint32_t);
    fn modifydeffereddata(eh: cty::uint32_t, ncnt: cty::uint32_t, nres: cty::c_float);
    fn modifyshadowuniform(eh: cty::uint32_t, pos: cty::uint32_t, value: cty::c_float);
    fn modifydeffereduniform(eh: cty::uint32_t, pos: cty::uint32_t, value: cty::c_float);
    fn neweng(shadowMapResolution: cty::uint32_t) -> cty::uint32_t;
    fn destroy(eh: cty::uint32_t);
    fn newmaterial(eh: cty::uint32_t, vert: *mut cty::uint32_t, frag: *mut cty::uint32_t, shadow: *mut cty::uint32_t, svert: cty::uint32_t, sfrag: cty::uint32_t, sshadow: cty::uint32_t, cullmode: cty::uint32_t, scullmode: cty::uint32_t) -> cty::uint32_t;
    fn newmodel(eh: cty::uint32_t, vert: *mut cty::c_float, uv: *mut cty::c_float, normals: *mut cty::c_float, size: cty::uint32_t) -> cty::uint32_t;
    fn setrendercamera(eme: cty::uint32_t, val: cty::int8_t);
    fn setmeshbuf(eme: cty::uint32_t, i: cty::uint32_t, val: cty::c_float);
    fn setdrawable(eme: cty::uint32_t, val: cty::uint8_t);
    fn newmesh(eh: cty::uint32_t, es: cty::uint32_t, em: cty::uint32_t, te: cty::uint32_t, usage: cty::uint32_t) -> cty::uint32_t;
    fn newtexture(eh: cty::uint32_t, xsize: cty::uint32_t, ysize: cty::uint32_t, zsize: cty::uint32_t, pixels: *mut cty::c_char) -> cty::uint32_t;
    fn loopcont(eh: cty::uint32_t) -> cty::uint32_t;
}

#[derive(Copy, Clone)]
pub struct Render{
    pub euclid: u32,
    pub shadow_map_resolution: u32,
    pub shadow_map_count: u32,
    pub camera_count: u32,
    pub resolution_scale: f32,
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub fullscreen: bool,
    pub frametime: f32,
    fullscreeno: bool,
}

impl Render{
    pub fn new() -> Render{
        Render { 
            euclid: unsafe {
                neweng(1000)
            },
            shadow_map_count: 1,
            shadow_map_resolution: 1000,
            camera_count: 1,
            resolution_scale: 1f32,
            resolution_x: 800,
            resolution_y: 600,
            fullscreen: false,
            fullscreeno: false,
            frametime: 0.0,
        }
    }
    pub fn continue_loop(&mut self) -> bool{
        unsafe{ 
            self.resolution_x = get_resx(self.euclid);
            self.resolution_y = get_resy(self.euclid);
            if self.fullscreen != self.fullscreeno {
                match self.fullscreen{
                    true => setfullscreen(self.euclid),
                    false => quitfullscreen(self.euclid),
                }
                self.fullscreeno = self.fullscreen;
            }
            modifyshadowdata(self.euclid, self.shadow_map_count, self.shadow_map_resolution);
            modifydeffereddata(self.euclid, self.camera_count, self.resolution_scale);
            self.frametime = get_frametime(self.euclid)
        };
        return unsafe { loopcont(self.euclid) } == 1;
    }
    pub fn set_shadow_uniform_data(&self, i: u32, value: f32){
        unsafe{ modifyshadowuniform(self.euclid, i, value); }
    }
    pub fn set_deffered_uniform_data(&self, i: u32, value: f32){
        unsafe{ modifydeffereduniform(self.euclid, i, value); }
    }
    pub fn set_new_resolution(&self, resx: u32, resy: u32){
        unsafe { setresolution(self.euclid, resx, resy); }
    }
    pub fn set_icon(&mut self, resx: u32, resy: u32, data: Vec<i8>){
        unsafe { seticon(self.euclid, resx, resy, data.as_ptr() as *mut i8) }
    }
    pub fn set_title(&self, title: &str){
        unsafe { settitle(self.euclid, CString::new(title).unwrap().as_ptr()); }
    }
    pub fn destroy(&self){
        unsafe{
            destroy(self.euclid);
        }
    }
}

#[derive(Copy, Clone)]
pub struct Control{
    euclid: u32,
    pub xpos: f64,
    pub ypos: f64,
    pub mouse_lock: bool,
    old_mouse_lock: bool,
    pub mousebtn: [bool; 3],
    pub gamepad_connected: bool,
    pub gamepad_axis_count: u8,
    pub gamepad_button_count: u8,
}

impl Control{
    pub fn new(render: Render) -> Control{
        Control {
            euclid: render.euclid,
            xpos: 0.0f64,
            ypos: 0.0f64,
            mouse_lock: false,
            old_mouse_lock: false,
            mousebtn: [false, false, false],
            gamepad_connected: false,
            gamepad_axis_count: 0,
            gamepad_button_count: 0,
        }
    }
    pub fn get_key_state(&self, keyindex: uint32_t) -> bool{
        return unsafe { getKeyPressed(self.euclid, keyindex) != 0 };
    }
    pub fn get_gamepad_button_state(&self, button_index: uint8_t) -> bool{
        return unsafe { get_button(self.euclid, button_index) != 0 };
    }
    pub fn get_gamepad_axis_state(&self, axis_index: uint8_t) -> f32{
        return unsafe { get_axis(self.euclid, axis_index) };
    }
    pub fn get_mouse_pos(&mut self){
        self.gamepad_connected = unsafe{gamepad_con(self.euclid)} == 1;
        self.gamepad_axis_count = unsafe{gamepad_axisnm(self.euclid)};
        self.gamepad_button_count = unsafe{gamepad_buttonnm(self.euclid)};
        if self.mouse_lock != self.old_mouse_lock{
            match self.mouse_lock {
                true => unsafe { req_mouse_lock(self.euclid) },
                false => unsafe { req_mouse_unlock(self.euclid) },
            }
            self.old_mouse_lock = self.mouse_lock;
        }
        self.xpos = unsafe{ get_mouse_posx(self.euclid) };
        self.ypos = unsafe{ get_mouse_posy(self.euclid) };
        self.mousebtn = [ unsafe { getmr(self.euclid) } == 1, unsafe { getmm(self.euclid) } == 1, unsafe { getml(self.euclid) } == 1];
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
    pub fn new(ren: Render, vert: Vec<u8>, frag: Vec<u8>, shadow: Vec<u8>, cullmode: CullMode, shadow_cullmode: CullMode) -> MaterialShaders{
        MaterialShaders { 
            materialid: unsafe{
                newmaterial(ren.euclid, vert.as_ptr() as *mut u32, frag.as_ptr() as *mut u32, shadow.as_ptr() as *mut u32, vert.len() as u32, frag.len() as u32, shadow.len() as u32, cullmode as u32, shadow_cullmode as u32)
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
    pub fn new(render: Render, xs: u32, ys: u32, texnm: u32, data: Vec<u8>) -> Texture{
        Texture { 
            texid: unsafe {
                newtexture(render.euclid, xs, ys, texnm, data.as_ptr() as *mut i8)
            }
        }
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum MeshUsage {
    LightingPass = 0,
    DefferedPass = 1,
    ShadowPass = 2,
    ShadowAndDefferedPass = 3,
}

#[derive(Copy, Clone)]
pub struct Mesh{
    pub meshid: u32,
    pub ubo: [f32; 52],
    pub draw: bool,
    pub draw_shadow: bool,
    pub keep_shadow: bool,
    pub render_all_cameras: bool,
    pub exclude_selected_camera: bool,
    pub camera_number: i8,
}

impl Mesh{
    pub fn new(ren: Render, model: Vertexes, material: MaterialShaders, texture: Texture, usage: MeshUsage) -> Mesh{
        Mesh { 
            meshid: unsafe{
                newmesh(ren.euclid, material.materialid, model.modelid, texture.texid,usage as u32)
            },
            ubo: [1.0; 52],
            draw: true,
            draw_shadow: true,
            keep_shadow: true,
            render_all_cameras: true,
            exclude_selected_camera: false,
            camera_number: 0,
        }
    }

    pub fn exec(&self){
        for i in 0..self.ubo.len(){
            unsafe {
                setmeshbuf(self.meshid, i as u32, self.ubo[i]);
            };
        }
        unsafe {
            setdrawable(self.meshid, match self.draw {
                true => match self.draw_shadow {
                    true => 1,
                    false => 3,
                },
                false => match self.keep_shadow {
                    true => 2,
                    false => 0,
                },
            });
            setrendercamera(self.meshid, match self.render_all_cameras{
                true => -1,
                false => {
                    if self.exclude_selected_camera {
                        self.camera_number + 10
                    }else{
                        self.camera_number
                    }
                }
            });
        }
    }
}