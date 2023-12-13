use godot::engine::{Control, IControl};
use godot::prelude::*;

use crate::player_inventory::PlayerInventory;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct InventoryGui {
    #[base]
    base: Base<Control>,
    #[init(default = false)]
    pub is_open: bool,
    inventory: Option<Gd<PlayerInventory>>,
}

#[godot_api]
impl InventoryGui {
    #[signal]
    fn opened();
    #[signal]
    fn closed();

    #[func]
    pub fn open(&mut self) {
        self.base.set_visible(true);
        self.is_open = true;
        self.base.emit_signal("opened".into(), &[]);
    }
    #[func]
    pub fn close(&mut self) {
        self.is_open = false;
        self.base.set_visible(false);
        self.base.emit_signal("closed".into(), &[]);
    }
    #[func]
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        self.base.set_visible(self.is_open);
        if self.is_open {
            self.base.emit_signal("opened".into(), &[]);
        } else {
            self.base.emit_signal("closed".into(), &[]);
        }
    }
}

#[godot_api]
impl IControl for InventoryGui {
    fn ready(&mut self) {
        if let Some(inventory) = try_load::<PlayerInventory>("res://player_inventory.tres") {
            self.inventory = Some(inventory);
        } else {
            godot_error!(
                "Error in inventory_gui, could not load player inventory as PlayerInventory"
            );
        }
    }
}
