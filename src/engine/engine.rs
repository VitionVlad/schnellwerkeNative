use super::render::render::Render;

pub struct Engine{
    pub render: Render,
}

impl Engine{
    pub fn new() -> Engine{
        Engine { 
            render: Render::new() 
        }
    }
    pub fn work(&mut self) -> bool{
        return self.render.continue_loop();
    }
    pub fn end(&mut self){
        self.render.destroy();
    }
}