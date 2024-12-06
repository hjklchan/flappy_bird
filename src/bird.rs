use bevy::prelude::*;

use crate::{
    components::{Bird, Velocity},
    constants::{GROUND_HALF_HEIGHT, WINDOW_HEIGHT},
    states::{GameState, PlayingState},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(BirdPlugin);
}

struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CountDownTimer>();
        app.init_resource::<FlyingAnimationTimer>();
        app.add_systems(OnEnter(GameState::InGame), spawn_bird);
        app.add_systems(
            Update,
            spawn_count_down
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(PlayingState::Ready)),
        );
        app.add_systems(Update, bird_animation);
        app.add_systems(
            Update,
            (
                bird_jumping,
                bird_gravity,
                // Change to [`PlayingState::GameOver`] if the Bird hits the Ground
                // This system will not running if it in [`PlayingState::GameOver`]
                bird_hits_ground,
            )
                .run_if(in_state(PlayingState::Start)),
        );
    }
}

#[derive(Resource)]
struct CountDownTimer(Timer);

impl Default for CountDownTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

#[derive(Resource)]
struct FlyingAnimationTimer(Timer);

impl Default for FlyingAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
    step: usize,
}

fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_playing_state: ResMut<NextState<PlayingState>>,
    // game: Res<GameResource>,
) {
    // Origin sheet w/h: 102 * 24
    // FIXME - [`tile_size`] may not be fixed
    // Should get the w/h of the real image and put those into the
    let layout = TextureAtlasLayout::from_grid(UVec2::new(34, 24), 3, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite {
            // TODO - Read the Game resource and load hero asset
            // image: asset_server.load(game.selected_hero.unwrap().into()),
            image: asset_server.load("texture/bird.png"),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: 0,
            }),
            ..default()
        },
        AnimationIndices {
            first: 0,
            last: 2,
            step: 1,
        },
        Transform::from_xyz(0.0, 0.0, 2.0),
        Velocity {
            value: Vec3::Z * 400.0,
        },
        Bird,
    ));

    next_playing_state.set(PlayingState::Ready);
}

fn spawn_count_down(
    time: Res<Time>,
    mut count_down_timer: ResMut<CountDownTimer>,
    mut next_start_game_state: ResMut<NextState<PlayingState>>,
) {
    count_down_timer.0.tick(time.delta());

    if count_down_timer.0.finished() {
        next_start_game_state.set(PlayingState::Start);
    }
}

fn bird_animation(
    mut query: Query<(&mut Sprite, &AnimationIndices), With<Bird>>,
    time: Res<Time>,
    mut flying_animation_timer: ResMut<FlyingAnimationTimer>,
) {
    if let Ok((mut sprite, animation_indices)) = query.get_single_mut() {
        let delta = time.delta();

        flying_animation_timer.0.tick(delta);

        if flying_animation_timer.0.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == animation_indices.last {
                    animation_indices.first
                } else {
                    atlas.index + animation_indices.step
                };
            }
        }
    }
}

fn bird_jumping(
    mut query: Query<&mut Velocity, With<Bird>>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        // Jump
        if button_input.just_pressed(KeyCode::Space) {
            velocity.value.y = 400.0;
        }
    }
}

fn bird_gravity(mut query: Query<(&mut Transform, &mut Velocity), With<Bird>>, time: Res<Time>) {
    const G: f32 = 9.8;

    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let delta = time.delta_secs();
        let speed = 150.0;
        let delta_velocity = speed * G * delta;

        velocity.value.y -= delta_velocity;
        transform.translation.y += velocity.value.y * delta;
    }
}

fn bird_hits_ground(
    mut bird_query: Query<(&mut Transform, &mut Velocity), With<Bird>>,
    mut next_playing_state: ResMut<NextState<PlayingState>>,
) {
    const BIRD_HEIGHT: f32 = 24.0;
    let (mut bird_transform, mut bird_velocity) = bird_query.single_mut();

    let bird_kiss_y = bird_transform.translation.y - BIRD_HEIGHT / 2.0;
    let ground_kiss_y = -(WINDOW_HEIGHT / 2.0) + GROUND_HALF_HEIGHT / 2.0;

    if bird_kiss_y < ground_kiss_y {
        bird_velocity.value = Vec3::ZERO;
        bird_transform.translation.y = ground_kiss_y;

        // Change the state to [`PlayingState::GameOver`]
        next_playing_state.set(PlayingState::GameOver);
    }
}
