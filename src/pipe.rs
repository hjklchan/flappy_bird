use bevy::prelude::*;

use crate::{
    components::Pipe,
    states::{GameState, PlayingState},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(PipePlugin);
}

struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>();
        app.add_systems(
            Update,
            spawn_pipe
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(PlayingState::Start)),
        );
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.2, TimerMode::Repeating))
    }
}

/// spawn_pipe system
///
/// Update in state [`GameState::InGame`] and [`PlayingState::Start`]
fn spawn_pipe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    let sprite_handle = asset_server.load("texture/pipe.png");

    spawn_timer.0.tick(time.delta());

    // Spawn an entity every second
    if spawn_timer.0.finished() {
        dbg!("Will spawn a new pipe");
        commands.spawn((
            Sprite {
                image: sprite_handle.clone(),
                ..default()
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.5),
                ..default()
            },
            Pipe,
        ));
    }
}
