use super::{engine::Engine, material::Material, model::Model, render::render::Mesh};

#[warn(dead_code)]
pub struct Object{
    pub mesh: Mesh,
}

impl Object {
    pub fn new(engine: Engine, model: Model, material: Material) -> Object{
        Object { 
            mesh: Mesh::new(engine.render, model.vertexbuf, material.material_shaders) 
        }
    }
    pub fn exec(&self){
        self.mesh.exec();
    }
}