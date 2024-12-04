pub mod common;
pub mod constants;
pub mod background;
pub mod components;
pub mod states;
pub mod menu;
pub mod camera;
pub mod game;

use bevy::prelude::{Bundle, Component, Resource};

#[derive(Resource)]
pub struct Game {
    selected_hero: Option<Hero>,
    score: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            selected_hero: None,
            score: 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Hero {
    pub key: &'static str,
    pub name: &'static str,
    pub image: &'static str,
}

#[derive(Component)]
pub enum MenuButtonAction {
    SelectHero,
    Settings,
}

#[derive(Component)]
pub enum Heroes {
    HuangZhao,
    XiaoMingYan,
}

pub const HERO_LIST: [Hero; 2] = [
    Hero {
        key: "hz",
        name: "Huang Zhao",
        image: "hz.png",
    },
    Hero {
        key: "xmy",
        name: "Xiao Ming Yan",
        image: "xmy.png",
    }
];