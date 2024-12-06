//! The hud.rs should integration into ui.rs
//!
//! Because the HUB is a part of UI management

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
