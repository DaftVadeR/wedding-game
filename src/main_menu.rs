use bevy::app::{AppExit, Plugin};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (reset_camera, spawn_main_menu_ui),
        )
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu_ui)
        // .add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui)
        // .add_systems(OnExit(GameState::GameOver), despawn_game_over_ui)
        .add_systems(Startup, play_music)
        .add_systems(Update, start_button_system)
        .add_systems(Update, exit_button_system);
    }
}

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct StartButtonUI;

#[derive(Component)]
pub struct ExitButtonUI;

#[derive(Component)]
pub struct SelectCharacterUI;

#[derive(Component)]
pub struct CharacterContainerUI;

#[derive(Component)]
struct MyMusic;

fn spawn_main_menu_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/spectral/spectral_medium.ttf");

    let menu_parent = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                /*position: UiRect {
                    left: Val::Percent(47.0),
                    right: Val::Auto,
                    top: Val::Percent(45.0),
                    bottom: Val::Auto,
                },*/
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        },
        MainMenuUI,
    );

    let start_button = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Px(80.),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                margin: UiRect::bottom(Val::Px(40.)),
                ..default()
            },

            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        StartButtonUI,
    );

    // let menu_title = TextBundle::from_section(
    //     "The Valiant Duo",
    //     TextStyle {
    //         font: font.clone(),
    //         font_size: 64.0,
    //         color: Color::rgb(0.9, 0.9, 0.9),
    //     },
    // );

    let start_button_text = TextBundle::from_section(
        "Start Game!",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let exit_button = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        ExitButtonUI,
    );

    let exit_button_text = TextBundle::from_section(
        "Exit",
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    commands.spawn(menu_parent).with_children(|commands| {
        commands.spawn(start_button).with_children(|commands| {
            commands.spawn(start_button_text);
        });
        commands.spawn(exit_button).with_children(|commands| {
            commands.spawn(exit_button_text);
        });
    });

    commands.spawn((
        AudioBundle {
            source: assets.load("music/loopnice2.ogg"),
            ..default()
        },
        MyMusic,
    ));
}

const NORMAL_BUTTON: Color = Color::CRIMSON;
const HOVERED_BUTTON: Color = Color::TURQUOISE;
const PRESSED_BUTTON: Color = Color::RED;

fn start_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<StartButtonUI>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::StartingCorridor);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn exit_button_system(
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<ExitButtonUI>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                exit.send(AppExit);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn reset_camera(
    // query: Query<(&Transform) /*(With<Player>)*/>,
    mut camera_query: Query<&mut Transform, With<Camera> /*, Without<Player>*/>,
) {
    // let player_transform = query.single();
    let mut camera_transform = camera_query.single_mut();

    //
    camera_transform.translation = Vec3::new(0., 0., 0.);
}

fn play_music(music_controller: Query<&AudioSink, With<MyMusic>>, time: Res<Time>) {
    if let Ok(sink) = music_controller.get_single() {
        // sink.toggle();
    }
}

fn despawn_main_menu_ui(mut commands: Commands, ui: Query<Entity, With<MainMenuUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}
