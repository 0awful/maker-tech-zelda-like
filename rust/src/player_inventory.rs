use godot::engine::{IResource, Resource};
use godot::prelude::*;

use crate::inventory_items::InventoryItem;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct PlayerInventory {
    #[base]
    base: Base<Resource>,
    #[export]
    items: Array<Gd<InventoryItem>>,
}

#[godot_api]
impl PlayerInventory {}
