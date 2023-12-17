use godot::engine::{Control, GridContainer, IControl};
use godot::prelude::*;

use crate::gui::slot_gui::SlotGui;
use crate::resources::player_inventory::PlayerInventory;

const GRID_CONTAINER_PATH: &str = "NinePatchRect/GridContainer";

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
    #[func]
    pub fn update(&mut self) {
        let slots = self
            .base
            .get_node_as::<GridContainer>(GRID_CONTAINER_PATH)
            .get_children();
        for i in 0..slots.len() {
            if let Ok(mut slot) = slots.get(i).try_cast::<SlotGui>() {
                let item = self
                    .inventory
                    .clone()
                    .expect("No inventory in update call, big error")
                    .bind()
                    .slots
                    .get(i);
                slot.bind_mut().update(item);
            } else {
                godot_error!(
                    "Unable to cast grid container slot to SlotGui, are you using the right nodes?"
                )
            }
        }
    }
    pub fn connect_slots(&mut self) {
        let slots = self
            .base
            .get_node_as::<GridContainer>(GRID_CONTAINER_PATH)
            .get_children();
        for i in 0..slots.len() {
            if let Ok(mut slot) = slots.get(i).try_cast::<SlotGui>() {
                slot.connect("pressed".into(), self.base.callable("on_slot_clicked"));
            } else {
                godot_error!(
                    "Unable to cast grid container slot to SlotGui, are you using the right nodes?"
                )
            }
        }
    }

    #[func]
    pub fn on_slot_clicked(&mut self, slot: Gd<SlotGui>) {
        godot_print!("And we clicked this slot: {:?}", slot);
    }
}

#[godot_api]
impl IControl for InventoryGui {
    fn ready(&mut self) {
        if let Some(mut inventory) = try_load::<PlayerInventory>("res://player_inventory.tres") {
            inventory
                .connect_ex("updated".into(), self.base.callable("update"))
                .flags(1)
                .done();
            self.inventory = Some(inventory);
            self.update();
        } else {
            godot_error!(
                "Error in inventory_gui, could not load player inventory as PlayerInventory"
            );
        }
        self.connect_slots();
    }
}
