use godot::prelude::*;

mod follow_cam;
mod player;
mod world;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
