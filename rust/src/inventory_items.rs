use godot::engine::{Resource, Texture2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct InventoryItem {
    #[base]
    base: Base<Resource>,
    #[export]
    name: GString,
    #[export]
    texture: Gd<Texture2D>,
}

#[godot_api]
impl InventoryItem {}
