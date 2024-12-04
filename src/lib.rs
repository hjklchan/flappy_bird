pub mod common;
pub mod components;
pub mod states;
pub mod menu;
pub mod camera;
pub mod game;

pub struct Hero {
    pub key: &'static str,
    pub name: &'static str,
    pub image: &'static str,
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