use super::*;

pub struct Bullet {
    pos: Point2<f32>,
    dim: Vector2<f32>,
    vel: Vector2<f32>,
}

impl Bullet {
    pub fn new(
        pos: Point2<f32>, 
        dim: Vector2<f32>, 
        vel: Vector2<f32>
    ) -> Self {
        Self {pos,dim,vel}
    }
}

impl Actor for Bullet {
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
                graphics::WHITE,
            );
        }
        else {
            let mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                rect,
                graphics::WHITE,
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

