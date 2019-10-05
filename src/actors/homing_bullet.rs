use super::*;
use std::sync::Mutex;

pub struct HomingBullet {
    pos: Point2<f32>,
    dim: Vector2<f32>,
    vel: Vector2<f32>,
    target: Mutex<Point2<f32>>,
}

impl HomingBullet {
    pub fn new(
        pos: Point2<f32>, 
        dim: Vector2<f32>, 
        vel: Vector2<f32>,
        target: Point2<f32>,
    ) -> Self {
        Self {
            pos,dim,vel,
            target: Mutex::new(target)
        }
    }
}

impl Actor for HomingBullet {
    #[inline]
    fn get_pos(&self) -> Point2<f32> {
        self.pos
    }

    #[inline]
    fn get_rect(&self) -> Rect {
        Rect{
            x: self.pos.x - self.dim.x / 2.0,
            y: self.pos.y - self.dim.y / 2.0,
            w: self.dim.x,
            h: self.dim.y,
        }
    }

    #[inline]
    fn get_vel(&self) -> Vector2<f32> {
        self.vel
    }

    #[inline]
    fn set_pos(&mut self, pos: Point2<f32>) {
        self.pos = pos
    }

    #[inline]
    fn set_dim(&mut self, dim: Vector2<f32>) {
        self.dim = dim
    }

    #[inline]
    fn set_vel(&mut self, vel: Vector2<f32>) {
        self.vel = vel
    }

    fn has_action(&self) -> bool { true }
    fn action(
        &self,
        _dt: f32,
        player: &Player,
        _: &[Box<dyn Actor>],
    ) -> Option<Vec<Box<dyn Actor>>> {
        *self.target.lock().unwrap() = player.get_pos();
        None
    }

    fn update(&mut self, dt: f32) {
        const HOMING_FACTOR: f32 = 50.0;
        const SPEED_LIMIT: f32 = 100.0;
        let diff = *self.target.lock().unwrap() - self.pos;
        let acc = diff.normalize() * HOMING_FACTOR;
        self.vel += acc * dt;
        let speed_sqr =
            self.vel.x * self.vel.x + self.vel.y * self.vel.y;
        if speed_sqr > SPEED_LIMIT * SPEED_LIMIT {
            self.vel = self.vel.normalize() * SPEED_LIMIT;
        }
        self.add_pos(self.vel * dt);
    }

    fn draw(
        &self,
        ctx: &mut ggez::Context,
        mesh_builder: Option<&mut MeshBuilder>
    ) -> ggez::GameResult
    {
        let rect = self.get_rect();
        if let Some(mesh_builder) = mesh_builder {
            mesh_builder.rectangle(
                DrawMode::fill(),
                rect,
                [0.0, 0.5, 1.0, 1.0].into(),
            );
        }
        else {
            let mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                rect,
                [0.0, 0.5, 1.0, 1.0].into(),
            )?;
            ggez::graphics::draw(
                ctx,
                &mesh,
                DrawParam::default(),
            )?;
        }
            
        Ok(())
    }
}

