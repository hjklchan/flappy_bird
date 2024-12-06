use bevy::prelude::*;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// ## Custom extension functions created for Bevy
pub mod condition_pro {
    use super::*;

    /// Similar to in_state,
    /// it's just an inverse operation
    /// 
    /// ### For example:
    /// 
    /// ```rust
    /// // like: !in_state(PlayingState::GameOver)
    /// some_system.run_if(not_in_state(PlayingState::GameOver));
    /// ```
    pub fn not_in_state<S: States>(state: S) -> impl FnMut(Option<Res<State<S>>>) -> bool + Clone {
        move |current_state: Option<Res<State<S>>>| match current_state {
            Some(current_state) => !(*current_state == state),
            None => false,
        }
    }
}
