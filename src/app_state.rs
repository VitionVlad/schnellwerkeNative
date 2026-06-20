#![allow(dead_code)]

use crate::engine::{
    loader::jsonparser::JsonF, math::{vec2::Vec2, vec3::Vec3}, scene::Scene, speaker::Speaker, ui::{UIplane, UItext}
};

pub const SPEED: f32 = 0.0025_f32;
pub const TICKSZ: f32 = 1.0 / 250.0;

pub struct Colectable {
    pub index: usize,
    pub ctype: u8,
    pub consumed: bool,
    pub initial_pos: Vec3,
}

pub struct Destructable {
    pub index: usize,
    pub initial_pos: Vec3,
    pub destroyed: bool,
}

pub struct Ingbutton{
    pub index: usize,
    pub axis: u8,
    pub pressed: bool,
    pub scene_index: u32,
    pub in_scene_index: u32,
    pub initial_rot: Vec3,
}

pub struct Scenelightsource{
    pub pos: Vec3,
}

pub struct Door{
    pub index: usize,
    pub axis: u8,
    pub movement: f32,
    pub initial_pos: Vec3,
}

pub struct Ist{
    pub index: usize,
    pub number: u8,
}

pub struct AppState {
    pub viewport: UIplane,
    pub bluepan: UIplane,
    pub cambtn: UIplane,
    pub bwbtn: UIplane,
    pub colbtn: UIplane,
    pub psbtn: UIplane,
    pub btnbtn: UIplane,
    pub nkbtn: UIplane,
    pub trambtn: UIplane,
    pub nebtn: UIplane,
    pub drbtn: UIplane,
    pub shbtn: UIplane,
    pub reccbtn: UIplane,
    pub lettbtn: UIplane,
    pub logo: UIplane,
    pub fpscnt: UItext,
    pub phcnt: Vec<UItext>,
    pub blacktxt: Vec<UItext>,
    pub ruitxt: Vec<Vec<UItext>>,   
    pub scn: Scene,
    pub cvec: Vec<Colectable>,
    pub destructables: Vec<Destructable>,
    pub ekey: usize,
    pub gkey: usize,
    pub initial_ekey: usize,
    pub initial_gkey: usize,
    pub stops: Vec<usize>,
    pub btns: Vec<Ingbutton>,
    pub scenelightsources: Vec<Scenelightsource>,
    pub doors: Vec<Door>,
    pub cstop: u32,
    pub intram: bool,
    pub tm: i32,
    pub ttm: i32,
    pub pu: usize,
    pub pivotr: f32,
    pub pkbf: f32,
    pub tramin: usize,
    pub bwfilm: u32,
    pub clfilm: u32,
    pub cme: bool,
    pub selp: u8,
    pub locls: u32,
    pub aproxpoint: [Vec2; 4],
    pub lsp: (Vec2, bool),
    pub sfx: Vec<Speaker>,
    pub dbg: bool,
    pub switch_states: [bool; 6],
    pub switched_1_4: bool,
    pub switched_5_6: bool,
    pub sc3state: u8,
    pub finaldooridx: usize,
    pub initial_pivot_pos: Vec3,
    pub skp2: bool,
    pub controlt: u8,
    pub joy_origin: Vec2,
    pub left_hand: bool,
    pub keycodes: Vec<u32>,
    pub gamepad_axes: Vec<u8>,
    pub gamepad_buttons: Vec<u8>,
    pub shadowmapquality: u32,
    pub ists: Vec<Ist>,
    pub jsontext: JsonF,
    pub current_letter: i8,
    pub current_light_scene: u8,
    pub firstbw: bool,
    pub firstcol: bool,
    pub pausemn: bool,
    pub framecnt: u64,
    pub menusel: u8,
    pub close: bool,
    pub autosaves: bool,
    pub showfps: bool,
    pub gamepadmenusel: i8,
    pub menumasel: i8,
    pub gameending: bool,
    pub lastltsim: usize,
    pub simtim: i32,
    pub abc: usize,
    pub current_lang: usize,
    pub max_lang_nm: usize,
}

pub fn distance(v1: Vec3, v2: Vec3) -> f32 {
    f32::sqrt((v2.x - v1.x).powi(2) + (v2.z - v1.z).powi(2))
}
