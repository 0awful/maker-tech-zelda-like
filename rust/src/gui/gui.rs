use godot::engine::{CanvasLayer, Control, ICanvasLayer, InputEvent};
use godot::prelude::*;

use crate::gui::inventory_gui::InventoryGui;

#[derive(GodotClass)]
#[class(init, base = CanvasLayer)]
pub struct Gui {
    #[base]
    base: Base<CanvasLayer>,
}

#[godot_api]
impl ICanvasLayer for Gui {
    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("toggle_inventory".into()) {
            let inventory_gui = self.base.get_node_as::<Control>("InventoryGui");
            if let Ok(mut inventory_gui) = inventory_gui.try_cast::<InventoryGui>() {
                inventory_gui.bind_mut().toggle();
            } else {
                godot_error!("could not cast inventory_gui to InventoryGui type");
            }
        }
    }
    fn ready(&mut self) {
        let inventory_gui = self.base.get_node_as::<Control>("InventoryGui");
        if let Ok(mut inventory_gui) = inventory_gui.try_cast::<InventoryGui>() {
            inventory_gui.bind_mut().close();
        } else {
            godot_error!("could not cast inventory_gui to InventoryGui type");
        }
    }
}
