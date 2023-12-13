use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Area2D)]
pub struct Collectable {
    #[base]
    base: Base<Area2D>,
}

pub fn collect_it<T>(node: Gd<T>)
where
    T: Inherits<Node>,
{
    node.upcast().queue_free();
}

#[godot_api]
impl Collectable {
    #[func]
    pub fn collect(&mut self) {
        collect_it(self.base.clone());
    }
}

#[godot_api]
impl IArea2D for Collectable {
    fn init(base: Base<Self::Base>) -> Self {
        Collectable { base }
    }
}
