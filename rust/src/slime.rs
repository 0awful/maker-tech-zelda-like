use godot::engine::{AnimationPlayer, Area2D, CharacterBody2D, ICharacterBody2D, Marker2D};
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
    dead: bool,
    #[export]
    marker: Option<Gd<Marker2D>>,
}

#[godot_api]
impl Slime {
    #[func]
    pub fn hurt_box_entered(&mut self, area: Gd<Area2D>) {
        if let Some(parent) = area.get_parent() {
            if parent.is_in_group("mobs".into()) {
                return;
            }
        }
        self.base
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
            .play_ex()
            .name("despawn".into())
            .done();
        self.set_dead();
    }

    #[func]
    pub fn set_dead(&mut self) {
        self.dead = true;
        self.base.get_node_as::<Area2D>("HitBox").hide();
        self.base.get_node_as::<Area2D>("HurtBox").hide();

        let mut timer = self.base.get_tree().unwrap().create_timer(0.5).unwrap();
        timer.connect("timeout".into(), self.base.callable("despawn"));
    }

    #[func]
    pub fn despawn(&mut self) {
        self.base.queue_free();
    }

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
        let mut animation_player = self.base.get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation_player.set_current_animation(animation_string.into());
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
            dead: false,
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
        if self.dead {
            return;
        }
        self.update_velocity();
        self.base.move_and_slide();
        self.update_animation();
    }
}
