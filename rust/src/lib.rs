use godot::engine::Sprite2D;
use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
<<<<<<< HEAD
<<<<<<< HEAD
use godot::engine::RandomNumberGenerator;
=======
>>>>>>> c18a55c (lib.rs updated to use new module layout)
=======
use godot::classes::RandomNumberGenerator;
>>>>>>> cfd1976 (revert and add dmg to gitignore)

mod player;

pub fn _ready() {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.randomize();
    godot_print!("{:#?}", rng.randf_range(-10.0, 10.0));
}
