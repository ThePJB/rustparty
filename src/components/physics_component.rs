use crate::rect::Rect;
use crate::vec2::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct PhysicsComponent {
    pub aabb: Rect,
    pub vel: Vec2,
}