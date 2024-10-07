use crate::components::*;
use crate::constants::*;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub shape: Shape,
    pub velocity: Velocity,
    pub position: Position,
}

impl BallBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0.0, 0.0)),
        }
    }
}

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub shape: Shape,
    pub position: Position,
    pub velocity: Velocity,
}

impl PaddleBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::new(0.0, 0.0)),
        }
    }
}

#[derive(Bundle)]
pub struct BoundaryBundle {
    pub boundary: Boundary,
    pub shape: Shape,
    pub position: Position,
}

impl BoundaryBundle {
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        Self {
            boundary: Boundary,
            shape: Shape(Vec2::new(width, BOUNDRY_HEIGHT)),
            position: Position(Vec2::new(x, y)),
        }
    }
}
