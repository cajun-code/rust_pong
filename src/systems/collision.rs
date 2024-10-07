use crate::bundles::BoundaryBundle;
use crate::components::*;
use crate::constants::*;
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Collision {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn handle_collisions(
    mut ball_query: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball_query.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = detect_collision(ball_position, ball_shape, position, shape) {
                respond_to_collision(&mut ball_velocity, collision);
            }
        }
    }
}

fn respond_to_collision(ball_velocity: &mut Velocity, collision: Collision) {
    match collision {
        Collision::Left | Collision::Right => ball_velocity.0.x *= -1.0,
        Collision::Top | Collision::Bottom => ball_velocity.0.y *= -1.0,
    }
}

fn detect_collision(
    ball_position: &Position,
    ball_shape: &Shape,
    other_position: &Position,
    other_shape: &Shape,
) -> Option<Collision> {
    let ball = BoundingCircle::new(ball_position.0, ball_shape.0.x);
    let wall = Aabb2d::new(other_position.0, other_shape.0 / 2.0);

    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    if offset.x.abs() > offset.y.abs() {
        if offset.x > 0.0 {
            Some(Collision::Right)
        } else {
            Some(Collision::Left)
        }
    } else {
        if offset.y > 0.0 {
            Some(Collision::Bottom)
        } else {
            Some(Collision::Top)
        }
    }
}

pub fn spawn_boundary(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
) {
    if let Ok(window) = window_query.get_single() {
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        let top_boundary_y = window_height / 2.0 - BOUNDRY_HEIGHT / 2.0;
        let bottom_boundary_y = -window_height / 2.0 + BOUNDRY_HEIGHT / 2.0;

        let top_boundary = BoundaryBundle::new(0.0, top_boundary_y, window_width);
        let bottom_boundary = BoundaryBundle::new(0.0, bottom_boundary_y, window_width);

        let mesh = Mesh::from(Rectangle::from_size(top_boundary.shape.0));
        let material = ColorMaterial::from(Color::srgb(0.0, 0.0, 0.0));

        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(material);

        commands.spawn((
            top_boundary,
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                ..default()
            },
        ));
        commands.spawn((
            bottom_boundary,
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                ..default()
            },
        ));
    }
}
