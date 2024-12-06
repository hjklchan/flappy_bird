use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct UpperPipe;

#[derive(Component)]
pub struct BottomPipe;

#[derive(Component)]
pub enum Score {
    Digit,     // 1
    Tenth,     // 1x
    Hundredth, // 1xx
    Infinite,
}

impl Score {
    pub fn from_index(index: i32) -> Score
    {
        match index {
            0 => Score::Digit,
            1 => Score::Tenth,
            2 => Score::Hundredth,
            _ => Score::Infinite,
        }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct DevLogLayout;
