use std::sync::Arc;

use godot::engine::{Engine, RenderingServer, Viewport};
use godot::prelude::*;

use crate::bulletml_bullet::BulletMLBullet;
use crate::bulletml_player::BulletMLPlayer;
use crate::pool::{Pool, PoolGetInstanceArea};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BulletMLCanvas {
    #[base]
    base: Base<Node>,

    runs_on_editor: bool,

    capacity: u32,

    pool: Pool<Bullet>,
    turn: u32,

    canvas_item: Rid,
}

#[godot_api]
impl BulletMLCanvas {
    #[func]
    fn create(&mut self, capacity: u32) {
        self.capacity = capacity;

        let mut viewport: Option<Gd<Viewport>> = None;
        {
            let mut n: Option<Gd<Node>> = Some(self.to_gd().upcast());
            while n.is_some() {
                if let Ok(v) = n.clone().unwrap().try_cast::<Viewport>() {
                    viewport = Some(v);
                    break;
                }
                n = n.unwrap().get_parent();
            }
        }

        if let Some(viewport) = viewport {
            let canvas_parent = viewport.find_world_2d().unwrap().get_canvas();

            self.canvas_item = RenderingServer::singleton().canvas_item_create();
            RenderingServer::singleton().canvas_item_set_parent(self.canvas_item, canvas_parent);
        }

        let mut rs = RenderingServer::singleton();
        let canvas_item = self.canvas_item;
        self.pool = Pool::new(capacity as usize, move || {
            let b = Bullet::default();
            rs.canvas_item_set_parent(b.bullet.bind().canvas_item_rid, canvas_item);
            b
        });
    }

    #[func]
    fn get_turn(&self) -> u32 {
        self.turn
    }

    #[func]
    fn get_count(&self) -> u32 {
        self.pool.get_num() as u32
    }

    pub fn create_bullet_new(&mut self, player: Gd<BulletMLPlayer>, bml: Arc<bulletml::BulletML>) {
        if let Some(actor) = self.pool.get_instance() {
            let mut bullet = actor.0.bullet.bind_mut();
            bullet.init_new(player, bml);
        }
    }
}

#[godot_api]
impl INode for BulletMLCanvas {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            runs_on_editor: true,
            capacity: 0,
            pool: Pool::new(0, Bullet::default),
            turn: 0,
            canvas_item: Rid::Invalid,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if !self.runs_on_editor && Engine::singleton().is_editor_hint() {
            return;
        }

        let (mut current_pool, mut new_pool) = self.pool.split();
        let mut iter = current_pool.into_iter();
        let mut factory = BulletFactory {
            pool: &mut new_pool,
        };
        let rs = RenderingServer::singleton();

        while let Some((bullet, _bullet_ref)) = iter.next() {
            let mut b = bullet.bullet.bind_mut();
            b.process(delta, &mut factory, self.turn, rs.clone());
        }

        self.turn += 1;
    }
}

#[derive(Clone)]
pub(crate) struct Bullet {
    pub(crate) bullet: Gd<BulletMLBullet>,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            bullet: BulletMLBullet::alloc_gd(),
        }
    }
}

pub(crate) struct BulletFactory<'a, 'p> {
    pool: &'a mut PoolGetInstanceArea<'p, Bullet>,
}

impl<'a, 'p> BulletFactory<'a, 'p> {
    pub fn create_bullet_simple(&mut self, label: &Option<String>, degree: f64, speed: f64) {
        if let Some(actor) = self.pool.get_instance() {
            let mut bullet = actor.0.bullet.bind_mut();
            bullet.init_simple(label, degree, speed);
        }
    }

    pub fn create_bullet_from_state(&mut self, label: &Option<String>, degree: f64, speed: f64, state: bulletml::State) {
        if let Some(actor) = self.pool.get_instance() {
            let mut bullet = actor.0.bullet.bind_mut();
            bullet.init_from_state(label, degree, speed, state);
        }
    }
}
