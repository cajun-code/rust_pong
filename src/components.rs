use bevy::prelude::*;
use bevy::math::Vec2;

#[derive(Component)]
pub struct Player1;
#[derive(Component)]
pub struct Player2;
#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Boundary;

#[derive(Component)]
pub struct Position(pub Vec2);
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Shape(pub Vec2);

#[derive(Component)]
pub struct Player1Score;
#[derive(Component)]
pub struct Player2Score;

pub enum Scorer{
    Player1,
    Player2,
}

#[derive(Event)]
pub struct ScoreEvent(pub Scorer);

#[derive(Resource, Default)]
pub struct Scoreboard {
    pub player1: u32,
    pub player2: u32,
}
