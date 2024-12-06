use bevy::prelude::*;

use crate::{
    components::Score,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    events::score,
    states::{GameState, PlayingState},
    Game,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(HudPlugin);
}

struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_score_number_text);
        app.add_systems(
            Update,
            update_score_number_text.run_if(in_state(PlayingState::Start)),
        );
    }
}

fn spawn_score_number_text(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let left_top_pos = Vec3::new(-WINDOW_WIDTH / 2.0 + 15.0, WINDOW_HEIGHT / 2.0 - 30.0, 1.0);
    let sep_distance = 25.0;

    for i in 0..3 {
        let next_score_text_pos = Vec3::new(
            left_top_pos.x + i as f32 * sep_distance,
            left_top_pos.y,
            left_top_pos.z,
        );

        commands.spawn((
            Sprite::from_atlas_image(
                asset_server.load("texture/numbers.png"),
                TextureAtlas {
                    layout: layout_handle.clone(),
                    index: 0,
                },
            ),
            Transform {
                translation: next_score_text_pos,
                ..default()
            },
            Score::from_index(i),
        ));
    }
}

fn update_score_number_text(
    mut game: ResMut<Game>,
    mut score_evt: EventReader<score::Add>,
    mut query: Query<(&mut Sprite, &Score)>,
) {
    for add in score_evt.read() {
        let new_score = game.score + add.step;

        // TODO
        dbg!(format!("score: {}", new_score));
        // // Update component
        // for (mut sprite, score) in query.iter_mut() {
        //     if let Some(texture_atlas) = &mut sprite.texture_atlas {
        //         match score {
        //             Score::Digit => {
        //                 dbg!("Digit");
        //                 // texture_atlas.index = new_score % 10;
        //             }
        //             Score::Tenth => {
        //                 dbg!("Tenth");
        //                 // texture_atlas.index = new_score % 100;
        //             }
        //             Score::Hundredth => {
        //                 dbg!("Hundredth");
        //                 // texture_atlas.index = new_score % 1000;
        //             }
        //             _ => {}
        //         }
        //     }
        // }

        // Update score field in the Game resource
        game.score = new_score;
    }
}
