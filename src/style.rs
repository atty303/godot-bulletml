use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct BulletMLStyle {
    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl BulletMLStyle {
}

#[godot_api]
impl IResource for BulletMLStyle {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
        }
    }
}
