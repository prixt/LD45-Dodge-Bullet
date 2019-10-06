// #[macro_use] extern crate lazy_static;

use std::collections::VecDeque;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{
    KeyCode, KeyMods,
};

mod scene; use scene::{SceneEvent, SceneBox};
mod actors;

struct MainState {
    current_scene: Option<SceneBox>,
    previous_scene_stack: Vec<SceneBox>,
    scene_event_queue: VecDeque<SceneEvent>,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let font = graphics::Font::new_glyph_font_bytes(
            ctx,
            include_bytes!("../resources/Silver.ttf")
        )?;
        let s = Self {
            current_scene: Some(
                scene::StartingScene::new_box(font)
            ),
            previous_scene_stack: vec![
                scene::GameplayScene::new_box(font),
            ],
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
        const DESIRED_FPS: u32 = 60;
        const DT: f32 = 1.0 / DESIRED_FPS as f32;

        let current_scene = &mut self.current_scene;
        let previous_scene_stack = &mut self.previous_scene_stack;
        let scene_event_queue = &mut self.scene_event_queue;

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            for event in scene_event_queue.drain(..) {
                match event {
                    SceneEvent::Push(mut scene_box) => {
                        scene_box.on_entry();
                        if let Some(mut prev_scene_box) = current_scene.replace(scene_box) {
                            prev_scene_box.on_exit();
                            previous_scene_stack.push(prev_scene_box);
                        }
                    }
                    SceneEvent::Pop => {
                        if let Some(mut prev_scene_box) = previous_scene_stack.pop() {
                            prev_scene_box.on_entry();
                            current_scene.replace(prev_scene_box);
                        }
                    }
                    SceneEvent::Replace(mut scene_box) => {
                        scene_box.on_entry();
                        current_scene.replace(scene_box);
                    }
                }
            }

            if let Some(scene) = current_scene {
                scene.update(ctx, DT, scene_event_queue)?;
            }

            for prev_scene in previous_scene_stack.iter_mut().rev() {
                if prev_scene.update_in_background() {
                    prev_scene.update(ctx, DT, scene_event_queue)?;
                }
            }
        }
        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut ggez::Context
    ) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.1, 0.2, 1.0].into());

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
    
    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        key: KeyCode,
        mods: KeyMods,
        repeat: bool
    ) {
        if let Some(current_scene) = &mut self.current_scene {
            current_scene.key_down_event(
                ctx,
                key,
                mods,
                repeat,
                &mut self.scene_event_queue
            );
        }
    }
    fn key_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        key: KeyCode,
        mods: KeyMods
    ) {
        if let Some(current_scene) = &mut self.current_scene {
            current_scene.key_up_event(
                ctx,
                key,
                mods,
                &mut self.scene_event_queue
            );
        }
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("LD45", "prixt");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}