use godot::engine::{RenderingServer, Texture2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct BulletMLStyle {
    #[base]
    base: Base<Resource>,

    #[export]
    texture: Option<Gd<Texture2D>>,
}

#[godot_api]
impl BulletMLStyle {
    #[func]
    pub fn apply(&self, canvas_item: Rid) {
        self.apply_base(canvas_item);
    }

    #[func]
    fn apply_base(&self, canvas_item: Rid) {
        let mut rs = RenderingServer::singleton();
        rs.canvas_item_clear(canvas_item);
        rs.canvas_item_add_rect(canvas_item, Rect2::new(Vector2::ZERO, Vector2::ONE), Color::WHITE);
        if let Some(texture) = &self.texture {
            rs.canvas_item_add_texture_rect(canvas_item, Rect2::new(Vector2::ZERO, texture.get_size()), texture.get_rid());
        }
    }
}

#[godot_api]
impl IResource for BulletMLStyle {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            texture: None,
        }
    }
}
