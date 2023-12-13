use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

use crate::collectable::collect_it;

#[derive(GodotClass)]
#[class(base = Area2D)]
pub struct Sword {
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl Sword {
    #[func]
    fn collect(&mut self) {
        collect_it(self.base.clone())
    }
}

#[godot_api]
impl IArea2D for Sword {
    fn init(base: Base<Self::Base>) -> Self {
        Sword { base }
    }
}
