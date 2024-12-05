use bevy::prelude::*;

use crate::{states::GameState, Game};

pub fn plugin(app: &mut App) {
    app.add_plugins(GamePlugin);
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, press_key_to_start_game.run_if(is_select_hero));
    }
}

fn press_key_to_start_game(
    button_input: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // Press <Space> to start the game
    if button_input.just_pressed(KeyCode::Space) {
        // Just change the state to [`GameState::InGame`]
        next_game_state.set(GameState::InGame);
    }
}

// Check to see if the hero is selected
fn is_select_hero(game: Res<Game>) -> bool {
    game.selected_hero.is_some()
}
