use bevy::app::{AppExit, Plugin};
use bevy::prelude::*;

use crate::main_menu::{BLACK, BLUE, LIGHT_BLUE, LIGHT_TEAL};
use crate::GameState;

use super::GamePlayState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GamePlayState::GameOver),
            (reset_camera, spawn_game_over_ui),
        )
        .add_systems(OnEnter(GamePlayState::Restart), unload)
        .add_systems(
            Update,
            (
                restart_button_system,
                exit_button_system,
                skip_button_system,
            )
                .run_if(in_state(GamePlayState::GameOver)),
        )
        .add_systems(OnExit(GameState::Gameplay), unload);
    }
}

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct ExitButtonUI;

#[derive(Component)]
pub struct RestartButtonUI;

#[derive(Component)]
pub struct SkipButtonUI;

fn spawn_game_over_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/spectral/spectral_medium.ttf");

    let menu_parent = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::left(Val::Percent(3.0)),
                ..default()
            },
            background_color: BLACK.into(),
            ..default()
        },
        GameOverUI,
    );

    let menu_title = NodeBundle {
        style: Style {
            width: Val::Percent(70.0),
            height: Val::Percent(60.0),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        // background_color: Color::DARK_GRAY.into(),
        ..default()
    };

    let exit_button = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(15.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            border_color: BLUE.into(),
            background_color: BLUE.into(),
            ..default()
        },
        ExitButtonUI,
    );

    let restart_button = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(15.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            border_color: BLUE.into(),
            background_color: BLUE.into(),
            ..default()
        },
        RestartButtonUI,
    );

    let skip_button = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(15.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            border_color: BLUE.into(),
            background_color: BLUE.into(),
            ..default()
        },
        SkipButtonUI,
    );

    let title_text = TextBundle::from_section(
        "Game Over!",
        TextStyle {
            font: font.clone(),
            font_size: 64.0,
            color: LIGHT_TEAL.into(),
        },
    );

    let exit_text = TextBundle::from_section(
        "Exit",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: BLUE.into(),
        },
    );

    let restart_text = TextBundle::from_section(
        "Restart",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: BLUE.into(),
        },
    );

    let skip_text = TextBundle::from_section(
        "Skip to last part",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: BLUE.into(),
        },
    );

    commands.spawn(menu_parent).with_children(|commands| {
        commands.spawn(menu_title).with_children(|commands| {
            commands.spawn(title_text);
            commands.spawn(restart_button).with_children(|commands| {
                commands.spawn(restart_text);
            });
            commands.spawn(skip_button).with_children(|commands| {
                commands.spawn(skip_text);
            });
            commands.spawn(exit_button).with_children(|commands| {
                commands.spawn(exit_text);
            });
        });
    });
}

fn restart_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<Button>, With<RestartButtonUI>),
    >,
    mut next_game_state: ResMut<NextState<GamePlayState>>,
    // mut next_fade_state: ResMut<NextState<FadeState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_BLUE.into();
                // next_fade_state.set(FadeState::FadeToGame);
                next_game_state.set(GamePlayState::Restart);
            }
            Interaction::Hovered => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_TEAL.into();
            }
            Interaction::None => {
                *color = LIGHT_TEAL.into();
                *border_color = BLUE.into();
            }
        }
    }
}

fn exit_button_system(
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<Button>, With<ExitButtonUI>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_TEAL.into();
                exit.send(AppExit);
            }
            Interaction::Hovered => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_TEAL.into();
            }
            Interaction::None => {
                *color = LIGHT_TEAL.into();
                *border_color = BLUE.into();
            }
        }
    }
}

fn skip_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<Button>, With<SkipButtonUI>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_state: ResMut<NextState<GamePlayState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_TEAL.into();
                next_game_state.set(GameState::GameWon);
                next_state.set(GamePlayState::Unloaded);
                println!("Set to game won scene");
            }
            Interaction::Hovered => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_TEAL.into();
            }
            Interaction::None => {
                *color = LIGHT_TEAL.into();
                *border_color = BLUE.into();
            }
        }
    }
}

fn unload(mut commands: Commands, ui: Query<Entity, With<GameOverUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
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
