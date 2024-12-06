use std::f32::consts::PI;

use crate::components::{BottomPipe, UpperPipe};
use crate::constants::{PIPE_HALF_HEIGHT, WINDOW_WIDTH};
use crate::states::{GameState, PlayingState};
use bevy::prelude::*;
use rand::Rng;

pub fn plugin(app: &mut App) {
    app.add_plugins(PipePlugin);
}

struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>();
        app.add_systems(
            Update,
            (spawn_pipe, pipe_moving, despawn_if_out_of_bound)
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(PlayingState::Start)),
        );
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.8, TimerMode::Repeating))
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
    let mut rng = rand::thread_rng();

    spawn_timer.0.tick(time.delta());

    // Spawn an entity every second
    if spawn_timer.0.finished() {
        let x = WINDOW_WIDTH / 2.0 + -30.0;
        // let mut rng = rand::thread_rng();

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
                rotation: Quat::from_rotation_z(PI),
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
    let speed = 110.0;
    let delta = time.delta_secs();

    for mut transform in bottom_pipe_query.iter_mut() {
        transform.translation.x -= speed * delta;
    }

    for mut transform in upper_pipe_query.iter_mut() {
        transform.translation.x -= speed * delta;
    }
}

#[allow(clippy::type_complexity)]
fn despawn_if_out_of_bound(
    mut commands: Commands,
    bottom_pipe_query: Query<(Entity, &Transform), With<BottomPipe>>,
    upper_pipe_query: Query<(Entity, &Transform), (With<UpperPipe>, Without<BottomPipe>)>,
) {
    let expect_x = -(WINDOW_WIDTH / 2.0);

    for (entity, transform) in bottom_pipe_query.iter() {
        if transform.translation.x < expect_x {
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform) in upper_pipe_query.iter() {
        if transform.translation.x < expect_x {
            commands.entity(entity).despawn();
        }
    }
}
