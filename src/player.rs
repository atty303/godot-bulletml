use godot::prelude::*;

use crate::canvas::BulletMLCanvas;
use crate::resource::BulletMLResource;
use crate::pool::PoolActorRef;
use crate::style::BulletMLStyle;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BulletMLPlayer {
    #[base]
    base: Base<Node2D>,

    #[export]
    canvas: Option<Gd<BulletMLCanvas>>,

    #[export]
    bulletml: Option<Gd<BulletMLResource>>,

    #[export]
    style: Option<Gd<BulletMLStyle>>,

    is_playing: bool,
    rank: f64,

    top_bullet_ref: Option<PoolActorRef>,
}

#[godot_api]
impl BulletMLPlayer {
    #[func]
    fn play(&mut self) {
        let player = self.to_gd();
        match (&mut self.canvas, &self.bulletml) {
            (Some(ref mut canvas), Some(bulletml)) => {
                self.is_playing = true;
                let style = self.style.clone().unwrap_or(BulletMLStyle::new_gd());
                self.top_bullet_ref = canvas.bind_mut().create_bullet_new(player, bulletml.bind().bml.clone(), style);
            }
            _ => {},
        }
    }

    #[func]
    fn stop(&mut self) {
        self.is_playing = false;
    }

    #[func]
    fn reset(&mut self) {
        self.top_bullet_ref = None;
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
            canvas: None,
            bulletml: None,
            style: None,
            is_playing: false,
            rank: 1.0,
            top_bullet_ref: None,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if let (Some(ref mut canvas), Some(top_bullet_ref)) = (&mut self.canvas, &self.top_bullet_ref) {
            canvas.bind_mut().maybe_index_mut(*top_bullet_ref).map(|bullet| {
                bullet.bind_mut().set_transform(Transform2D::IDENTITY.translated(self.base.get_position()));
            });
        }
    }
}
