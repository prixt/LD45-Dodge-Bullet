use super::*;

pub struct StartingScene {
    title: graphics::Text,
    body: graphics::Text,
}

impl StartingScene {
    pub fn new_box() -> SceneBox {
        let s = Self {
            title: graphics::Text::new("StartingScene"),
            body: graphics::Text::new("Lorem Ipsum"),
        };
        Box::new(s)
    }
}

impl Scene for StartingScene {
    fn update(
        &mut self,
        _ctx: &mut ggez::Context,
        _scene_event_queue: &mut VecDeque<SceneEvent>
    ) -> ggez::GameResult {
        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut ggez::Context
    ) -> ggez::GameResult {
        graphics::draw(
            ctx,
            &self.title,
            graphics::DrawParam::default()
                .dest([0.0, 0.0])
        )?;
        graphics::draw(
            ctx,
            &self.body,
            graphics::DrawParam::default()
                .dest([0.0, 20.0])
        )?;
        Ok(())
    }
}

