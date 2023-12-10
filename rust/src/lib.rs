use godot::prelude::*;

mod follow_cam;
mod heart_gui;
mod hearts_container;
mod player;
mod slime;
mod world;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
