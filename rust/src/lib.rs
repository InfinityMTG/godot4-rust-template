use godot::prelude::*;

mod player;

struct DemoGame;

#[gdextension]
unsafe impl ExtensionLibrary for DemoGame {}
