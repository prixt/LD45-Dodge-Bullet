use super::*;

pub struct GameOverScene;

impl GameOverScene {
    pub fn new_box() -> SceneBox {
        let s = Self;
        Box::new(s)
    }
}

impl Scene for GameOverScene {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        _: KeyMods,
        _: bool,
        scene_event_queue: &mut VecDeque<SceneEvent>
    ) {
        match key {
            KeyCode::R => {
                scene_event_queue.push_back(
                    SceneEvent::Replace(GameplayScene::new_box())
                )
            }
            KeyCode::Q | KeyCode::Escape => {
                ggez::event::quit(ctx)
            }
            _ => ()
        }
    }
}