use engine::engine::Engine;


mod engine;

fn main() {
    let mut eng = Engine::new();
    while eng.work(){
    }
}
