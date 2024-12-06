use bevy::prelude::*;
use flappy_bird::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use flappy_bird::events::score;
use flappy_bird::{background, camera, game, menu, pipe, ui, states::{GameState, PlayingState}, Game};
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
        .add_event::<score::Add>()
        .add_plugins(menu::plugin)
        .add_plugins(game::plugin)
        .add_plugins(background::plugin)
        .add_plugins(ground::plugin)
        .add_plugins(hud::plugin)
        .add_plugins(bird::plugin)
        .add_plugins(pipe::plugin)
        .add_plugins(ui::plugin)
        .run();
}
