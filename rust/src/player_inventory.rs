use godot::engine::Resource;
use godot::prelude::*;

use crate::inventory_item::InventoryItem;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct PlayerInventory {
    #[base]
    base: Base<Resource>,
    #[export]
    pub items: Array<Option<Gd<InventoryItem>>>,
}

#[godot_api]
impl PlayerInventory {}
