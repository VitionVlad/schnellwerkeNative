#![windows_subsystem = "windows"]
use std::{env, fs};

use crate::loop_tick::save_load::{load_progress, load_settings};

mod app_state;
mod engine;
mod init;
mod loop_tick;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (mut eng, mut state) = init::create_app(
        if args.iter().any(|x| x == "-debug") { 
            true 
        } 
        else { 
            false
        },
        if args.iter().any(|x| x == "-skipl2") { 
            true 
        } 
        else { 
            false
        }
    );

    if fs::exists("settings.json").unwrap() {
        let _ = load_settings("settings.json", &mut eng, &mut state);
    }

    if fs::exists("save.json").unwrap() {
        let _ = load_progress("save.json", &mut state);
    }

    while eng.work() {
        loop_tick::soundwork::soundwork(&mut eng, &mut state);
        loop_tick::control_handle::control_handle(&mut eng, &mut state);
        loop_tick::tick::tick(&mut eng, &mut state);
        loop_tick::per_select_tick::per_select_tick(&mut eng, &mut state);
        loop_tick::handle_scene::handle_scene(&mut eng, &mut state);
        loop_tick::menu_handle::menu_handle(&mut eng, &mut state);
        state.framecnt+=1;
        if state.close{
            break;
        }
    }

    eng.end();
}