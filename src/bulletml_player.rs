use godot::prelude::*;

use crate::bulletml_canvas::BulletMLCanvas;
use crate::bulletml_resource::BulletMLResource;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BulletMLPlayer {
    #[base]
    base: Base<Node2D>,

    #[export]
    node: Option<Gd<BulletMLCanvas>>,

    #[export]
    bulletml: Option<Gd<BulletMLResource>>,

    is_playing: bool,
    rank: f64,
}

#[godot_api]
impl BulletMLPlayer {
    #[func]
    fn play(&mut self) {
        let player = self.to_gd();
        match (&mut self.node, &self.bulletml) {
            (Some(ref mut node), Some(bulletml)) => {
                self.is_playing = true;
                node.bind_mut().create_bullet_new(player, bulletml.bind().bml.clone());
            },
            _ => {},
        }
    }

    #[func]
    fn stop(&mut self) {
        self.is_playing = false;
    }

    #[func]
    fn reset(&mut self) {
    }

    #[func]
    fn is_playing(&self) -> bool {
        self.is_playing
    }

    #[func]
    pub fn get_turn(&self) -> u32 {
        0
    }

    #[func]
    pub fn get_rank(&self) -> f64 {
        self.rank
    }
}

#[godot_api]
impl INode2D for BulletMLPlayer {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            node: None,
            bulletml: None,
            is_playing: false,
            rank: 1.0,
        }
    }
}
