use godot::engine::{AnimationPlayer, Area2D, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::slime::Slime;

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Player {
    #[base]
    base: Base<CharacterBody2D>,
    #[export]
    speed: real,
    pub current_health: i32,
    pub max_health: i32,
    pub knockback_power: real,
}

#[godot_api]
impl Player {
    #[signal]
    pub fn hit(val: i32);

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
    //    #[func]
    //   pub fn handle_collision(&mut self) {
    //      for collision_index in 1..self.base.get_slide_collision_count() {
    //         if let Some(collision) = self.base.get_slide_collision(collision_index) {
    //            let collider = collision.get_collider();
    //       }
    //  }
    //}

    #[func]
    pub fn on_player_hit(&mut self, area: Gd<Area2D>) {
        if area.get_name() == "HitBox".into() {
            self.current_health -= 1;
            self.base
                .emit_signal("hit".into(), &[Variant::from(self.current_health)]);
            if let Some(parent) = area.get_parent() {
                if let Ok(parent) = parent.try_cast::<Slime>() {
                    self.knockback(parent.get_velocity());
                } else {
                    godot_error!("Hitbox does not belong to slime");
                }
            } else {
                godot_error!("Hitbox does not have a parent");
            }
        }
    }

    #[func]
    pub fn knockback(&mut self, enemy_velocity: Vector2) {
        let knockback_direction =
            (enemy_velocity - self.base.get_velocity()).normalized() * self.knockback_power;
        self.base.set_velocity(knockback_direction);
        self.base.move_and_slide();
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Player {
            base,
            speed: 35.0.into(),
            current_health: 3,
            max_health: 3,
            knockback_power: 500.,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        self.handle_input();
        self.update_animation();
        //self.handle_collision();
        self.base.move_and_slide();
    }
}
