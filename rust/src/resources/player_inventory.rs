use godot::engine::Resource;
use godot::prelude::*;

use crate::resources::inventory_item::InventoryItem;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct PlayerInventory {
    #[base]
    base: Base<Resource>,
    #[export]
    pub items: Array<Option<Gd<InventoryItem>>>,
}

#[godot_api]
impl PlayerInventory {
    #[signal]
    pub fn updated();
    #[func]
    pub fn insert(&mut self, item: Gd<InventoryItem>) {
        for i in 0..self.items.len() {
            if self.items.get(i) == None {
                self.items.set(i, Some(item.clone()));
                break;
            }
        }
        self.base.emit_signal("updated".into(), &[]);
    }
}
