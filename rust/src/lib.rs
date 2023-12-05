use godot::prelude::*;

mod player;
mod world;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
