use godot::engine::Resource;
use godot::prelude::*;

use crate::resources::inventory_item::InventoryItem;
use crate::resources::inventory_slot::InventorySlot;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct PlayerInventory {
    #[base]
    base: Base<Resource>,
    #[export]
    pub slots: Array<Gd<InventorySlot>>,
}

#[godot_api]
impl PlayerInventory {
    #[signal]
    pub fn updated();
    #[func]
    pub fn insert(&mut self, item: Gd<InventoryItem>) {
        for mut slot in self.slots.iter_shared() {
            let mut slot = slot.bind_mut();
            if slot.item == Some(item.clone()) {
                if slot.amount < item.bind().get_max_amount() {
                    slot.amount += 1;
                    self.base.emit_signal("updated".into(), &[]);
                    return;
                }
            }
        }

        for i in 0..self.slots.len() {
            if self.slots.get(i).bind().item == None {
                let mut slot = self.slots.get(i);
                {
                    let mut slot = slot.bind_mut();
                    slot.set_item(Some(item));
                    slot.set_amount(1);
                }
                self.slots.set(i, slot);
                self.base.emit_signal("updated".into(), &[]);
                return;
            }
        }
    }
}
