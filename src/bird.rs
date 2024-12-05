use bevy::prelude::*;

use crate::{
    components::Bird,
    states::{GameState, PlayingState},
    // Game as GameResource,
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
        app.add_systems(Update, spawn_count_down.run_if(in_state(GameState::InGame)));
        app.add_systems(Update, bird_animation);
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
        Bird,
    ));
}

fn spawn_count_down(
    time: Res<Time>,
    mut count_down_timer: ResMut<CountDownTimer>,
    mut next_start_game_state: ResMut<NextState<PlayingState>>,
) {
    count_down_timer.0.tick(time.delta());

    if count_down_timer.0.finished() {
        next_start_game_state.set(PlayingState::Ready);
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
