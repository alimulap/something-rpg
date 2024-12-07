use godot::prelude::*;

struct GDExt;

pub mod pg_tilemap;
pub mod tile;

#[gdextension]
unsafe impl ExtensionLibrary for GDExt {}
