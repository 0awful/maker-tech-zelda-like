use godot::engine::{Control, IControl, Sprite2D};
use godot::prelude::*;

use crate::resources::inventory_item::InventoryItem;

#[derive(GodotClass)]
#[class(base = Control)]
pub struct SlotGui {
    #[base]
    base: Base<Control>,
}

const ITEM_SPRITE_PATH: &str = "CenterContainer/Panel/Item";
const BACKGROUND_SPRITE_PATH: &str = "Background";

#[godot_api]
impl SlotGui {
    #[func]
    pub fn update(&mut self, item: Option<Gd<InventoryItem>>) {
        if let Some(item) = item {
            let mut background = self.base.get_node_as::<Sprite2D>(BACKGROUND_SPRITE_PATH);
            background.set_frame(1);
            let mut item_sprite = self.base.get_node_as::<Sprite2D>(ITEM_SPRITE_PATH);
            item_sprite.set_visible(true);
            item_sprite.set_texture(item.bind().get_texture());
        } else {
            let mut background = self.base.get_node_as::<Sprite2D>(BACKGROUND_SPRITE_PATH);
            background.set_frame(0);
            let mut item_sprite = self.base.get_node_as::<Sprite2D>(ITEM_SPRITE_PATH);
            item_sprite.set_visible(false);
        }
    }
}

#[godot_api]
impl IControl for SlotGui {
    fn init(base: Base<Self::Base>) -> Self {
        SlotGui { base }
    }
}
