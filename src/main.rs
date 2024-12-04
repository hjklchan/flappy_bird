use bevy::prelude::*;
use bevy::window::WindowResolution;
use flappy_bird::{camera, menu, game, states::GameState, background, Game};
use flappy_bird::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(camera::plugin)
        .init_resource::<Game>()
        .init_state::<GameState>()
        .add_plugins(menu::plugin)
        .add_plugins(game::plugin)
        .add_plugins(background::plugin)
        .run();
}
