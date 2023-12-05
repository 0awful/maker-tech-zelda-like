use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Player {
    #[base]
    base: Base<CharacterBody2D>,
    #[export]
    speed: real,
}

#[godot_api]
impl Player {
    #[func]
    pub fn handle_input(&mut self) {
        let input = Input::singleton();
        let movement_vec = input.get_vector(
            "ui_left".into(),
            "ui_right".into(),
            "ui_up".into(),
            "ui_down".into(),
        );
        self.base.set_velocity(movement_vec * self.speed)
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Player {
            base,
            speed: 35.0.into(),
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        self.handle_input();
        self.base.move_and_slide();
    }
}
