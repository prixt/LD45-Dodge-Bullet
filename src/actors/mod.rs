use ggez::nalgebra as na;
use na::{Point2, Vector2};
use ggez::graphics::{
    self, Rect, DrawMode, DrawParam, Mesh, MeshBuilder,
};

mod player;
mod bullet;
mod drunk_bullet;
pub use player::Player;
pub use bullet::Bullet;
pub use drunk_bullet::DrunkBullet;

pub trait Actor: Send + Sync {
    fn get_pos(&self) -> Point2<f32>;
    fn get_rect(&self) -> Rect;
    fn get_vel(&self) -> Vector2<f32>;

    fn set_pos(&mut self, pos: Point2<f32>);
    fn set_dim(&mut self, dim: Vector2<f32>);
    fn set_vel(&mut self, vel: Vector2<f32>);

    fn add_pos(&mut self, diff: Vector2<f32>) {
        self.set_pos(self.get_pos() + diff)
    }

    fn update(
        &mut self,
        dt: f32,
    ) {
        self.add_pos(self.get_vel() * dt);
    }

    fn has_action(&self) -> bool { false }

    fn action(
        &self,
        _dt: f32,
        _player: &Player,
        _current_enemies: &[Box<dyn Actor>],
    ) -> Option<Vec<Box<dyn Actor>>> { None }
    
    fn draw(
        &self,
        ctx: &mut ggez::Context,
        mesh_builder: Option<&mut MeshBuilder>
    ) -> ggez::GameResult;
}