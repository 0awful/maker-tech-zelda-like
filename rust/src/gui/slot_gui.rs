use godot::engine::{Control, IControl, Label, Sprite2D};
use godot::prelude::*;

use crate::resources::inventory_slot::InventorySlot;

#[derive(GodotClass)]
#[class(base = Control)]
pub struct SlotGui {
    #[base]
    base: Base<Control>,
}

const ITEM_SPRITE_PATH: &str = "CenterContainer/Panel/Item";
const BACKGROUND_SPRITE_PATH: &str = "Background";
const LABEL_PATH: &str = "CenterContainer/Panel/Label";

#[godot_api]
impl SlotGui {
    #[func]
    pub fn update(&mut self, slot: Gd<InventorySlot>) {
        if let Some(item) = &slot.bind().item {
            let mut background = self.base.get_node_as::<Sprite2D>(BACKGROUND_SPRITE_PATH);
            background.set_frame(1);
            let mut item_sprite = self.base.get_node_as::<Sprite2D>(ITEM_SPRITE_PATH);
            item_sprite.set_visible(true);
            item_sprite.set_texture(item.bind().get_texture());

            if slot.bind().get_amount() > 1 {
                let mut label = self.base.get_node_as::<Label>(LABEL_PATH);
                label.set_text(slot.bind().get_amount().to_string().into());
            }
        } else {
            let mut background = self.base.get_node_as::<Sprite2D>(BACKGROUND_SPRITE_PATH);
            background.set_frame(0);
            let mut item_sprite = self.base.get_node_as::<Sprite2D>(ITEM_SPRITE_PATH);
            item_sprite.set_visible(false);
            let mut label = self.base.get_node_as::<Label>(LABEL_PATH);
            label.set_text("".into());
        }
    }
}

#[godot_api]
impl IControl for SlotGui {
    fn init(base: Base<Self::Base>) -> Self {
        SlotGui { base }
    }
}
