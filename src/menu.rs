use bevy::prelude::*;

use crate::{components::OnMainMenuScreen, states::GameState};

pub fn plugin(app: &mut App) {
    app.add_plugins(MenuPlugin);
}

struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();

        // When entering the [`GameState::Menu`] state,
        // switch the state to [`MenuState::Main`] instead of [`MenuState::Closed`]
        app.add_systems(OnEnter(GameState::Menu), setup_menu);

        // Enter main menu
        app.add_systems(OnEnter(MenuState::Main), setup_main_menu);
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    #[default]
    Closed,
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    SelectHero,
    Quit,
}

fn setup_menu(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::Main);
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    dbg!("setup main menu");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb_u8(0, 200, 0)),
                ))
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        Text::new("Rust Flappy Bird"),
                        TextFont {
                            font_size: 56.0,
                            ..default()
                        },
                        TextColor(Color::srgb_u8(250, 0, 0)),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    // Selection hero text
                    parent.spawn((
                        Text::new("Please select your hero:"),
                        TextFont::from_font_size(21.0),
                        TextColor::WHITE,
                    ));

                    // Display the hero selection
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(100.0),
                                // border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor(Color::WHITE),
                        ))
                        .with_children(|parent| {
                            // Hero list
                            parent.spawn(ImageNode {
                                image: asset_server.load("texture/heros/hz.png"),
                                ..default()
                            });
                        });

                    // Display the starting game text
                    parent.spawn((
                        Text::new("Press <space> to start the game"),
                        TextFont::from_font_size(21.0),
                        TextColor::WHITE,
                    ));

                    // TODO - Display Quit button
                });
        });
}
