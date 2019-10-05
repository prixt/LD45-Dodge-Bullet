use super::*;

pub struct StartingScene {
    title: graphics::Text,
    body: graphics::Text,
    pos: Point2<f32>,
}

impl StartingScene {
    pub fn new_box() -> SceneBox {
        let s = Self {
            title: graphics::Text::new("StartingScene"),
            body: graphics::Text::new("Lorem Ipsum"),
            pos: [0.0, 0.0].into(),
        };
        Box::new(s)
    }
}

impl Scene for StartingScene {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        _: KeyMods,
        _: bool,
        scene_event_queue: &mut VecDeque<SceneEvent>
    ) {
        match key {
            KeyCode::Space => scene_event_queue.push_back(
                SceneEvent::Replace(GameplayScene::new_box())
            ),
            KeyCode::Q | KeyCode::Escape => {
                ggez::event::quit(ctx)
            }
            _ => ()
        }
    }

    fn draw(
        &mut self,
        ctx: &mut Context
    ) -> ggez::GameResult {
        graphics::draw(
            ctx,
            &self.title,
            graphics::DrawParam::default()
                .dest(self.pos)
        )?;
        graphics::draw(
            ctx,
            &self.body,
            graphics::DrawParam::default()
                .dest(self.pos + Vector2::new(0.0, 10.0))
        )?;
        Ok(())
    }
}

