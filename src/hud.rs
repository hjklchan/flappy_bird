use bevy::prelude::*;

use crate::{
    components::Score,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    states::GameState,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(HudPlugin);
}

struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_score_number_text);
    }
}

fn spawn_score_number_text(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    dbg!("Should spawn score number text");
    
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
