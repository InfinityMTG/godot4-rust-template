use godot::classes::INode2D;
use godot::classes::InputEvent;
use godot::classes::InputEventMouseButton;
use godot::classes::Node2D;
use godot::classes::RigidBody2D;
use godot::classes::RigidBody3D;
use godot::global::MouseButton;
use godot::prelude::*;

use crate::falling_object::FallingObject;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct SpawnFallingObject {
    falling_object_scene: Gd<PackedScene>,
    rigid_body_scene: Gd<PackedScene>,
    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for SpawnFallingObject {
    fn init(base: Base<Node2D>) -> Self {
        SpawnFallingObject {
            falling_object_scene: PackedScene::new_gd(),
            rigid_body_scene: PackedScene::new_gd(),
            base,
        }
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(v) = event.try_cast::<InputEventMouseButton>() {
            if v.get_button_index() == MouseButton::LEFT && v.is_pressed() {
                self.spawn(self.base().get_local_mouse_position());
            }
            if v.get_button_index() == MouseButton::RIGHT && v.is_pressed() {
                self.spawn_rigid_body_scene(self.base().get_local_mouse_position());
            }
        }
    }
    fn process(&mut self, delta: f64) {
        for o in self.base_mut().get_children().iter_shared() {
            if let Ok(mut f) = o.clone().try_cast::<FallingObject>() {
                let pos = f.get_global_position();
                if pos.x > 2560.0 || pos.y > 1440.0 {
                    f.queue_free();
                }
            } else if let Ok(mut r) = o.try_cast::<RigidBody2D>() {
                let pos = r.get_global_position();
                if pos.x > 2560.0 || pos.y > 1440.0 {
                    r.queue_free();
                }
            }
        }
    }
    fn ready(&mut self) {
        // godot_print!("Trying to spawn object");
        self.falling_object_scene = load("res://falling_object.tscn");
        self.rigid_body_scene = load("res://rigid_body_test.tscn");
        // godot_print!("Loaded self.rigid_body_scene");
    }
}
impl SpawnFallingObject {
    fn spawn(&mut self, spawn_global_position: Vector2) {
        // godot_print!("{:#?}", self.base_mut().get_children());
        let mut instance = self.falling_object_scene.instantiate_as::<FallingObject>();
        instance.set_global_position(spawn_global_position);
        self.base_mut().add_child(&instance);
    }
    fn spawn_rigid_body_scene(&mut self, spawn_global_position: Vector2) {
        // godot_print!("Trying to spawn object");
        let mut instance = self.rigid_body_scene.instantiate_as::<RigidBody2D>();
        instance.set_global_position(spawn_global_position);
        self.base_mut().add_child(&instance);
    }
}
