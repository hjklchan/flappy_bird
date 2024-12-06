//! The hud.rs should integration into ui.rs
//!
//! Because the HUB is a part of UI management

use std::time::Duration;

use bevy::prelude::*;

use crate::states::PlayingState;

pub fn plugin(app: &mut App) {
    app.add_plugins(UiPlugin);
}

struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingState::GameOver), spawn_game_over_text);
        app.add_systems(Update, game_over_text_animation.after(spawn_game_over_text));

        app.add_systems(OnEnter(PlayingState::Ready), spawn_count_down_layer);
    }
}

#[derive(Component)]
struct GameOverText {
    blink_timer: Timer,
}

fn spawn_game_over_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/game-over.png"),
            ..default()
        },
        Transform {
            translation: Vec3::Z * 1.5,
            ..default()
        },
        Visibility::Visible,
        GameOverText {
            blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        },
    ));
}

fn game_over_text_animation(
    mut query: Query<(&mut GameOverText, &mut Visibility)>,
    time: Res<Time>,
) {
    if let Ok((mut game_over_text, mut visibility)) = query.get_single_mut() {
        game_over_text.blink_timer.tick(time.delta());

        if game_over_text.blink_timer.finished() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

#[derive(Component)]
struct CountDownLayer;

// TODO - Do testing
fn spawn_count_down_layer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite {
            image: asset_server.load("texture/numbers.png"),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 3, // Counting down from 3 (Not sure)
            }),
            ..default()
        },
        Transform {
            translation: Vec3::Z * 1.5,
            ..default()
        },
        CountDownLayer,
    ));
}
