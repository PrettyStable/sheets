use godot::prelude::*;
use godot::classes::Control;

#[derive(GodotClass)]
#[class(base=Control)]
struct Sheet {
    base: Base<Control>,
}

use godot::classes::IControl;
#[godot_api]
impl IControl for Sheet {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
        }
    }
}

#[godot_api]
impl Sheet {
}