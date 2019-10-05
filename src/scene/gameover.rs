use super::*;

pub struct GameOverScene;

impl Scene for GameOverScene {
    fn update(
        &mut self,
        _ctx: &mut ggez::Context,
        _scene_event_queue: &mut VecDeque<SceneEvent>
    ) -> ggez::GameResult {
        Ok(())
    }

    fn draw(
        &mut self,
        _ctx: &mut ggez::Context
    ) -> ggez::GameResult {
        Ok(())
    }
}