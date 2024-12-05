use bevy::prelude::*;
use bevy::text::TextPlugin;
use flappy_bird::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use flappy_bird::{background, camera, game, menu, pipe, states::{GameState, PlayingState}, Game};
use flappy_bird::{bird, ground, hud};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Flappy Bird".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(camera::plugin)
        .init_resource::<Game>()
        .init_state::<GameState>()
        .init_state::<PlayingState>()
        .add_plugins(menu::plugin)
        .add_plugins(game::plugin)
        .add_plugins(background::plugin)
        .add_plugins(ground::plugin)
        .add_plugins(hud::plugin)
        .add_plugins(bird::plugin)
        .add_plugins(pipe::plugin)
        .run();
}
