use godot::engine::{AnimationPlayer, Area2D};
use godot::prelude::*;

use crate::collectable::collect;
use crate::inventory_item::InventoryItem;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct Sword {
    #[base]
    base: Base<Area2D>,
    #[export]
    item: Option<Gd<InventoryItem>>,
}

#[godot_api]
impl Sword {
    #[func]
    fn collect(&mut self) {
        let mut animation_player = self.base.get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation_player.play_ex().name("collect".into()).done();

        let mut timer = self.base.get_tree().unwrap().create_timer(0.2).unwrap();
        timer.connect("timeout".into(), self.base.callable("collect_callback"));
    }
    #[func]
    fn collect_callback(&mut self) {
        collect(self.base.clone())
    }
}
