use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use bulletml::{AppRunner, Runner, RunnerData, State};
use godot::engine::utilities::randf_range;
use godot::prelude::*;

use crate::resource::BulletML;

#[derive(GodotClass)]
#[class(base=Node)]
struct BulletMLPlayer {
    #[base]
    base: Base<Node>,

    #[export]
    bullet_root: Option<Gd<Node2D>>,

    #[export]
    bulletml: Option<Gd<BulletML>>,

    #[export]
    bullet_scene: Gd<PackedScene>,

    turn: u32,
}

#[godot_api]
impl BulletMLPlayer {
    fn add_bullet(&mut self, is_simple: bool, direction: f32, speed: f32, state: Option<State>) {
        if self.bullet_root.is_none() {
            return;
        }

        let player = self.get_node_as::<BulletMLPlayer>(".");
        let child = self.bullet_scene.instantiate_as::<Node2D>();

        let bml = self.bulletml.as_ref().unwrap().bind().bml.clone();
        let mut bullet = Gd::<Bullet>::with_base(|base| {
            Bullet::new(base, player, child, bml.clone(), is_simple)
        });
        {
            let mut b = bullet.bind_mut();
            if let Some(s) = state {
                b.runner.init_from_state(s);
            } else {
                b.runner.init(bml.clone().deref());
            }
            b.set(direction, speed);
        }

        let a = self.bullet_root.as_mut().unwrap();
        a.deref_mut().add_child(bullet.upcast());

    }
}

#[godot_api]
impl NodeVirtual for BulletMLPlayer {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            bullet_root: None,
            bulletml: None,
            bullet_scene: PackedScene::new(),
            turn: 0,
        }
    }

    fn enter_tree(&mut self) {
        if self.bulletml.is_none() {
            return;
        }
        self.add_bullet(false, 0.0, 0.0, None);
    }

    fn physics_process(&mut self, _delta: f64) {
        // if Engine::singleton().is_editor_hint() {
        //     return;
        // }

        self.turn += 1;
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Bullet {
    #[base]
    base: Base<Node2D>,

    player: Gd<BulletMLPlayer>,
    presentation: Gd<Node2D>,
    bml: Rc<bulletml::BulletML>,
    runner: Runner<GodotRunner>,
    is_simple: bool,

    bullet_impl: BulletImpl,
}

#[godot_api]
impl Bullet {
    fn new(base: Base<Node2D>, player: Gd<BulletMLPlayer>, presentation: Gd<Node2D>, bml: Rc<bulletml::BulletML>, is_simple: bool) -> Self {
        Self {
            base,
            player,
            presentation,
            bml: bml.clone(),
            runner: Runner::new(GodotRunner::new(), bml.clone().deref()),
            is_simple,
            bullet_impl: BulletImpl { degree: 0.0, speed: 0.0 },
        }
    }

    fn set(&mut self, direction: f32, speed: f32) {
        self.bullet_impl.degree = direction;
        self.bullet_impl.speed = speed;
    }
}

#[godot_api]
impl Node2DVirtual for Bullet {
    fn ready(&mut self) {
        let p = self.presentation.share();
        self.add_child(p.upcast());
    }

    fn physics_process(&mut self, _delta: f64) {
        if !self.is_simple {
            if !self.runner.is_end() {
                let runner = &mut self.runner;
                let data = &mut GodotData {
                    player: self.player.share(),
                    bullet: &mut self.bullet_impl,
                };
                let r = &mut RunnerData {
                    bml: self.bml.deref(),
                    data,
                };
                runner.run(r);
            }
        }

        let mx = f32::sin(self.bullet_impl.degree) * self.bullet_impl.speed;
        let my = f32::cos(self.bullet_impl.degree) * self.bullet_impl.speed;
        let pos = self.get_position();
        self.set_position(pos + Vector2::new(mx, my));
    }
}

struct BulletImpl {
    degree: f32,
    speed: f32,
}

struct GodotData<'a> {
    player: Gd<BulletMLPlayer>,
    bullet: &'a mut BulletImpl,
}

struct GodotRunner {}

impl GodotRunner {
    fn new() -> Self {
        Self {}
    }
}

impl<'a> AppRunner<GodotData<'a>> for GodotRunner {
    fn get_bullet_direction(&self, data: &GodotData) -> f64 {
        rtod(data.bullet.degree) as f64
    }

    fn get_aim_direction(&self, _data: &GodotData) -> f64 {
        0.0
    }

    fn get_bullet_speed(&self, data: &GodotData) -> f64 {
        data.bullet.speed as f64
    }

    fn get_default_speed(&self) -> f64 {
        1.0
    }

    fn get_rank(&self, _data: &GodotData) -> f64 {
        1.0
    }

    fn create_simple_bullet(&mut self, data: &mut GodotData, direction: f64, speed: f64) {
        data.player.bind_mut().add_bullet(true, dtor(direction as f32), speed as f32, None);
    }

    fn create_bullet(&mut self, data: &mut GodotData, state: State, direction: f64, speed: f64) {
        data.player.bind_mut().add_bullet(false, dtor(direction as f32), speed as f32, Some(state));
    }

    fn get_turn(&self, data: &GodotData) -> u32 {
        data.player.bind().turn
    }

    fn do_vanish(&mut self, _data: &mut GodotData) {
        //data.bullet.queue_free();
    }

    fn do_change_direction(&mut self, data: &mut GodotData, direction: f64) {
        data.bullet.degree = dtor(direction as f32);
    }

    fn do_change_speed(&mut self, data: &mut GodotData, speed: f64) {
        data.bullet.speed = speed as f32;
    }

    fn do_accel_x(&mut self, _: f64) {
        todo!()
    }

    fn do_accel_y(&mut self, _: f64) {
        todo!()
    }

    fn get_rand(&self, _data: &mut GodotData) -> f64 {
        randf_range(0.0, 1.0)
    }
}

fn rtod(a: f32) -> f32 {
    a * 180. / std::f32::consts::PI
}

fn dtor(a: f32) -> f32 {
    a * std::f32::consts::PI / 180.
}


