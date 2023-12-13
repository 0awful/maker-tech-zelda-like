use godot::prelude::*;

mod collectable;
mod follow_cam;
mod heart_gui;
mod hearts_container;
mod player;
mod slime;
mod sword;
mod world;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
