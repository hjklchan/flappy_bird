use bevy::prelude::*;
use flappy_bird::{camera, states::GameState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::plugin)
        .init_state::<GameState>()
        .run();
}
