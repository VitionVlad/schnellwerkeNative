use crate::{app_state::AppState, engine::engine::Engine};

pub fn soundwork(eng: &mut Engine, state: &mut AppState) {
    for i in 0..state.sfx.len() {
        state.sfx[i].exec(eng);
        if state.sfx[i].loopsound{
            state.sfx[i].play = false;
        }
    }
}
