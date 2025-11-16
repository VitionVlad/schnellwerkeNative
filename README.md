# <p align="center"> <img src="https://github.com/VitionVlad/schnellwerkeNative/blob/main/assets/logo_long.png"> </p>
Schnellwerke was one of my projects that managed to evolve into something more—it was actually used by me in several other projects. Having a solid way to develop games across all platforms is great, but as a true PC gamer, I aim for much more: better graphics, higher performance, and pushing the limits of hardware.  
This is my current goal. Schnellwerke Native is a port of my graphics engine from Rust (WebAssembly) + JavaScript to Rust + C. This doesn’t mean the web version will be deprecated—on the contrary, the native and web versions will remain compatible with each other. The result will be a powerful and flexible tool that still allows for cross-platform development, while achieving significantly better performance and visuals on PC.  
# <p align="center"> Structure  </p>
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

Engine handle creation and render loop handling is also quiet simple:  

```rust
let mut eng = Engine::new();

...

while eng.work(){

...

}

eng.end();
```

Objects as was earlier mentioned didnt change that much, as shown in this diagram:  
<p align="center"><img width="621" height="382" alt="Diagramă fără titlu-Pagină-2 drawio" src="https://github.com/user-attachments/assets/3104494d-1da7-40af-9cfe-adc1047312b2" /></p>
and objects structure itself:  

```rust
pub struct Object{
    pub mesh: Mesh,
    pub physic_object: PhysicsObject,
    pub is_looking_at: bool,
    pub draw: bool,
    pub draw_shadow: bool,
    pub draw_distance: f32,
    pub view_reaction_distance: f32,
    pub render_in_behind: bool,
}
```

But now creation an object is much more tricky, as it requires to load in a separate structure model, images, and shaders:  

```rust
let dvert = fs::read("shaders/vdeffered").unwrap();
let dfragem = fs::read("shaders/fdeffem").unwrap();
let shadow = fs::read("shaders/shadow").unwrap();
let mat4 = Material::new(&eng, dvert, dfragem, shadow, [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
let image = Image::new_color(&eng, [i8::MAX, i8::MAX, i8::MAX, i8::MAX]);//can also be loaded from file
let mut vrt1 = ModelAsset::load_obj("assets/train_em.obj");
let md1 = Model::new(&mut eng, vrt1.vertices[0].clone());
let mut trainem = Object::new(&mut eng, md1, mat4, image, engine::render::render::MeshUsage::DefferedPass, true);

while eng.work(){
...
//but executing them is still simple
trainem.exec(&mut eng);
...
}
```

There also are some special objects, which are UItext and UIplane, that are obviosly used for ui, here an example:  

```rust
let mut viewport = UIplane::new(&mut eng, mat, image);
let mut text: [UItext; 5] = [
  UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_"),
  ...
];
```

they also can be intractive if you want, this will requiere to change a flag, speaking about this, here is their structure:  

```rust
pub struct UIplane{
    pub object: Object,
    pub clickzone: Clickzone,
    pub signal: bool,
    pub allow_when_mouse_locked: bool,
}

pub struct UItext{
    pub font: Image,
    pub symbols: Vec<u8>,
    pub planes: Vec<Object>,
    pub symbol_number: u32,
    pub material: Material,
    pub size: Vec2,
    pub pos: Vec3,
    pub clickzone: Clickzone,
    pub signal: bool,
    pub per_symbol: bool,
    pub allow_when_mouse_locked: bool,
    pub draw: bool,
    pub symbol_pressed: u8,
    pub symbol_index: usize,
}
```

Also, Objects can be loaded from file, via scene, in this case, engine will parse also material library, only needed thing along side textures and model itself are shaders, so an example:  

```rust
let mat2 = Material::new(&eng, dvert.clone(), dfrag, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
...
let mut train = Scene::load_from_obj(&mut eng, "assets/train.obj", mat2);
```

here is also scene structure:  

```rust
pub struct Scene{
    pub objects: Vec<Object>,
    pub use_global_values: bool,
    pub pos: Vec3,
    pub scale: Vec3,
    pub rot: Vec3,
    pub render_all_cameras: bool,
    pub exclude_selected_camera: bool,
    pub camera_number: i8,
}
```

# <p align="center"> Render </p>
Rendering in Schnellwerke 3 Native is based on the Euclid component, which handles all interaction with the Vulkan API, as well as managing input and window operations.  
The Euclid component was named this way due to the original intention of creating a renderer capable of handling non-Euclidean space. While this feature has never been tested, in theory, it should work—mainly because the engine supports multiple cameras in the deferred pass (up to 10) and up to 100 light sources.  
The main rendering approach is deferred rendering, although you're free to rewrite the shaders yourself and use traditional forward rendering instead.  
Below is a diagram that represents the entire rendering process.  
<p align="center"><img width="531" height="1791" alt="Diagramă fără titlu-Pagină-3 drawio" src="https://github.com/user-attachments/assets/6b755df8-e7dc-43e9-949f-8d3db038684d" /> </p>  
Important note: Since version 3.1, no additional image is used to store texel positions; they are now computed using the matrix inverse. Also, the deferred render framebuffer now uses 16-bit depth color, which slightly improves performance in cases of bandwidth limitation  
This demo also showcase the rendering of transparent objects, which is significantly more difficult in a deferred rendering approach.  

# <p align="center"> Physics </p>
All physics calculations are not directly exposed to the programmer. They are mostly executed at the start of each new frame, as I chose to use a tick-based approach for physics simulation. This means the physics engine runs at a different tick rate than the game itself—it can be higher or lower. This approach ensures frame rate–independent physics.  

# <p align="center"> Audio </p>
The native version of the engine uses Miniaudio for audio handling.  
It supports all popular audio formats and provides simple yet sufficient functionality—such as setting pan and volume.  
To create an audio source, the engine uses a Speaker structure, which looks like this:  

```rust
pub struct Speaker{
    pub pos: Vec3,
    pub play: bool,
    pub power: f32,
    pub use_pan: bool,
    pub pos_dependency: bool,
    pub volume: f32,
}
```

and working with this structure looks like this:  

```rust
let mut trains = Speaker::new(&mut eng, "assets/audio/train.mp3");
...
while eng.work(){
...
trains.exec(&mut eng);
...
}
```

# <p align="center"> <img width="1024" height="512" alt="postdev" src="https://github.com/user-attachments/assets/b7fb0768-696b-4328-977a-10dc5f5bbeda" /> </p>  
