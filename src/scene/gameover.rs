use super::*;

pub struct GameOverScene {
    title: Text,
    body: Text,
    font: Font,
}

impl GameOverScene {
    pub fn new_box(font: Font) -> SceneBox {
        let mut title = Text::new("Game Over...\nYou surived for");
        title.set_font(font, Scale::uniform(50.0))
            .set_bounds(
                [800.0, 600.0],
                graphics::Align::Center,
            );
        let mut body = Text::new("seconds.\nPress [R] to Restart.\nPress [Q] to Quit.");
        body.set_font(font, Scale::uniform(30.0))
            .set_bounds(
                [800.0, 600.0],
                graphics::Align::Center,
            );
        let s = Self {
            title, body,
            font,
        };
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
                    SceneEvent::Pop
                );
                scene_event_queue.push_back(
                    SceneEvent::Replace(
                        GameplayScene::new_box(self.font)
                    )
                )
            }
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
        let title_pos = {
            let h = self.title.height(ctx) as f32;
            Point2::new(0.0, 250.0 - h)
        };
        graphics::draw(
            ctx,
            &self.title,
            graphics::DrawParam::default()
                .dest(title_pos)
        )?;
        let body_pos = Point2::new(0.0, 350.0);
        graphics::draw(
            ctx,
            &self.body,
            graphics::DrawParam::default()
                .dest(body_pos)
        )?;
        Ok(())
    }
}