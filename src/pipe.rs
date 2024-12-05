use bevy::prelude::*;
use rand::Rng;
use crate::{
    states::{GameState, PlayingState},
};
use crate::components::{BottomPipe, UpperPipe};
use crate::constants::{PIPE_HALF_HEIGHT, WINDOW_WIDTH};

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
    let rng = rand::thread_rng();

    spawn_timer.0.tick(time.delta());

    // Spawn an entity every second
    if spawn_timer.0.finished() {
        let x = WINDOW_WIDTH / 2.0 + -30.0;
        let mut rng = rand::thread_rng();

        let mut rand_y = || {
            let min_y = -200.0 - PIPE_HALF_HEIGHT + 20.0;
            let max_y = -200.0 + PIPE_HALF_HEIGHT - 20.0;
            let bottom_y = rng.gen_range(min_y..max_y);
            (bottom_y, bottom_y + 420.0)
        };

        let (bottom_y, upper_y) = rand_y();

        // Spawn upper pipe
        commands.spawn((
            Sprite {
                image: sprite_handle.clone(),
                ..default()
            },
            Transform {
                translation: Vec3::new(x, upper_y, 0.5),
                ..default()
            },
            UpperPipe,
        ));

        // Spawn bottom pipe
        commands.spawn((
            Sprite {
                image: sprite_handle.clone(),
                ..default()
            },
            Transform {
                translation: Vec3::new(x, bottom_y, 0.5),
                ..default()
            },
            BottomPipe,
        ));
    }
}

fn pipe_moving(
    mut upper_pipe_query: Query<&mut Transform, With<UpperPipe>>,
    mut bottom_pipe_query: Query<&mut Transform, (With<BottomPipe>, Without<UpperPipe>)>,
    time: Res<Time>,
) {
    let speed = 30.0;
    let delta = time.delta_secs();

    for mut transform in bottom_pipe_query.iter_mut() {
        // TODO
    }

    for mut transform in upper_pipe_query.iter_mut() {
        // TODO
    }
}