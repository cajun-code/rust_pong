mod bundles;
mod components;
mod constants;
mod systems;
mod window;

use bevy::{prelude::*, ui::update};
use components::*;
use systems::*;
use window::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(create_window()))
        .init_resource::<Scoreboard>()
        .add_event::<ScoreEvent>()
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_dotted_line,
                spawn_ball,
                spawn_paddle,
                spawn_boundary,
                spawn_scoreboard,
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                move_player1_paddle,
                detect_scoring,
                move_player2_paddle,
                respawn_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                update_scoreboard.after(update_score),
                update_entity_position.after(move_ball),
                move_paddles.after(move_player1_paddle),
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}
