use super::*;
use crate::actors::*;

pub struct GameplayScene {
    player: Player,
    enemies: Vec<Box<dyn Actor>>,
    timer: f32,
    total_time: f32,
    is_game_over: bool,
}

impl GameplayScene {
    pub fn new_box() -> SceneBox {
        let s = Self {
            player: Player::new([400.0,300.0].into(), [18.0, 18.0].into()),
            enemies: vec![],
            timer: 5.0,
            total_time: 0.0,
            is_game_over: false,
        };
        Box::new(s)
    }

    fn spawn_bullet(&mut self) {
        use rand::Rng;
        let rng = &mut rand::thread_rng();
        let spawn_pos = match rng.gen_range(0,4) {
            0 => Point2::new(rng.gen_range(0.0, 800.0), 1.0),
            1 => Point2::new(rng.gen_range(0.0, 800.0), 599.0),
            2 => Point2::new(1.0, rng.gen_range(0.0, 600.0)),
            _ => Point2::new(799.0, rng.gen_range(0.0, 600.0)),
        };
        let dir_vec = self.player.get_pos() - spawn_pos;
        let vel_vec = dir_vec.normalize() * rng.gen_range(50.0, 150.0);
        let size = rng.gen_range(5.0, 15.0);
        let bullet: Box<dyn Actor> = if rng.gen_range(0, 10) <= 8 {
            Box::new(Bullet::new(
                spawn_pos,
                [size, size].into(),
                vel_vec,
            ))
        }
        else {
            Box::new(DrunkBullet::new(
                spawn_pos,
                [size, size].into(),
                vel_vec,
            ))
        };
        self.enemies.push(bullet);
    }
}

impl Scene for GameplayScene {
    fn update(
        &mut self,
        ctx: &mut Context,
        dt: f32,
        scene_event_queue: &mut VecDeque<SceneEvent>
    ) -> ggez::GameResult {
        self.total_time += dt;

        self.timer -= dt;
        if self.timer <= 0.0 {
            self.timer += 5.0;
            self.spawn_bullet();
        }

        const VELOCITY_SCALAR: f32 = 150.0;
        let keyset = keyboard::pressed_keys(ctx);
        let mut dir = Point2::new(0.0, 0.0);
        if keyset.contains(&KeyCode::W) {
            dir += Vector2::new(0.0, -1.0);
        }
        if keyset.contains(&KeyCode::S) {
            dir += Vector2::new(0.0, 1.0);
        }
        if keyset.contains(&KeyCode::A) {
            dir += Vector2::new(-1.0, 0.0);
        }
        if keyset.contains(&KeyCode::D) {
            dir += Vector2::new(1.0, 0.0);
        }

        if dir.x == 0.0 && dir.y == 0.0 {
            self.player.set_vel(Vector2::new(0.0, 0.0));
        }
        else {
            self.player.set_vel(
                (dir - Point2::new(0.0, 0.0)).normalize() * VELOCITY_SCALAR
            );
        }
        self.player.update(dt);
        let mut pos = self.player.get_pos();
        pos.x = na::wrap(pos.x, 0.0, 800.0);
        pos.y = na::wrap(pos.y, 0.0, 600.0);
        self.player.set_pos(pos);

        let player_rect = self.player.get_rect();
        self.is_game_over = self.enemies
            .par_iter_mut()
            .update(|enemy| {
                enemy.update(dt);
                let mut pos = enemy.get_pos();
                pos.x = na::wrap(pos.x, 0.0, 800.0);
                pos.y = na::wrap(pos.y, 0.0, 600.0);
                enemy.set_pos(pos);
            })
            .any(|enemy| {
                let enemy_rect = enemy.get_rect();
                player_rect.overlaps(&enemy_rect)
            });
        
        if self.is_game_over {
            scene_event_queue.push_back(
                SceneEvent::Push(
                    GameOverScene::new_box()
                )
            )
        }
        else {
            let mut new_enemies: Vec<_> = self.enemies
                .par_iter()
                .filter_map(|enemy| {
                    if enemy.has_action() {
                        enemy.action(
                            dt,
                            &self.player,
                            &self.enemies,
                        )
                    }
                    else {
                        None
                    }
                })
                .flatten()
                .collect();
            self.enemies.append(&mut new_enemies);
        }

        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut Context
    ) -> ggez::GameResult {
        let mesh_builder = &mut graphics::MeshBuilder::new();
        self.player.draw(ctx, Some(mesh_builder))?;
        for enemy in self.enemies.iter() {
            enemy.draw(ctx, Some(mesh_builder))?;
        }
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        )?;
        Ok(())
    }

    fn draw_in_background(&self) -> bool { true }
}
