use bevy::prelude::*;

use crate::states::GameState;

pub fn plugin(app: &mut App) {
    app.add_plugins(GamePlugin);
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, start_game);
    }
}

fn start_game(
    button_input: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // Press <Space> to start the game
    if button_input.just_pressed(KeyCode::Space) {
        next_game_state.set(GameState::InGame);
    }
}
