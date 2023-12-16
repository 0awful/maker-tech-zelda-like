use godot::engine::Resource;
use godot::prelude::*;

use crate::resources::inventory_item::InventoryItem;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct InventorySlot {
    #[base]
    base: Base<Resource>,
    #[var]
    pub item: Option<Gd<InventoryItem>>,
    #[var]
    pub amount: i32,
}

#[godot_api]
impl InventorySlot {}
