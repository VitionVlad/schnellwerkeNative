use super::render::render::Render;

#[warn(dead_code)]
#[derive(Copy, Clone)]
pub struct Engine{
    pub render: Render,
}

impl Engine{
    #[warn(dead_code)]
    pub fn new() -> Engine{
        Engine { 
            render: Render::new() 
        }
    }
    #[warn(dead_code)]
    pub fn work(&mut self) -> bool{
        return self.render.continue_loop();
    }
    #[warn(dead_code)]
    pub fn end(&mut self){
        self.render.destroy();
    }
}