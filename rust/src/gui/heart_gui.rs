use godot::engine::{Control, IControl, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Control)]
pub struct HeartGui {
    #[base]
    base: Base<Control>,
}

#[godot_api]
impl HeartGui {
    #[func]
    pub fn update_whole(&mut self, whole: bool) {
        let mut sprite = self.base.get_node_as::<Sprite2D>("Sprite");
        if whole {
            sprite.set_frame(0)
        } else {
            sprite.set_frame(4)
        }
    }
}

#[godot_api]
impl IControl for HeartGui {
    fn init(base: Base<Self::Base>) -> Self {
        HeartGui { base }
    }
}
