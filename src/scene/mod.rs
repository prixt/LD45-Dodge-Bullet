use std::collections::VecDeque;

use ggez::Context;
use ggez::graphics::{self, Font, Text, Scale};
use ggez::input::keyboard::{
    self, KeyCode, KeyMods,
};
use ggez::nalgebra as na;
use na::{Point2, Vector2};
use rayon::prelude::*;

pub type SceneBox = Box<dyn Scene>;

pub trait Scene {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _dt: f32,
        _scene_event_queue: &mut VecDeque<SceneEvent>
    ) -> ggez::GameResult { Ok(()) }

    fn draw(
        &mut self,
        _ctx: &mut Context
    ) -> ggez::GameResult { Ok(()) }
    
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _key: KeyCode,
        _mods: KeyMods,
        _repeat: bool,
        _scene_event_queue: &mut VecDeque<SceneEvent>
    ) {}
    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _key: KeyCode,
        _mods: KeyMods,
        _scene_event_queue: &mut VecDeque<SceneEvent>
    ) {}

    fn update_in_background(&self) -> bool { false }
    fn draw_in_background(&self) -> bool { false }
}

#[allow(dead_code)]
pub enum SceneEvent {
    Pop,
    Push(SceneBox),
    Replace(SceneBox),
}

mod starting;
mod gameplay;
mod gameover;
pub use starting::StartingScene;
pub use gameplay::GameplayScene;
pub use gameover::GameOverScene;