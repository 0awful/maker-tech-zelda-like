use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct World {
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl World {}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Self::Base>) -> Self {
        World { base }
    }
}
