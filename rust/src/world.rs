use godot::engine::{CanvasLayer, INode2D, Node2D};
use godot::prelude::*;

use crate::hearts_container::HeartsContainer;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct World {
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl World {
    #[func]
    fn on_player_hit(&mut self, health: i32) {
        let gui = self.base.get_node_as::<CanvasLayer>("Gui");
        let mut hearts_container = gui.get_node_as::<HeartsContainer>("HeartsContainer");
        let mut hearts_container = hearts_container.bind_mut();
        hearts_container.update_hearts(health);
    }
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Self::Base>) -> Self {
        World { base }
    }

    fn ready(&mut self) {
        let gui = self.base.get_node_as::<CanvasLayer>("Gui");
        let mut hearts_container = gui.get_node_as::<HeartsContainer>("HeartsContainer");
        let mut hearts_container = hearts_container.bind_mut();

        let player = self.base.get_node_as::<Player>("Player");

        hearts_container.set_max_hearts(player.bind().max_health);
    }
}
