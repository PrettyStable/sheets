use godot::prelude::*;
mod sheet;
mod sheet_style;

struct Sheets;

#[gdextension]
unsafe impl ExtensionLibrary for Sheets {}
