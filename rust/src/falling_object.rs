use godot::classes::IRigidBody2D;
use godot::classes::RigidBody2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
#[export]
pub struct FallingObject {
    base: Base<RigidBody2D>,
}
#[godot_api]
impl FallingObject {
    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut()
            .set_constant_force(Vector2 { x: 1.0, y: 1.0 });
        self.base_mut().show();
    }
}

#[godot_api]
impl IRigidBody2D for FallingObject {
    fn init(base: Base<RigidBody2D>) -> Self {
        FallingObject { base }
    }
    fn process(&mut self, delta: f64) {
        let mut current_force = self.base().get_constant_force().clone();
        current_force.x += 5.0;
        current_force.y += 5.0;
        self.base_mut().set_constant_force(current_force);
        self.base_mut().set_gravity_scale(0.0);
        self.base_mut().set_contact_monitor(true);
        self.base_mut().set_max_contacts_reported(1);
    }
}
