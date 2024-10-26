use godot::prelude::*;

mod falling_object;
mod player;
mod spawn_falling_object;

struct DemoGame;

#[gdextension]
unsafe impl ExtensionLibrary for DemoGame {}
