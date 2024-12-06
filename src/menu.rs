use crate::{common::despawn_screen, components::{DevLogLayout, OnMainMenuScreen}, states::GameState, Game, Hero, Heroes};
use bevy::prelude::*;

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
        app.add_systems(OnExit(GameState::Menu), despawn_screen::<OnMainMenuScreen>);

        // Enter main menu
        app.add_systems(OnEnter(MenuState::Main), setup_main_menu);

        app.add_systems(OnEnter(MenuState::Main), setup_dev_log);
        app.add_systems(OnExit(MenuState::Main), despawn_screen::<DevLogLayout>);

        // Update the state when the menu is clicked
        app.add_systems(Update, menu_action.run_if(in_state(GameState::Menu)));

        // Enter the selecting hero
        app.add_systems(
            Update,
            select_hero_action.run_if(in_state(MenuState::SelectHero)),
        );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    SelectHero, // If the hero is clicked
    Settings,
    #[default]
    Closed,
}

#[derive(Component)]
enum MenuButtonAction {
    SelectHero,
    Quit,
}

fn setup_menu(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::Main);
}

fn setup_dev_log(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(120.0),
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                bottom: Val::Px(10.0),
                // TODO
                ..default()
            },
            ZIndex(99),
            BackgroundColor(Color::srgb_u8(0, 0, 0)),
            DevLogLayout,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Development Log(s)"),
                TextColor(Color::srgb_u8(250, 0, 0)),
                TextFont::from_font_size(17.0),
            ));
        });
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
                    // BackgroundColor(Color::srgb_u8(0, 200, 0)),
                ))
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        Text::new("Rust Flappy Bird"),
                        TextFont {
                            font_size: 40.0,
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
                        .spawn(Node {
                            justify_content: JustifyContent::SpaceAround,
                            ..default()
                        })
                        .with_children(|parent| {
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
                                    parent.spawn((
                                        ImageNode {
                                            image: asset_server.load("texture/heroes/hz.png"),
                                            ..default()
                                        },
                                        Node {
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..default()
                                        },
                                        MenuButtonAction::SelectHero,
                                        Heroes::HuangZhao,
                                        Button,
                                    ));

                                    parent.spawn((
                                        ImageNode {
                                            image: asset_server.load("texture/heroes/xmy.png"),
                                            ..default()
                                        },
                                        Node {
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..default()
                                        },
                                        MenuButtonAction::SelectHero,
                                        Heroes::XiaoMingYan,
                                        Button,
                                    ));
                                });
                        });

                    // Display the starting game text
                    parent.spawn((
                        Text::new("Press <space> to start the game"),
                        TextFont::from_font_size(16.0),
                        TextColor::WHITE,
                    ));

                    // TODO - Display Quit button
                });
        });
}

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::SelectHero => {
                    next_menu_state.set(MenuState::SelectHero);
                }
                MenuButtonAction::Quit => break,
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn select_hero_action(
    interaction_query: Query<(&Interaction, &Heroes), (Changed<Interaction>, With<Button>)>,
    mut game: ResMut<Game>,
) {
    for (interaction, hero) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            dbg!("Select Hero");
            match hero {
                Heroes::HuangZhao => {
                    dbg!("Select HuangZhao");
                    game.selected_hero = Some(Hero {
                        key: "hz",
                        name: "HuangZhao",
                        image: "hz.png",
                    })
                }
                Heroes::XiaoMingYan => {
                    dbg!("Select XiaoMingYan");
                    game.selected_hero = Some(Hero {
                        key: "xmy",
                        name: "XiaoMingYan",
                        image: "xmy.png",
                    })
                }
            }
        }
    }
}
