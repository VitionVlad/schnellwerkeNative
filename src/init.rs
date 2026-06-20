use std::fs;

use crate::{
    app_state::*,
    engine::{
        engine::Engine, image::Image, loader::{imageasset::ImageAsset, jsonparser::JsonF}, material::Material, math::{vec2::Vec2, vec3::Vec3}, scene::Scene, speaker::Speaker, ui::{UIplane, UItext}
    },
};

pub fn create_app(show_dbg_info: bool, skipl2: bool) -> (Engine, AppState) {
    let mut eng = Engine::new();
    eng.render.set_title("35mm");
    eng.render.set_new_resolution(1280, 720);

    let icon = ImageAsset::other_load("assets/ui/icon.png");

    eng.render.set_icon(icon.size[0], icon.size[1], icon.data);

    for _ in 0..2{
      eng.work();
    }

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();
    let imgf = fs::read("shaders/fimg").unwrap();

    let matt = Material::new(
        &eng,
        vert.clone(),
        textf,
        vec![],
        [
            super::engine::render::render::CullMode::CullModeNone,
            super::engine::render::render::CullMode::CullModeNone,
        ],
    );
    let mati = Material::new(
        &eng,
        vert.clone(),
        imgf,
        vec![],
        [
            super::engine::render::render::CullMode::CullModeNone,
            super::engine::render::render::CullMode::CullModeNone,
        ],
    );
    let mat = Material::new(
        &eng,
        vert.clone(),
        frag,
        vec![],
        [
            super::engine::render::render::CullMode::CullModeNone,
            super::engine::render::render::CullMode::CullModeNone,
        ],
    );
    let matgeneral = Material::new(
        &eng,
        dvert.clone(),
        dfrag,
        shadow.clone(),
        [
            super::engine::render::render::CullMode::CullModeBackBit,
            super::engine::render::render::CullMode::CullModeFrontBit,
        ],
    );
    let black = Image::new_color(&eng, [11, 23, 40, u8::MAX]);
    let bti1 = Image::new_from_files(&eng, vec!["assets/ui/cam.png".to_string()]);
    let bti2 = Image::new_from_files(&eng, vec!["assets/ui/bwf.png".to_string()]);
    let bti3 = Image::new_from_files(&eng, vec!["assets/ui/bwc.png".to_string()]);
    let bti4 = Image::new_from_files(&eng, vec!["assets/ui/pas.png".to_string()]);
    let bti5 = Image::new_from_files(&eng, vec!["assets/ui/btn.png".to_string()]);
    let bti6 = Image::new_from_files(&eng, vec!["assets/ui/nk.png".to_string()]);
    let bti7 = Image::new_from_files(&eng, vec!["assets/ui/tram.png".to_string()]);
    let bti8 = Image::new_from_files(&eng, vec!["assets/ui/ne.png".to_string()]);
    let bti9 = Image::new_from_files(&eng, vec!["assets/ui/fin.png".to_string()]);
    let bti10 = Image::new_from_files(&eng, vec!["assets/ui/shutter.png".to_string()]);
    let bti11 = Image::new_from_files(&eng, vec!["assets/ui/recc.png".to_string()]);
    let bti12 = Image::new_from_files(&eng, vec!["assets/ui/letter.png".to_string()]);
    let lg = Image::new_from_files(&eng, vec!["assets/ui/logo.png".to_string()]);

    let mut viewport = UIplane::new(&mut eng, mat, black);
    viewport.object.physic_object.pos.z = 1.0;
    viewport.signal = false;

    let mut bluepan = UIplane::new(&mut eng, mati, black);
    bluepan.object.physic_object.pos.z = 0.2;
    bluepan.signal = false;

    let mut cambtn = UIplane::new(&mut eng, mati, bti1);
    cambtn.object.physic_object.pos.z = 0.1;
    cambtn.signal = true;
    let mut bwbtn = UIplane::new(&mut eng, mati, bti2);
    bwbtn.object.physic_object.pos.z = 0.1;
    bwbtn.signal = true;
    let mut colbtn = UIplane::new(&mut eng, mati, bti3);
    colbtn.object.physic_object.pos.z = 0.1;
    colbtn.signal = true;
    let mut psbtn = UIplane::new(&mut eng, mati, bti4);
    psbtn.object.physic_object.pos.z = 0.1;
    psbtn.signal = true;

    let mut btnbtn = UIplane::new(&mut eng, mati, bti5);
    btnbtn.object.physic_object.pos.z = 0.1;
    btnbtn.signal = true;
    let mut nkbtn = UIplane::new(&mut eng, mati, bti6);
    nkbtn.object.physic_object.pos.z = 0.1;
    nkbtn.signal = true;
    let mut trambtn = UIplane::new(&mut eng, mati, bti7);
    trambtn.object.physic_object.pos.z = 0.1;
    trambtn.signal = true;
    let mut nebtn = UIplane::new(&mut eng, mati, bti8);
    nebtn.object.physic_object.pos.z = 0.1;
    nebtn.signal = true;
    let mut drbtn = UIplane::new(&mut eng, mati, bti9);
    drbtn.object.physic_object.pos.z = 0.1;
    drbtn.signal = true;
    let mut shbtn = UIplane::new(&mut eng, mati, bti10);
    shbtn.object.physic_object.pos.z = 0.1;
    shbtn.signal = true;
    let mut reccbtn = UIplane::new(&mut eng, mati, bti11);
    reccbtn.object.physic_object.pos.z = 0.1;
    reccbtn.signal = true;
    let mut lettbtn = UIplane::new(&mut eng, mati, bti12);
    lettbtn.object.physic_object.pos.z = 0.1;
    lettbtn.signal = true;
    let mut logo = UIplane::new(&mut eng, mati, lg);
    logo.object.physic_object.pos.z = 0.1;
    logo.signal = true;

    let mut fpscnt = UItext::new_from_file(
        &mut eng,
        matt,
        "assets/lat.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
    );
    let phcnt = vec![UItext::new_from_file(
        &mut eng,
        matt,
        "assets/lat.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
    ),UItext::new_from_file(
        &mut eng,
        matt,
        "assets/cyr.png",
        "AaBbVvGgDdEe[]JjZzIiYyKkLlMmNnOoPpRrSsTtUuFfHhXxCc{}/*`~!@#$%^&()'0123456789,.;:'+-<>_ ",
    )];
    let blacktxt = vec![UItext::new_from_file(
        &mut eng,
        matt,
        "assets/latblack.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
    ),UItext::new_from_file(
        &mut eng,
        matt,
        "assets/cyrblack.png",
        "AaBbVvGgDdEe[]JjZzIiYyKkLlMmNnOoPpRrSsTtUuFfHhXxCc{}/*`~!@#$%^&()'0123456789,.;:'+-<>_ ",
    )];
    //let phcnt = UItext::new_from_file(
    //    &mut eng,
    //    matt,
    //    "assets/textlat.png",
    //    "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%",
    //);
    //let mut ruitxt = vec![fpscnt, phcnt];

    let mut ruitxt = vec![vec![], vec![]];
    for _ in 0..8{
        ruitxt[0].push(UItext::new_from_file(
            &mut eng,
            matt,
            "assets/lat.png",
            "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$% ",
        ));
    }
    for _ in 0..8{
        ruitxt[1].push(UItext::new_from_file(
            &mut eng,
            matt,
            "assets/cyr.png",
            "AaBbVvGgDdEe[]JjZzIiYyKkLlMmNnOoPpRrSsTtUuFfHhXxCc{}/*`~!@#$%^&()'0123456789,.;:'+-<>_ ",
        ));
    }

    let mut scn = Scene::load_from_gltf(&mut eng, "assets/scene.glb", matgeneral);

    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.solid = false;
    eng.render.shadow_map_resolution = 1000;

    let mut cvec = vec![];
    let mut destructables = vec![];
    let mut stops = vec![];
    let mut btns = vec![];
    let mut doors = vec![];
    let mut ltsc = vec![]; 
    let mut ists = vec![]; 
    let mut pu = 0usize;
    let mut tramin = 0usize;

    let mut ekey = 0usize;
    let mut gkey = 0usize;
    let mut fdi = 0usize;

    for i in 0..scn.objects.len() {
        scn.objects[i].draw_distance = 1000_f32;
        if scn.objects[i].name == "Pivot" {
            pu = i;
        } else {
            let bt = scn.objects[i].name.as_bytes();
            if bt[0] == b'c' && bt[1] == b'a' && bt[2] == b'm' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                cvec.push(Colectable {
                    index: i,
                    ctype: 0,
                    consumed: false,
                    initial_pos: Vec3 {
                        x: scn.objects[i].physic_object.pos.x,
                        y: scn.objects[i].physic_object.pos.y,
                        z: scn.objects[i].physic_object.pos.z,
                    },
                });
                if show_dbg_info{
                    println!("Camera collectible found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                }
            } else if bt[0] == b'b' && bt[1] == b'w' && bt[2] == b'f' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                cvec.push(Colectable {
                    index: i,
                    ctype: 1,
                    consumed: false,
                    initial_pos: Vec3 {
                        x: scn.objects[i].physic_object.pos.x,
                        y: scn.objects[i].physic_object.pos.y,
                        z: scn.objects[i].physic_object.pos.z,
                    },
                });
                if show_dbg_info{
                    println!("B&W film collectible found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                }
            } else if bt[0] == b'c' && bt[1] == b'l' && bt[2] == b'f' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                cvec.push(Colectable {
                    index: i,
                    ctype: 2,
                    consumed: false,
                    initial_pos: Vec3 {
                        x: scn.objects[i].physic_object.pos.x,
                        y: scn.objects[i].physic_object.pos.y,
                        z: scn.objects[i].physic_object.pos.z,
                    },
                });
                if show_dbg_info{
                    println!("Color film collectible found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                }
            } else if bt[0] == b'c' && bt[1] == b'm' && bt[2] == b'r'{
                if bt[3] == b'e'{
                    ekey = i;
                    destructables.push(Destructable {
                        index: i,
                        initial_pos: Vec3 {
                            x: scn.objects[i].physic_object.pos.x,
                            y: scn.objects[i].physic_object.pos.y,
                            z: scn.objects[i].physic_object.pos.z,
                        },
                        destroyed: false,
                    });
                    if show_dbg_info{
                        println!("destructable ekey found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                    }
                } else if bt[3] == b'g'{
                    gkey = i;
                    destructables.push(Destructable {
                        index: i,
                        initial_pos: Vec3 {
                            x: scn.objects[i].physic_object.pos.x,
                            y: scn.objects[i].physic_object.pos.y,
                            z: scn.objects[i].physic_object.pos.z,
                        },
                        destroyed: false,
                    });
                    if show_dbg_info{
                        println!("destructable gkey found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                    }
                }else{
                    destructables.push(Destructable {
                        index: i,
                        initial_pos: Vec3 {
                            x: scn.objects[i].physic_object.pos.x,
                            y: scn.objects[i].physic_object.pos.y,
                            z: scn.objects[i].physic_object.pos.z,
                        },
                        destroyed: false,
                    });
                    if show_dbg_info{
                        println!("destructable found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                    }
                }
            } else if bt[0] == b't' && bt[1] == b'r' && bt[2] == b'a' && bt[3] == b'm' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                tramin = i;
            } else if bt[0] == b'f' && bt[1] == b'i' && bt[2] == b'n' && bt[3] == b'a' && bt[4] == b'l' {
                fdi = i;
            } else if bt[0] == b'n' && bt[1] == b's' {
                scn.objects[i].physic_object.solid = false;
            } else if bt[0] == b'o' && bt[1] == b'p' {
                stops.push(i);
            }else if bt[0] == b'b' && bt[1] == b't' && bt[2] == b'n' {
                btns.push(Ingbutton {
                    index: i,
                    axis: match bt[3] {
                        b'x' => 0,
                        b'y' => 1,
                        b'z' => 2,
                        _ => 0,
                    },
                    pressed: false,
                    scene_index: (bt[5]-b'0') as u32,
                    in_scene_index: (bt[4]-b'0') as u32,
                    initial_rot: scn.objects[i].physic_object.rot,
                });
                if show_dbg_info{
                    println!("button found at index {}, pos ({}, {}, {}), scene_index: {}, in_scene_index: {}", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z, btns.last().unwrap().scene_index, btns.last().unwrap().in_scene_index);
                }
            }else if bt[0] == b'd' {
                doors.push(Door {
                    index: i,
                    axis: match bt[1] {
                        b'x' => 0,
                        b'y' => 1,
                        b'z' => 2,
                        _ => 0,
                    },
                    movement: match bt[2] {
                        b'-' => -1.0 * (bt[3]-b'0') as f32,
                        _ =>(bt[3]-b'0') as f32,
                    },
                    initial_pos: Vec3 {
                        x: scn.objects[i].physic_object.pos.x,
                        y: scn.objects[i].physic_object.pos.y,
                        z: scn.objects[i].physic_object.pos.z,
                    },
                });
                if show_dbg_info{
                    println!("door found at index {}, pos ({}, {}, {}), axis: {}, movement: {}", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z, doors.last().unwrap().axis, doors.last().unwrap().movement);
                }
            }else if bt[0] == b's' && bt[1] == b't' && bt[2] == b'l' {
                btns.push(Ingbutton {
                    index: i,
                    axis: match bt[3] {
                        b'x' => 4,
                        b'y' => 5,
                        b'z' => 6,
                        _ => 4,
                    },
                    pressed: false,
                    scene_index: (bt[5]-b'0') as u32,
                    in_scene_index: (bt[4]-b'0') as u32,
                    initial_rot: scn.objects[i].physic_object.rot,
                });
                if show_dbg_info{
                    println!("switch found at index {}, pos ({}, {}, {}), scene_index: {}, in_scene_index: {}", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z, btns.last().unwrap().scene_index, btns.last().unwrap().in_scene_index);
                }
            }else if bt[0] == b'l' && bt[1] == b't'{
                scn.objects[i].draw = false;
                ltsc.push(Scenelightsource {
                    pos: Vec3 {
                        x: scn.objects[i].physic_object.pos.x,
                        y: scn.objects[i].physic_object.pos.y,
                        z: scn.objects[i].physic_object.pos.z,
                    },
                });
                if show_dbg_info{
                    println!("light source found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                }
            }else if bt[0] == b'i' && bt[1] == b's' && bt[2] == b't' {
                ists.push(Ist {
                    index: i,
                    number: (bt[3]-b'0') as u8,
                });
                if show_dbg_info{
                    println!("Paper found at index {}, number: {}", i, ists.last().unwrap().number);
                }
            }
        }
    }

    scn.objects[pu].physic_object.gravity = true;
    scn.objects[pu].physic_object.is_static = false;
    scn.objects[pu].physic_object.solid = true;
    scn.objects[pu].physic_object.step_height = 0.1;

    fpscnt.pos.x = 0.0;
    fpscnt.pos.y = 0.0;

    let mut sfx = vec![];

    sfx.push(Speaker::new(&mut eng, "assets/audio/walking.mp3"));
    sfx[0].loopsound = true;
    sfx[0].play = false;
    sfx[0].pos_dependency = false;
    sfx[0].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/shutter.mp3"));
    sfx[1].loopsound = false;
    sfx[1].play = false;
    sfx[1].pos_dependency = false;
    sfx[1].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/buzz.mp3"));
    sfx[2].loopsound = true;
    sfx[2].play = false;
    sfx[2].pos_dependency = false;
    sfx[2].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/switch.mp3"));
    sfx[3].loopsound = false;
    sfx[3].play = false;
    sfx[3].pos_dependency = false;
    sfx[3].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/tension.mp3"));
    sfx[4].loopsound = false;
    sfx[4].play = false;
    sfx[4].pos_dependency = false;
    sfx[4].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/tram.mp3"));
    sfx[5].loopsound = true;
    sfx[5].play = false;
    sfx[5].pos_dependency = false;
    sfx[5].use_pan = false;
    sfx[5].volume = 0.0;
    sfx.push(Speaker::new(&mut eng, "assets/audio/pickup.mp3"));
    sfx[6].loopsound = false;
    sfx[6].play = false;
    sfx[6].pos_dependency = false;
    sfx[6].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/bell.mp3"));
    sfx[7].loopsound = false;
    sfx[7].play = false;
    sfx[7].pos_dependency = false;
    sfx[7].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/stop.mp3"));
    sfx[8].loopsound = true;
    sfx[8].play = false;
    sfx[8].pos_dependency = false;
    sfx[8].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/ca.mp3"));
    sfx[9].loopsound = true;
    sfx[9].play = false;
    sfx[9].pos_dependency = false;
    sfx[9].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/type.mp3"));
    sfx[10].loopsound = false;
    sfx[10].play = false;
    sfx[10].pos_dependency = false;
    sfx[10].use_pan = false;

    for i in 0..sfx.len() {
        sfx[i].volume = 1.0;
    }

    sfx[2].volume = 0.5;

    eng.audio.vol = 1.0;

    let initial_pivot_pos = scn.objects[pu].physic_object.pos;

    let jsontext = JsonF::load_from_file("assets/text.json");
    if show_dbg_info{
        println!("Loaded JSON:");
        jsontext.printme();
    }

    let mln = jsontext.other_nodes[0].other_nodes.len();

    let abc = jsontext.other_nodes[0].other_nodes[0].other_nodes[0].numeral_val as usize;

    if show_dbg_info{
        println!("total number of languages: {}", mln)
    }

    let state = AppState {
        viewport,
        bluepan,
        cambtn,
        bwbtn,
        colbtn,
        psbtn,
        btnbtn,
        nkbtn,
        trambtn,
        nebtn,
        drbtn,
        shbtn,
        reccbtn,
        lettbtn,
        logo,
        fpscnt,
        phcnt,
        blacktxt,
        ruitxt,
        scn,
        cvec,
        destructables,
        ekey: ekey,
        gkey: gkey,
        initial_ekey: ekey,
        initial_gkey: gkey,
        btns: btns,
        scenelightsources: ltsc,
        doors: doors,
        stops,
        cstop: 0_u32,
        intram: false,
        tm: 0,
        ttm: 0,
        pu,
        pivotr: 0.0_f32,
        pkbf: 11_f32,
        tramin,
        bwfilm: 0_u32,
        clfilm: 0_u32,
        cme: false,
        selp: 0_u8,
        locls: 1,
        aproxpoint: [
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.0, y: 0.0 },
        ],
        lsp: (Vec2 { x: 0.0, y: 0.0 }, false),
        sfx: sfx,
        dbg: show_dbg_info,
        switch_states: [false; 6],
        switched_1_4: false,
        switched_5_6: false,
        sc3state: 0,
        finaldooridx: fdi,
        initial_pivot_pos,
        skp2: skipl2,
        controlt: 0,
        joy_origin: Vec2 { x: 0.0, y: 0.0 },
        left_hand: false,
        keycodes: vec![26u32, 48u32, 40u32, 44u32, 25u32, 22u32, 13u32, 14u32, 15u32, 3u32, 4u32, 49u32],
        gamepad_axes: vec![0u8, 1u8, 5u8],
        gamepad_buttons: vec![0u8, 1u8, 2u8, 3u8, 7u8, 10u8, 12u8],
        shadowmapquality: eng.render.shadow_map_resolution,
        ists: ists,
        jsontext,
        current_letter: -1,
        current_light_scene: 0,
        firstbw: false,
        firstcol: false,
        pausemn: false,
        framecnt: 0u64,
        menusel: 0u8,
        close: false,
        autosaves: false,
        showfps: false,
        gamepadmenusel: 0i8,
        menumasel: 0i8,
        gameending: false,
        lastltsim: 1,
        simtim: 0,
        abc: abc,
        current_lang: 0,
        max_lang_nm: mln,
    };

    (eng, state)
}
