use godot::classes::ISprite2D;
use godot::classes::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
#[export]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}
#[godot_api]
impl Player {
    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();
    }
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        // godot_print!("Hello, World!"); // Prints to the Godot console

        Player {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }
    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32)
    }
}
