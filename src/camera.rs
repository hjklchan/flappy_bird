use bevy::prelude::*;

use crate::components::MainCamera;

pub fn plugin(app: &mut App) {
    app.add_plugins(CameraPlugin);
}

struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}
