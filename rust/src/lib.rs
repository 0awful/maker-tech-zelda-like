use godot::prelude::*;

mod collectable;
mod follow_cam;
mod gui;
mod heart_gui;
mod hearts_container;
mod inventory_gui;
mod player;
mod slime;
mod sword;
mod world;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
