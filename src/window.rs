use bevy::prelude::*;

use crate::constants;

pub fn create_window()-> WindowPlugin{
    WindowPlugin {
        primary_window: Some(Window {
            title: "Pong".to_string(),
            ..default()
        }),
        ..default()
    }
}

pub fn spawn_dotted_line(mut commands: Commands) {
    let dot_color = Color::srgb(1.0, 1.0, 1.0);
    let dot_size = Vec3::new(5.0, 20.0, 1.0);
    let gap_size = 10.0;
    let num_dots = (constants::ARENA_HEIGHT / (dot_size.y + gap_size)) as i32;
    for i in 0..num_dots {

        let calculated_y = i as f32 * (dot_size.y + gap_size) - constants::ARENA_HEIGHT/2.0;
        let z = 0.0;
        let x = 0.0;
        //print!("placeing dot {i} of {num_dots}:({x}, {calculated_y}, {z}) \n");
        commands.spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: dot_color,
                    ..default()
                },
                transform: Transform { 
                    translation: Vec3::new(
                        x, 
                        calculated_y,
                        z), 
                    scale: dot_size, 
                    ..default() 
                },
                ..default()
            }
        );
    }
}