use godot::engine::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Marker2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Slime {
    #[base]
    base: Base<CharacterBody2D>,
    start_position: Vector2,
    end_position: Vector2,
    speed: real,
    limit: real,
    #[export]
    marker: Option<Gd<Marker2D>>,
}

#[godot_api]
impl Slime {
    #[func]
    pub fn update_velocity(&mut self) {
        let move_direction = self.end_position - self.base.get_position();
        if move_direction.length() < self.limit {
            self.update_direction();
        }
        self.base
            .set_velocity(move_direction.normalized() * self.speed);
    }

    #[func]
    pub fn update_direction(&mut self) {
        let temp = self.end_position;
        self.end_position = self.start_position;
        self.start_position = temp;
    }

    #[func]
    pub fn update_animation(&mut self) {
        let mut animation_string = "walkUp";
        if self.base.get_velocity().y > 0. {
            animation_string = "walkDown";
        } else if self.base.get_velocity().y < 0. {
            animation_string = "walkUp";
        } else if self.base.get_velocity().x > 0. {
            animation_string = "walkRight";
        } else if self.base.get_velocity().x < 0. {
            animation_string = "walkLeft";
        }
        let mut animation_player = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        animation_player.set_animation(animation_string.into());
        animation_player.play();
    }
}

#[godot_api]
impl ICharacterBody2D for Slime {
    fn init(base: Base<Self::Base>) -> Self {
        Slime {
            base,
            start_position: Vector2::ZERO,
            end_position: Vector2::ZERO,
            speed: 36.,
            limit: 0.5,
            marker: None,
        }
    }

    fn ready(&mut self) {
        self.start_position = self.base.get_position();
        if let Some(marker) = &self.marker {
            self.end_position = marker.get_global_position();
        } else {
            self.end_position = self.base.get_position() + Vector2 { x: 0., y: 3. * 16. }
        }
    }

    fn physics_process(&mut self, _: f64) {
        self.update_velocity();
        self.base.move_and_slide();
        self.update_animation();
    }
}
