use crate::components::Background;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::states::GameState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(BackgroundPlugin);
}

struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
        app.add_systems(
            FixedUpdate,
            move_background.run_if(in_state(GameState::Menu).or(in_state(GameState::InGame))),
        );
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
        Transform {
            translation: Vec3::ZERO,
            ..default()
        },
        Background,
    ));
}

fn move_background(mut query: Query<&mut Transform, With<Background>>, time: Res<Time>) {
    let mut background = query.single_mut();

    background.translation.x -= 20.0 * time.delta_secs();

    if background.translation.x < -288.0 {
        background.translation.x = 0.0;
    }
}
