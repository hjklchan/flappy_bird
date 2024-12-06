use bevy::prelude::*;

use crate::{
    states::{GameState, PlayingState},
    Game,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(GamePlugin {
        enable_restart_game: false,
    });
}

struct GamePlugin {
    // Debug options...
    enable_restart_game: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, press_key_to_start_game.run_if(is_select_hero));

        if self.enable_restart_game {
            app.add_systems(
                Update,
                press_space_restart_game.run_if(in_state(PlayingState::GameOver)),
            );
        }
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

fn press_space_restart_game(
    button_input: Res<ButtonInput<KeyCode>>,
    mut next_playing_state: ResMut<NextState<PlayingState>>,
) {
    if button_input.just_pressed(KeyCode::Space) {
        // Just change the state to [`GameState::Ready`]
        next_playing_state.set(PlayingState::Ready);
    }
}
