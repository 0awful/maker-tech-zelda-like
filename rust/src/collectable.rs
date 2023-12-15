use godot::engine::Area2D;
use godot::prelude::*;

use crate::inventory_item::InventoryItem;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct Collectable {
    #[base]
    base: Base<Area2D>,
    #[export]
    item: Option<Gd<InventoryItem>>,
}

pub fn collect<T>(node: Gd<T>)
where
    T: Inherits<Node>,
{
    node.upcast().queue_free();
}

#[godot_api]
impl Collectable {
    #[func]
    pub fn collect(&mut self) {
        collect(self.base.clone());
    }
}
