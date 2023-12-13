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
    invincible: bool,
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

    pub fn hurt_by_enemy(&mut self, area: Gd<Area2D>) {
        self.current_health -= 1;

        if self.current_health < 0 {
            self.current_health = self.max_health;
        }

        self.invincible = true;
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
        self.base
            .get_node_as::<AnimationPlayer>("Effects")
            .play_ex()
            .name("HurtBlink".into())
            .done();
        let mut timer = self.base.get_tree().unwrap().create_timer(2.0).unwrap();
        timer.connect("timeout".into(), self.base.callable("end_invincibility"));
    }

    #[func]
    pub fn end_invincibility(&mut self) {
        self.base
            .get_node_as::<AnimationPlayer>("Effects")
            .play_ex()
            .name("RESET".into())
            .done();
        self.invincible = false
    }

    #[func]
    pub fn knockback(&mut self, enemy_velocity: Vector2) {
        let knockback_direction =
            (enemy_velocity - self.base.get_velocity()).normalized() * self.knockback_power;
        self.base.set_velocity(knockback_direction);
        self.base.move_and_slide();
    }

    #[func]
    pub fn collided_with_object(&mut self, mut area: Gd<Area2D>) {
        if area.has_method("collect".into()) {
            area.call("collect".into(), &[]);
        }
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
            invincible: false,
        }
    }

    fn ready(&mut self) {
        self.base
            .get_node_as::<AnimationPlayer>("Effects")
            .play_ex()
            .name("RESET".into())
            .done()
    }

    fn physics_process(&mut self, _delta: f64) {
        self.handle_input();
        self.update_animation();
        self.base.move_and_slide();
        if !self.invincible {
            let hurtbox = self.base.get_node_as::<Area2D>("HurtBox");
            for area in hurtbox.get_overlapping_areas().iter_shared() {
                if area.get_name() == "hitbox".into() {
                    self.hurt_by_enemy(area);
                }
            }
        }
    }
}
