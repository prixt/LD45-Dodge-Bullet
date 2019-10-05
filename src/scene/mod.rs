use std::collections::VecDeque;

use ggez::graphics;
use ggez::nalgebra as na;

pub type SceneBox = Box<dyn Scene>;

pub trait Scene {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
        scene_event_queue: &mut VecDeque<SceneEvent>
    ) -> ggez::GameResult;
    fn draw(
        &mut self,
        ctx: &mut ggez::Context
    ) -> ggez::GameResult;
    fn update_in_background(&self) -> bool {false}
    fn draw_in_background(&self) -> bool {false}
}

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