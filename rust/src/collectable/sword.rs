use godot::engine::{AnimationPlayer, Area2D};
use godot::prelude::*;

use crate::collectable::collectable::collect;
use crate::resources::inventory_item::InventoryItem;
use crate::resources::player_inventory::PlayerInventory;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct Sword {
    #[base]
    base: Base<Area2D>,
    #[export]
    item: Option<Gd<InventoryItem>>,
    inventory: Option<Gd<PlayerInventory>>,
}

#[godot_api]
impl Sword {
    #[func]
    fn collect(&mut self, inventory: Gd<PlayerInventory>) {
        let mut animation_player = self.base.get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation_player.play_ex().name("collect".into()).done();

        self.inventory = Some(inventory);
        let mut timer = self.base.get_tree().unwrap().create_timer(0.2).unwrap();
        timer.connect("timeout".into(), self.base.callable("collect_callback"));
    }
    #[func]
    fn collect_callback(&mut self) {
        if let Some(mut inventory) = self.inventory.clone() {
            inventory.call_deferred("insert".into(), &[Variant::from(self.item.clone())]);
            collect(self.base.clone());
        }
    }
}
