use crate::GameState;
use bevy::app::{AppExit, Plugin};
use bevy::audio::{PlaybackMode, Volume, VolumeLevel};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (reset_camera, spawn_main_menu_ui),
        )
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu_ui)
        .add_systems(
            Update,
            (start_button_system, exit_button_system).run_if(in_state(GameState::MainMenu)),
        );
    }
}

#[derive(Component)]
pub struct MainMenuUi;

#[derive(Component)]
pub struct StartButtonUI;

#[derive(Component)]
pub struct ExitButtonUI;

#[derive(Component)]
pub struct MyMusic;

#[derive(Component)]
pub struct FlowerImageContainer;

#[derive(Component)]
pub struct FlowerImage;

pub const DARK_PURPLE: Color = Color::rgb(0.165, 0.09, 0.231);
pub const PURPLE: Color = Color::rgb(0.247, 0.173, 0.373);
pub const PURPLISH: Color = Color::rgb(0.298, 0.361, 0.529);
pub const BLUE: Color = Color::rgb(0.267, 0.247, 0.482);
pub const LIGHT_BLUE: Color = Color::rgb(0.412, 0.502, 0.62);
pub const LIGHT_TEAL: Color = Color::rgb(0.584, 0.773, 0.675);

pub const BORDER_COLOR: Color = Color::rgb(0.828, 0.606, 0.161);
pub const BLACK: Color = Color::rgb(0., 0., 0.);
pub const WHITE: Color = Color::rgb(1., 1., 1.);

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
            background_color: DARK_PURPLE.into(),
            ..default()
        },
        MainMenuUi,
    );

    let image_top_container = (
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
                // justify_content: JustifyContent::Center,
                ..default()
            },
            // background_color: BACKGROUND_COLOR.into(),
            ..default()
        },
        FlowerImageContainer {},
    );

    let image_top = (
        ImageBundle {
            style: Style {
                width: Val::Px(618.),
                height: Val::Px(256.),
                // justify_content: JustifyContent::Center,
                // align_items: AlignItems::Center,
                // align_self: AlignSelf::Center,
                ..default()
            },
            image: UiImage::new(assets.load("sprites/ui/1.png")),
            ..default()
        },
        // FlowerImage {},
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
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },

            background_color: BLUE.into(),
            border_color: BLUE.into(),
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
            color: LIGHT_TEAL.into(),
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
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },

            background_color: BLUE.into(),
            border_color: BLUE.into(),
            ..default()
        },
        ExitButtonUI,
    );

    let exit_button_text = TextBundle::from_section(
        "Exit",
        TextStyle {
            font,
            font_size: 40.0,
            color: LIGHT_TEAL.into(),
        },
    );

    commands.spawn((
        AudioBundle {
            source: assets.load("music/menu.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Absolute(VolumeLevel::new(0.5)),
                ..default()
            },
        },
        MyMusic,
    ));

    commands.spawn(menu_parent).with_children(|commands| {
        commands
            .spawn(image_top_container)
            .with_children(|commands| {
                commands.spawn(image_top);
            });

        commands.spawn(start_button).with_children(|commands| {
            commands.spawn(start_button_text);
        });
        commands.spawn(exit_button).with_children(|commands| {
            commands.spawn(exit_button_text);
        });
    });
}

fn start_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<Button>, With<StartButtonUI>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
    // mut next_fade_state: ResMut<NextState<FadeState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_BLUE.into();
                // next_fade_state.set(FadeState::FadeToGame);
                next_game_state.set(GameState::CharacterSelect);
            }
            Interaction::Hovered => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_TEAL.into();
            }
            Interaction::None => {
                *color = BLUE.into();
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
    mut next_state: ResMut<NextState<GameState>>,
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
                *color = BLUE.into();
                *border_color = BLUE.into();
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

fn despawn_main_menu_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<MainMenuUi>>,
    music_query: Query<Entity, With<MyMusic>>,
    sb_query: Query<Entity, With<StartButtonUI>>,
    eb_query: Query<Entity, With<ExitButtonUI>>,
) {
    for ui in &ui_query {
        commands.entity(ui).despawn_recursive();
    }
    // for music in &music_query {
    //     commands.entity(music).despawn_recursive();
    // }
    // for exit in &eb_query {
    //     commands.entity(exit).despawn_recursive();
    // }
    // for start in &sb_query {
    //     commands.entity(start).despawn_recursive();
    // }
}
