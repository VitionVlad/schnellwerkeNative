use super::{engine::Engine, material::Material, model::Model, render::render::{Mesh, MeshUsage}};

#[warn(dead_code)]
#[derive(Copy, Clone)]
pub struct Object{
    pub mesh: Mesh,
}

impl Object {
    pub fn new(engine: Engine, model: Model, material: Material, usage: MeshUsage) -> Object{
        Object { 
            mesh: Mesh::new(engine.render, model.vertexbuf, material.material_shaders, material.textures[material.used_texture], usage) 
        }
    }
    pub fn exec(&self){
        self.mesh.exec();
    }
}