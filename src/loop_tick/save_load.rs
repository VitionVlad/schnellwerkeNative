#[allow(dead_code)]
use std::fs;
use std::io::Write;
use std::f32::consts::PI;
use crate::engine::loader::jsonparser::JsonF;
use crate::app_state::AppState;
use crate::engine::engine::Engine;

/// Save the provided `AppState` as JSON to `path`.
pub fn save_progress(path: &str, app: &AppState) -> Result<(), std::io::Error> {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str(&format!("  \"firstbw\": {},\n", app.firstbw));
    s.push_str(&format!("  \"firstcol\": {},\n", app.firstcol));
    s.push_str(&format!("  \"current_light_scene\": {},\n", app.current_light_scene));
    s.push_str(&format!("  \"cstop\": {},\n", app.cstop));
    s.push_str(&format!("  \"switched_1_4\": {},\n", app.switched_1_4));
    s.push_str(&format!("  \"switched_5_6\": {},\n", app.switched_5_6));

    // switch_states array
    s.push_str("  \"switch_states\": [");
    for (i, v) in app.switch_states.iter().enumerate() {
        if i != 0 { s.push_str(", "); }
        s.push_str(if *v { "true" } else { "false" });
    }
    s.push_str("],\n");

    // lsp as object
    s.push_str("  \"lsp\": { ");
    s.push_str(&format!("\"x\": {:.6}, \"y\": {:.6}, \"enabled\": {} }},\n", app.lsp.0.x, app.lsp.0.y, app.lsp.1));
    s.push_str(&format!("  \"sc3state\": {},\n", app.sc3state));
    s.push_str(&format!("  \"bwfilm\": {},\n", app.bwfilm));
    s.push_str(&format!("  \"clfilm\": {},\n", app.clfilm));
    s.push_str(&format!("  \"cme\": {},\n", app.cme));

    // btns pressed state array
    s.push_str("  \"btns\": [");
    for (i, b) in app.btns.iter().enumerate() {
        if i != 0 { s.push_str(", "); }
        s.push_str(if b.pressed { "true" } else { "false" });
    }
    s.push_str("],\n");

    // destructables_draw array
    s.push_str("  \"destructables\": [");
    for i in 0..app.destructables.len(){
        if i != 0 {
            s.push_str(", ");
        }
        let draw = app.destructables[i].destroyed;
        s.push_str(if draw { "true" } else { "false" });
    }
    s.push_str("],\n");

    // cvec_consumed array
    s.push_str("  \"cvec\": [");
    for (i, c) in app.cvec.iter().enumerate() {
        if i != 0 { s.push_str(", "); }
        s.push_str(if c.consumed { "true" } else { "false" });
    }
    s.push_str("],\n");
    s.push_str(&format!("  \"pivot\": [{:.6}, {:.6}, {:.6}]\n", app.scn.objects[app.pu].physic_object.pos.x, app.scn.objects[app.pu].physic_object.pos.y, app.scn.objects[app.pu].physic_object.pos.z));
    s.push_str("}\n");

    let mut f = fs::File::create(path)?;
    f.write_all(s.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
pub fn save_settings(path: &str, eng: &Engine, app: &AppState) -> Result<(), std::io::Error> {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str(&format!("  \"fullscreen\": {},\n", eng.render.fullscreen));
    s.push_str(&format!("  \"render_resolution\": {:.2},\n", eng.render.resolution_scale));
    s.push_str(&format!("  \"shadowmaps_quality\": {},\n", app.shadowmapquality));
    s.push_str(&format!("  \"show_fps\": {},\n", app.showfps));
    s.push_str(&format!("  \"volume\": {:.3},\n", eng.audio.vol));
    s.push_str(&format!("  \"left_hand_mode\": {},\n", app.left_hand));
    s.push_str(&format!("  \"autosaves\": {},\n", app.autosaves));
    s.push_str(&format!("  \"lang\": {}\n", app.current_lang));
    s.push_str("}\n");

    let mut f = fs::File::create(path)?;
    f.write_all(s.as_bytes())?;
    Ok(())
}

pub fn load_progress(path: &str, state: &mut AppState){
    let json = JsonF::load_from_file(path);
    for i in 0..json.other_nodes.len() {
        match json.other_nodes[i].name.as_str() {
            "firstbw" => state.firstbw = json.other_nodes[i].bolean,
            "firstcol" => state.firstcol = json.other_nodes[i].bolean,
            "current_light_scene" => state.current_light_scene = json.other_nodes[i].numeral_val as u8,
            "cstop" => {
                let lcstop = json.other_nodes[i].numeral_val as u32;
                //if state.dbg{
                //    println!("cstop {}, initial_tram_pos: {}, => {}", state.cstop, state.scn.objects[state.tramin].physic_object.pos.x, state.scn.objects[state.stops[state.cstop as usize-1]].physic_object.pos.x)
                //}
                if lcstop == 0{
                    state.scn.objects[state.tramin].physic_object.pos.x = 6.34336;
                    state.cstop = 0;
                }else{
                    state.cstop = lcstop;
                    state.scn.objects[state.tramin].physic_object.pos.x = state.scn.objects[state.stops[state.cstop as usize-1]].physic_object.pos.x;
                }
            },
            "switched_1_4" => state.switched_1_4 = json.other_nodes[i].bolean,
            "switched_5_6" => state.switched_5_6 = json.other_nodes[i].bolean,
            "switch_states" => {
                for j in 0..json.other_nodes[i].other_nodes.len() {
                    state.switch_states[j] = json.other_nodes[i].other_nodes[j].bolean;
                }
            },
            "lsp" => {
                state.lsp.0.x = json.other_nodes[i].other_nodes[0].numeral_val as f32;
                state.lsp.0.y = json.other_nodes[i].other_nodes[1].numeral_val as f32;
                state.lsp.1 = json.other_nodes[i].other_nodes[2].bolean;
            },
            "destructables" => {
                for j in 0..json.other_nodes[i].other_nodes.len() {
                    if !json.other_nodes[i].other_nodes[j].bolean {
                        state.scn.objects[state.destructables[j].index].draw = true;
                        state.scn.objects[state.destructables[j].index].physic_object.pos = state.destructables[j].initial_pos;
                        state.destructables[j].destroyed = false;
                    }else{
                        state.scn.objects[state.destructables[j].index].draw = false;
                        state.scn.objects[state.destructables[j].index].physic_object.pos.y = -1000.0;
                        state.destructables[j].destroyed = true;
                    }
                }
            },
            "sc3state" => state.sc3state = json.other_nodes[i].numeral_val as u8,
            "bwfilm" => state.bwfilm = json.other_nodes[i].numeral_val as u32,
            "clfilm" => state.clfilm = json.other_nodes[i].numeral_val as u32,
            "cme" => state.cme = json.other_nodes[i].bolean,
            "btns" => {
                for j in 0..json.other_nodes[i].other_nodes.len() {
                    if j < state.btns.len() {
                        state.btns[j].pressed = json.other_nodes[i].other_nodes[j].bolean;
                    }
                }
            },
            "cvec" => {
                for j in 0..json.other_nodes[i].other_nodes.len() {
                    state.cvec[j].consumed = json.other_nodes[i].other_nodes[j].bolean;
                    state.scn.objects[state.cvec[j].index].draw = !state.cvec[j].consumed;
                    state.scn.objects[state.cvec[j].index].physic_object.pos.y = if state.cvec[j].consumed { -1000.0 } else { state.scn.objects[state.cvec[j].index].physic_object.pos.y };
                }
            },
            "pivot" => {
                state.scn.objects[state.pu].physic_object.pos.x = json.other_nodes[i].other_nodes[0].numeral_val as f32;
                state.scn.objects[state.pu].physic_object.pos.y = json.other_nodes[i].other_nodes[1].numeral_val as f32;
                state.scn.objects[state.pu].physic_object.pos.z = json.other_nodes[i].other_nodes[2].numeral_val as f32;
            },
            _ => {}
        }
    }

    let button_status: Vec<(u32, u32, bool)> = state.btns.iter().map(|b| (b.in_scene_index, b.scene_index, b.pressed)).collect();
    for button in state.btns.iter() {
        let rot_axis = if button.axis >= 4 { button.axis - 3 } else { button.axis };
        let same_index_pressed = button_status.iter().any(|&(idx, scene_idx, pressed)| idx == button.in_scene_index && scene_idx == button.scene_index && pressed);

        if button.axis < 4 {
            match rot_axis {
                0 => state.scn.objects[button.index].physic_object.rot.x = button.initial_rot.x + if button.pressed { PI } else { 0.0 },
                1 => state.scn.objects[button.index].physic_object.rot.y = button.initial_rot.y + if button.pressed { PI } else { 0.0 },
                2 => state.scn.objects[button.index].physic_object.rot.z = button.initial_rot.z + if button.pressed { PI } else { 0.0 },
                _ => {}
            }
        } else {
            match rot_axis {
                0 => state.scn.objects[button.index].physic_object.rot.x = button.initial_rot.x + if same_index_pressed { PI } else { 0.0 },
                1 => state.scn.objects[button.index].physic_object.rot.y = button.initial_rot.y + if same_index_pressed { PI } else { 0.0 },
                2 => state.scn.objects[button.index].physic_object.rot.z = button.initial_rot.z + if same_index_pressed { PI } else { 0.0 },
                _ => {}
            }
        }
    }
}

#[allow(dead_code)]
pub fn load_settings(path: &str, eng: &mut Engine, state: &mut AppState){
    let json = JsonF::load_from_file(path);
    for i in 0..json.other_nodes.len() {
        match json.other_nodes[i].name.as_str() {
            "fullscreen" => eng.render.fullscreen = json.other_nodes[i].bolean,
            "render_resolution" => eng.render.resolution_scale = json.other_nodes[i].numeral_val as f32,
            "shadowmaps_quality" => state.shadowmapquality = json.other_nodes[i].numeral_val as u32,
            "show_fps" => state.showfps = json.other_nodes[i].bolean,
            "volume" => eng.audio.vol = json.other_nodes[i].numeral_val as f32,
            "left_hand_mode" => state.left_hand = json.other_nodes[i].bolean,
            "autosaves" => state.autosaves = json.other_nodes[i].bolean,
            "lang" => {
                state.current_lang = json.other_nodes[i].numeral_val as usize;
                state.abc = state.jsontext.other_nodes[0].other_nodes[state.current_lang].other_nodes[0].numeral_val as usize;
            },
            _ => {}
        }
    }
}