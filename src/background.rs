use bevy::asset::io::memory::Value::Vec;
use bevy::prelude::*;
use crate::components::Background;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn plugin(app: &mut App) {
    app.add_plugins(BackgroundPlugin);
}

struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/background.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            custom_size: Some(Vec2::new(WINDOW_WIDTH + 288.0 * 2.0, WINDOW_HEIGHT)),
            ..default()
        },
        Background,
    ));
}
