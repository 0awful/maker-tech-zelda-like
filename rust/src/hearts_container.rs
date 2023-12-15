use godot::engine::{Control, HBoxContainer, IHBoxContainer};
use godot::prelude::*;

use crate::gui::heart_gui::HeartGui;

#[derive(GodotClass)]
#[class(base = HBoxContainer)]
pub struct HeartsContainer {
    #[base]
    base: Base<HBoxContainer>,
    heart_gui: Gd<PackedScene>,
}

#[godot_api]
impl HeartsContainer {
    #[func]
    pub fn set_max_hearts(&mut self, max: i32) {
        for _ in 0..max {
            let heart = self.heart_gui.instantiate_as::<Control>();
            self.base.add_child(heart.clone().upcast());
        }
    }

    #[func]
    pub fn update_hearts(&mut self, current_hearts: i32) {
        let hearts = self.base.get_children();

        for i in 0..current_hearts {
            let heart = hearts.get(i as usize);
            if let Ok(mut typed) = heart.try_cast::<HeartGui>() {
                typed.bind_mut().update_whole(true);
            }
        }
        for i in current_hearts..self.base.get_child_count() {
            let heart = hearts.get(i as usize);
            if let Ok(mut typed) = heart.try_cast::<HeartGui>() {
                typed.bind_mut().update_whole(false);
            }
        }
    }
}

#[godot_api]
impl IHBoxContainer for HeartsContainer {
    fn init(base: Base<Self::Base>) -> Self {
        HeartsContainer {
            base,
            heart_gui: PackedScene::new(),
        }
    }
    fn ready(&mut self) {
        self.heart_gui = load("scenes/heart_gui.tscn");
    }
}
