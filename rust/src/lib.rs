use godot::prelude::*;

mod collectable;
mod follow_cam;
mod gui;
mod hearts_container;
mod player;
mod resources;
mod slime;
mod world;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
