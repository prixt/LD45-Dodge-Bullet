use super::*;

pub struct StartingScene {
    title: Text,
    body: Text,
}

impl StartingScene {
    pub fn new_box(font: Font) -> SceneBox {
        let mut title = Text::new("Untitled\nBullet Dodger");
        title.set_font(font, Scale::uniform(50.0))
            .set_bounds(
                [300.0, std::f32::INFINITY],
                graphics::Align::Center,
            );
        let mut body = Text::new("[W,A,S,D] to move.\nTry to dodge the bullets for as long as possible.");
        body.set_font(font, Scale::uniform(30.0))
            .set_bounds(
                [300.0, std::f32::INFINITY],
                graphics::Align::Center,
            );
        let s = Self {
            title, body,
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
            KeyCode::W | KeyCode::A
            | KeyCode::S | KeyCode::D
            | KeyCode::Space => {
                scene_event_queue.push_back(
                    SceneEvent::Pop
                )
            },
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
            Point2::new(250.0, 275.0 - h)
        };
        graphics::draw(
            ctx,
            &self.title,
            graphics::DrawParam::default()
                .dest(title_pos)
        )?;
        let body_pos = Point2::new(250.0, 350.0);
        graphics::draw(
            ctx,
            &self.body,
            graphics::DrawParam::default()
                .dest(body_pos)
        )?;
        Ok(())
    }
}

