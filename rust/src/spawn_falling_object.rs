use godot::classes::INode2D;
use godot::classes::InputEvent;
use godot::classes::InputEventMouseButton;
use godot::classes::Node2D;
use godot::global::MouseButton;
use godot::prelude::*;

use crate::falling_object::FallingObject;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct SpawnFallingObject {
    falling_object_scene: Gd<PackedScene>,
    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for SpawnFallingObject {
    fn init(base: Base<Node2D>) -> Self {
        SpawnFallingObject {
            falling_object_scene: PackedScene::new_gd(),
            base,
        }
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(v) = event.try_cast::<InputEventMouseButton>() {
            if v.get_button_index() == MouseButton::LEFT && v.is_pressed() {
                self.spawn(self.base().get_local_mouse_position())
            }
        }
    }
    fn ready(&mut self) {
        // godot_print!("Trying to spawn object");
        self.falling_object_scene = load("res://falling_object.tscn");
    }
}
impl SpawnFallingObject {
    fn spawn(&mut self, spawn_global_position: Vector2) {
        let mut instance = self.falling_object_scene.instantiate_as::<FallingObject>();
        instance.set_global_position(spawn_global_position);
        self.base_mut().add_child(&instance);
        // godot_print!("Trying to spawn object");
    }
}
