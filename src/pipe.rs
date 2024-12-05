use bevy::prelude::*;

use crate::states::{GameState, PlayingState};

pub fn plugin(app: &mut App) {
    app.add_plugins(PipePlugin);
}

struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_pipe.run_if(in_state(GameState::InGame).or(in_state(PlayingState::Start))),
        );
    }
}

/// Update in state [`crate::state::GameState::InGame`]
fn spawn_pipe() {}
