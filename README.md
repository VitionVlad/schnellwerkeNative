# <p align="center"> <img src="https://github.com/VitionVlad/schnellwerkeNative/blob/main/assets/logo_long.png"> </p>
Schnellwerke was one of my projects that managed to evolve into something more—it was actually used by me in several other projects. Having a solid way to develop games across all platforms is great, but as a true PC gamer, I aim for much more: better graphics, higher performance, and pushing the limits of hardware.  
This is my current goal. Schnellwerke Native is a port of my graphics engine from Rust (WebAssembly) + JavaScript to Rust + C. This doesn’t mean the web version will be deprecated—on the contrary, the native and web versions will remain compatible with each other. The result will be a powerful and flexible tool that still allows for cross-platform development, while achieving significantly better performance and visuals on PC.  
# Structure  
Version 3.0 introduced a restructured system focused on more efficient resource usage. Now, textures, shaders, and models are no longer bound to individual objects—they can be shared across multiple objects.  
This is especially useful for textures, as it eliminates unnecessary duplication. Instead of loading the same texture for each object (which leads to increased memory consumption), a single shared texture is used, significantly reducing memory usage.  
For example, the demo game ZUG runs with a scene containing over 100 objects—many of which reuse the same materials — and it consumes only about 300 MB of RAM.  
The engine and object structure hasn’t changed much, remaining mostly the same. The only notable difference is the controls, which are now part of the engine structure, as shown in this diagram.  
<p align="center"><img width="602" height="622" alt="Diagramă fără titlu-Pagină-1 drawio" src="https://github.com/user-attachments/assets/3e6b3ecf-67af-4a25-8325-e272f89e98dd" /> </p>
and the strucutre itself:  
```rust
pub struct Engine{
    pub render: Render,
    pub audio: AudioEngine,
    pub control: Control,
    pub cameras: [Camera; 10],
    pub used_camera_count: u32,
    pub lights: [Light; 100],
    pub used_light_count: u32,
    pub physics_tick: u32,
    pub times_to_calculate_physics: u32,
    pub obj_ph: Vec<PhysicsObject>,
    pub fps: u32,
    pub primary_camera: usize,
}
```
