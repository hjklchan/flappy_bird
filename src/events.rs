use bevy::prelude::*;

pub mod score {
    use super::*;

    #[derive(Event)]
    pub struct Add {
        pub step: usize,
    }

    impl Add {
        pub fn with_step(step: usize) -> Self {
            Self { step }
        }
    }
}
