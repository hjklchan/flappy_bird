use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    // LoadAssets, // TODO - Requires implementation of the load assets plugin
    #[default]
    Menu,
    InGame,
}