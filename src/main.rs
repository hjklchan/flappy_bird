use bevy::prelude::*;
use flappy_bird::camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::plugin)
        .run();
}
