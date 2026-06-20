use std::f32::consts::PI;

use crate::{app_state::{AppState, SPEED}, engine::engine::Engine};

pub fn control_handle(eng: &mut Engine, state: &mut AppState) {
    if !state.intram && !state.pausemn && !state.gameending{
        if eng.control.get_key_state(state.keycodes[2]) && state.selp != 3{
            state.scn.objects[state.pu].physic_object.acceleration.x += -SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = PI / 2.0;
            state.sfx[0].play = true;
            state.controlt = 0;
        } else if eng.control.get_key_state(state.keycodes[3]) && state.selp != 3{
            state.scn.objects[state.pu].physic_object.acceleration.x += SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = (PI / 2.0) * 3.0;
            state.sfx[0].play = true;
            state.controlt = 0;
        } else if eng.control.get_key_state(state.keycodes[4]) && state.selp != 3{
            state.scn.objects[state.pu].physic_object.acceleration.z += SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = PI;
            state.sfx[0].play = true;
            state.controlt = 0;
        } else if eng.control.get_key_state(state.keycodes[5]) && state.selp != 3{
            state.scn.objects[state.pu].physic_object.acceleration.z += -SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = 0.0;
            state.sfx[0].play = true;
            state.controlt = 0;
        }else if eng.control.get_key_state(state.keycodes[6]) && state.cme && state.tm <= 0 && !state.intram {
            state.selp = 0;
            state.tm = 50;
            state.controlt = 0;
            state.current_letter = -1;
        }else if eng.control.get_key_state(state.keycodes[7]) && state.cme && state.tm <= 0 && !state.intram {
            state.selp = 1;
            state.tm = 50;
            state.controlt = 0;
            state.current_letter = -1;
        }else if eng.control.get_key_state(state.keycodes[8]) && state.cme && state.tm <= 0 && !state.intram {
            state.selp = 2;
            state.tm = 50;
            state.controlt = 0;
            state.current_letter = -1;
        }

        if eng.control.mousebtn[2] && state.selp != 3 && state.tm <= 0{
            let resx_half = (eng.render.resolution_x as f64) / 2.0;
            if eng.control.xpos < resx_half - 80.0 && !state.left_hand || eng.control.xpos >= resx_half + 80.0 && state.left_hand {
                if state.controlt != 1 {
                    state.controlt = 1;
                    state.joy_origin.x = eng.control.xpos as f32;
                    state.joy_origin.y = eng.control.ypos as f32;
                }
                let dx = eng.control.xpos as f32 - state.joy_origin.x;
                let dy = eng.control.ypos as f32 - state.joy_origin.y;
                if dx != 0.0 || dy != 0.0 {
                    let ang = dx.atan2(dy)+PI/4.0;
                    state.pivotr = ang;
                    let a = SPEED * eng.times_to_calculate_physics as f32;
                    state.scn.objects[state.pu].physic_object.acceleration.x += -a * state.pivotr.sin();
                    state.scn.objects[state.pu].physic_object.acceleration.z += -a * state.pivotr.cos();
                    state.sfx[0].play = true;
                }
            }
        }

        if eng.control.gamepad_axis_count > 0 && state.selp != 3{
            let axis_x = eng.control.get_gamepad_axis_state(state.gamepad_axes[0]);
            let axis_y = eng.control.get_gamepad_axis_state(state.gamepad_axes[1]);
            if axis_x.abs() > 0.1 || axis_y.abs() > 0.1 {
                state.controlt = 2;
                let ang = (axis_x as f32).atan2(axis_y as f32);
                state.pivotr = ang;
                let a = SPEED * eng.times_to_calculate_physics as f32;
                state.scn.objects[state.pu].physic_object.acceleration.x += -a * state.pivotr.sin();
                state.scn.objects[state.pu].physic_object.acceleration.z += -a * state.pivotr.cos();
                state.sfx[0].play = true;
            }
        }

        if eng.control.gamepad_button_count > 0 {
            if eng.control.get_gamepad_button_state(state.gamepad_buttons[0]) && state.cme && state.tm <= 0 && !state.intram && state.selp != 3{
                state.selp = 0;
                state.tm = 50;
                state.controlt = 2;
            } else if eng.control.get_gamepad_button_state(state.gamepad_buttons[1]) && state.tm <= 0 && !state.intram{
                if state.selp != 3 && state.cme{
                    state.selp = 1;
                    state.tm = 50;
                    state.controlt = 2;
                }else if state.selp == 3{
                    state.selp = 0;
                    state.tm = 50;
                    state.controlt = 2;
                    state.current_letter = -1;
                }
            } else if eng.control.get_gamepad_button_state(state.gamepad_buttons[2]) && state.cme && state.tm <= 0 && !state.intram && state.selp != 3{
                state.selp = 2;
                state.tm = 50;
                state.controlt = 2;
            }
        }

        if state.controlt != 1{
            state.shbtn.object.draw = false;
            state.reccbtn.object.draw = false;
            //state.selp = 0;
            state.shbtn.exec(eng);
            state.reccbtn.exec(eng);
        }
    }else if state.pausemn{
        if eng.control.get_gamepad_button_state(state.gamepad_buttons[5]) && state.tm <= 0{
            state.gamepadmenusel = (state.gamepadmenusel-1).max(0);
            state.tm = 50;
            state.controlt = 2;
        }else if eng.control.get_gamepad_button_state(state.gamepad_buttons[6]) && state.tm <= 0{
            state.gamepadmenusel = (state.gamepadmenusel+1).min(state.menumasel-1);
            state.tm = 50;
            state.controlt = 2;
        }
    }

    if eng.control.get_gamepad_button_state(state.gamepad_buttons[4]) && state.tm <= 0{
        state.pausemn = !state.pausemn;
        state.menusel = 0;
        state.gamepadmenusel = 0;
        state.tm = 50;
        state.controlt = 2;
    }

    if eng.control.get_key_state(state.keycodes[11]) && state.tm <= 0{
        state.pausemn = !state.pausemn;
        state.menusel = 0;
        state.tm = 50;
    }

    let step = SPEED * eng.times_to_calculate_physics as f32 * 20.0;
    let error_margin = SPEED * 5.0;
    let mut delta = (state.pivotr - state.scn.objects[state.pu].physic_object.rot.y + PI) % (2.0 * PI) - PI;
    if delta < -PI {
        delta += 2.0 * PI;
    }
    if delta.abs() <= error_margin {
        state.scn.objects[state.pu].physic_object.rot.y = state.pivotr;
    } else {
        let direction = delta.signum();
        let movement = direction * step;
        if step > delta.abs() {
            state.scn.objects[state.pu].physic_object.rot.y = state.pivotr;
        } else {
            state.scn.objects[state.pu].physic_object.rot.y += movement;
        }
    }
}
