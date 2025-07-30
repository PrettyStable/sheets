use godot::prelude::*;
use godot::classes::Resource;

#[derive(GodotClass)]
#[class(base=Resource)]
struct SheetStyle {
    /// Size as percentage
    #[export]
    size: Vector2,
    /// Position as percentage
    #[export]
    position: Vector2,
    #[export]
    background_color: Color,
    #[export]
    disabled: bool,
    base: Base<Resource>,
}

use godot::classes::IResource;

#[godot_api]
impl IResource for SheetStyle {
    fn init(base: Base<Resource>) -> Self {
        Self {
            size: Vector2::new(1.0, 1.0),
            position: Vector2::new(0.0, 0.0),
            background_color: Color::BLACK,
            disabled: false,
            base,
        }
    }
}

impl SheetStyle {}