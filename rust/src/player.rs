use godot::engine::{AnimationPlayer, CharacterBody2D, ICharacterBody2D};
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

    #[func]
    pub fn update_animation(&mut self) {
        let mut animation_player = self.base.get_node_as::<AnimationPlayer>("AnimationPlayer");
        if self.base.get_velocity().length_squared() == 0.0 {
            return animation_player.stop();
        }

        let mut animation_name = "walkDown";
        if self.base.get_velocity().x < 0.0 {
            animation_name = "walkLeft";
        }
        if self.base.get_velocity().x > 0.0 {
            animation_name = "walkRight";
        }
        if self.base.get_velocity().y < 0.0 {
            animation_name = "walkUp";
        }

        animation_player
            .play_ex()
            .name(animation_name.into())
            .done();
    }

    #[func]
    pub fn handle_collision(&mut self) {
        for collision_index in 1..self.base.get_slide_collision_count() {
            if let Some(collision) = self.base.get_slide_collision(collision_index) {
                let collider = collision.get_collider();
                godot_print!("{:?}", collider);
            }
        }
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
        self.update_animation();
        self.handle_collision();
        self.base.move_and_slide();
    }
}
