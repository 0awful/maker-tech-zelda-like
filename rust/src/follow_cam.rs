use godot::engine::global::Side;
use godot::engine::{Camera2D, ICamera2D, TileMap};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Camera2D)]
pub struct FollowCam {
    #[base]
    base: Base<Camera2D>,
    #[export]
    tilemap: Option<Gd<TileMap>>,
}

#[godot_api]
impl FollowCam {}

#[godot_api]
impl ICamera2D for FollowCam {
    fn init(base: Base<Self::Base>) -> Self {
        FollowCam {
            base,
            tilemap: None,
        }
    }

    fn ready(&mut self) {
        if let Some(tilemap) = &self.tilemap {
            let map_rect = tilemap.get_used_rect();
            let tile_size = tilemap.get_rendering_quadrant_size();
            let world_size_in_px = map_rect.size * tile_size;
            let offset = -2 * tile_size;
            self.base
                .set_limit(Side::SIDE_RIGHT, world_size_in_px.x + offset);
            self.base
                .set_limit(Side::SIDE_BOTTOM, world_size_in_px.y + offset);
            self.base.set_limit(Side::SIDE_LEFT, offset);
            self.base.set_limit(Side::SIDE_TOP, offset);
        } else {
            println!("Error In Followcam: Unset Tilemap");
            println!("Continuing Execution");
        }
    }
}
