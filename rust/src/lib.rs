use godot::prelude::*;

mod collectable;
mod follow_cam;
mod gui;
mod heart_gui;
mod hearts_container;
mod inventory_gui;
mod inventory_item;
mod player;
mod player_inventory;
mod slime;
mod slot_gui;
mod sword;
mod world;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
