use std::collections::VecDeque;

use ggez;
use ggez::event;
use ggez::graphics;

mod scene; use scene::{SceneEvent, SceneBox};

struct MainState {
    current_scene: Option<SceneBox>,
    previous_scene_stack: Vec<SceneBox>,
    scene_event_queue: VecDeque<SceneEvent>,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = Self {
            current_scene: Some(
                scene::StartingScene::new_box()
            ),
            previous_scene_stack: vec![],
            scene_event_queue: VecDeque::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(
        &mut self,
        ctx: &mut ggez::Context
    ) -> ggez::GameResult {
        let current_scene = &mut self.current_scene;
        let previous_scene_stack = &mut self.previous_scene_stack;
        let scene_event_queue = &mut self.scene_event_queue;

        for event in scene_event_queue.drain(..) {
            match event {
                SceneEvent::Push(scene_box) => {
                    if let Some(prev_scene_box)
                    = current_scene.replace(scene_box) {
                        previous_scene_stack.push(prev_scene_box);
                    }
                }
                SceneEvent::Pop => {
                    if let Some(prev_scene_box)
                    = previous_scene_stack.pop() {
                        current_scene.replace(prev_scene_box);
                    }
                }
                SceneEvent::Replace(scene_box) => {
                    current_scene.replace(scene_box);
                }
            }
        }

        if let Some(scene) = current_scene {
            scene.update(ctx, scene_event_queue)?;
        }

        for prev_scene in previous_scene_stack.iter_mut().rev() {
            if prev_scene.update_in_background() {
                prev_scene.update(ctx, scene_event_queue)?;
            }
        }
        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut ggez::Context
    ) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let current_scene = &mut self.current_scene;
        let previous_scene_stack = &mut self.previous_scene_stack;

        for prev_scene in previous_scene_stack.iter_mut() {
            if prev_scene.draw_in_background() {
                prev_scene.draw(ctx)?;
            }
        }

        if let Some(scene) = current_scene {
            scene.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("LD45", "prixt");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}