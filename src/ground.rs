//! The Ground doesn't need to be moved

use bevy::prelude::*;

use crate::{components::Ground, constants::WINDOW_WIDTH};

pub fn plugin(app: &mut App) {
    app.add_plugins(GroundPlugin);
}

// TODO - Possible have the Background plugin,
// and Ground plugin merged into one plugin
struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/base.png"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, 112.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, -256.0, 1.0),
        Ground,
    ));
}
