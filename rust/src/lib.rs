use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
use godot::classes::RandomNumberGenerator;

mod player;

pub fn _ready() {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randomize();
    godot_print!("{:#?}", rng.randf_range(-10.0, 10.0));
}
