use std::ops::Deref;
use std::sync::Arc;

use bulletml;
use godot::engine::RenderingServer;
use godot::prelude::*;

use crate::bulletml_canvas::BulletFactory;
use crate::bulletml_player::BulletMLPlayer;
use crate::bulletml_resource::get_empty_bulletml;

struct BulletData {
    player: Gd<BulletMLPlayer>,
    position: Vector2,
    degree: f64,
    speed: f64,
    velocity: Vector2,
}

impl BulletData {
    fn update_velocity(&mut self) {
        self.velocity = Vector2::UP.rotated(utilities::deg_to_rad(self.degree) as real) * (self.speed as real);
    }
}

impl Default for BulletData {
    fn default() -> Self {
        Self {
            player: BulletMLPlayer::alloc_gd(),
            position: Vector2::ZERO,
            degree: 0.0,
            speed: 0.0,
            velocity: Vector2::ZERO,
        }
    }
}

#[derive(GodotClass)]
#[class(base=Object)]
pub struct BulletMLBullet {
    #[base]
    base: Base<Object>,

    pub(crate) canvas_item_rid: Rid,

    bml: Arc<bulletml::BulletML>,
    is_simple: bool,
    runner: bulletml::Runner<GodotRunner>,
    data: BulletData,
}

#[godot_api]
impl BulletMLBullet {
    pub fn new(base: Base<Object>, godot_runner: &GodotRunner) -> Self {
        Self {
            base,
            canvas_item_rid: RenderingServer::singleton().canvas_item_create(),
            bml: get_empty_bulletml().clone(),
            is_simple: false,
            runner: bulletml::Runner::new(godot_runner.clone(), get_empty_bulletml()),
            data: BulletData::default(),
        }
    }

    pub fn init_new(&mut self, player: Gd<BulletMLPlayer>, bml: Arc<bulletml::BulletML>) {
        RenderingServer::singleton().canvas_item_clear(self.canvas_item_rid);
        RenderingServer::singleton().canvas_item_add_rect(self.canvas_item_rid, Rect2::new(Vector2::ZERO, Vector2::ONE), Color::WHITE);
        self.data.player = player;
        self.bml = bml.clone();
        self.runner.init(bml.deref());
        self.is_simple = false;
    }

    pub fn init_simple(&mut self, _label: &Option<String>, degree: f64, speed: f64) {
        RenderingServer::singleton().canvas_item_clear(self.canvas_item_rid);
        RenderingServer::singleton().canvas_item_add_rect(self.canvas_item_rid, Rect2::new(Vector2::ZERO, Vector2::ONE), Color::WHITE);
        self.is_simple = true;

        self.data.degree = degree;
        self.data.speed = speed;
        self.data.update_velocity();
    }

    pub fn init_from_state(&mut self, _label: &Option<String>, degree: f64, speed: f64, state: bulletml::State) {
        RenderingServer::singleton().canvas_item_clear(self.canvas_item_rid);
        RenderingServer::singleton().canvas_item_add_rect(self.canvas_item_rid, Rect2::new(Vector2::ZERO, Vector2::ONE), Color::WHITE);
        self.is_simple = false;
        self.runner.init_from_state(state);

        self.data.degree = degree;
        self.data.speed = speed;
        self.data.update_velocity();
    }

    pub(crate) fn process(&mut self, delta: f64, factory: &mut BulletFactory, turn: u32) {
        if !self.is_simple && !self.runner.is_end() {
            let bml = self.bml.clone();
            let runner = &mut self.runner;
            let mut data = GodotData {
                bullet: &mut self.data,
                factory,
                turn,
             };
            let runner_data = &mut bulletml::RunnerData {
                bml: bml.deref(),
                data: &mut data,
            };
            runner.run(runner_data);
        }

        self.data.position = self.data.position + self.data.velocity;// * (delta as real);

        RenderingServer::singleton().canvas_item_set_transform(self.canvas_item_rid, Transform2D::IDENTITY.translated(self.data.position));
    }
}

#[godot_api]
impl IObject for BulletMLBullet {
    // Dummy function for hot reloading
    fn init(base: Base<Object>) -> Self {
        Self::new(base, &GodotRunner::default())
    }
}

impl Drop for BulletMLBullet {
    fn drop(&mut self) {
        //RenderingServer::singleton().free_rid(self.canvas_item_rid);
    }
}


#[derive(Clone, Default)]
pub struct GodotRunner;

struct GodotData<'a, 'm, 'p> where 'm: 'a {
    bullet: &'a mut BulletData,
    factory: &'a mut BulletFactory<'m, 'p>,
    turn: u32,
}

impl<'a, 'm, 'p> bulletml::AppRunner<GodotData<'a, 'm, 'p>> for GodotRunner {
    fn get_bullet_direction(&self, data: &GodotData) -> f64 {
        data.bullet.degree
    }

    fn get_aim_direction(&self, _data: &GodotData) -> f64 {
        0.0 // TODO
    }

    fn get_bullet_speed(&self, data: &GodotData) -> f64 {
        data.bullet.speed
    }

    fn get_default_speed(&self) -> f64 {
        0.0
    }

    fn get_rank(&self, data: &GodotData) -> f64 {
        data.bullet.player.bind().get_rank()
    }

    fn create_simple_bullet(&mut self, data: &mut GodotData, direction: f64, speed: f64, label: &Option<String>) {
        data.factory.create_bullet_simple(label, direction, speed);
    }

    fn create_bullet(&mut self, data: &mut GodotData, state: bulletml::State, direction: f64, speed: f64, label: &Option<String>) {
        data.factory.create_bullet_from_state(label, direction, speed, state);
    }

    fn get_turn(&self, data: &GodotData) -> u32 {
        data.turn
    }

    fn do_vanish(&mut self, _data: &mut GodotData) {
        //data.bullet.queue_free();
    }

    fn do_change_direction(&mut self, data: &mut GodotData, direction: f64) {
        data.bullet.degree = direction;
        data.bullet.update_velocity();
    }

    fn do_change_speed(&mut self, data: &mut GodotData, speed: f64) {
        data.bullet.speed = speed;
        data.bullet.update_velocity();
    }

    fn do_accel_x(&mut self, _: f64) {
        todo!()
    }

    fn do_accel_y(&mut self, _: f64) {
        todo!()
    }

    fn get_rand(&self, _data: &mut GodotData) -> f64 {
        utilities::randf_range(0.0, 1.0)
    }
}
