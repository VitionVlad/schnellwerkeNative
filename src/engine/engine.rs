use super::render::render::{Control, Render};

#[warn(dead_code)]
#[derive(Copy, Clone)]
pub struct Engine{
    pub render: Render,
    pub control: Control,
}

impl Engine{
    #[warn(dead_code)]
    pub fn new() -> Engine{
        let rn = Render::new();
        Engine { 
            render: rn,
            control: Control::new(rn), 
        }
    }
    #[warn(dead_code)]
    pub fn work(&mut self) -> bool{
        self.control.get_mouse_pos();
        return self.render.continue_loop();
    }
    #[warn(dead_code)]
    pub fn end(&mut self){
        self.render.destroy();
    }
}