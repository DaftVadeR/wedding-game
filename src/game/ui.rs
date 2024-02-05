// use crate::player::CharacterLife;
use bevy::prelude::*;

use super::player::{CanLevel, Player};
use crate::{
    game::GamePlayState,
    main_menu::{BORDER_COLOR, DARK_PURPLE, LIGHT_TEAL, PURPLISH},
    sprite::Health,
    GameState,
};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        println!("Load game play ui plugin");
        app.add_systems(OnEnter(GamePlayState::Init), ui_setup)
            .insert_resource(Time::<Fixed>::from_seconds(0.5))
            .add_systems(
                FixedUpdate,
                ui_update.run_if(in_state(GamePlayState::Started)),
            )
            .add_systems(OnExit(GameState::Gameplay), unload);

        // app.add_systems(OnEnterStartup, ui_setup)
        //     .add_systems(PostUpdate, ui_update);
    }
}

#[derive(Component)]
struct HealthUiValue;

#[derive(Component)]
struct HealthUiBar;

#[derive(Component)]
struct UiContainer;

#[derive(Component)]
struct LvlContainer;

#[derive(Component)]
struct LvlText;

fn ui_setup(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_container = (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                left: Val::Px(0.),
                top: Val::Px(0.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        UiContainer,
        Name::new("UI Container"),
    );

    // Health bar
    let parent_node = (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                width: Val::Px(300.),
                height: Val::Px(35.),
                left: Val::Px(40.),
                right: Val::Auto,
                bottom: Val::Px(40.),
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            border_color: BORDER_COLOR.into(),
            background_color: PURPLISH.into(),
            ..default()
        },
        HealthUiBar,
        Name::new("Health Bar UI"),
    );

    let health_node = (
        NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BORDER_COLOR.into(),
            ..default()
        },
        HealthUiValue,
        Name::new("Health Bar Filled UI"),
    );

    let lvl_container = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                padding: UiRect {
                    top: Val::Px(10.),
                    left: Val::Px(20.),
                    right: Val::Px(20.),
                    bottom: Val::Px(10.),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BORDER_COLOR.into(),
            background_color: DARK_PURPLE.into(),
            ..default()
        },
        LvlContainer,
        Name::new("Lvl Container"),
    );

    let font = assets.load("fonts/patua_one/patuaone.ttf");

    let section = TextSection {
        value: format!("Level: {}", 0.),
        style: TextStyle {
            font: font,
            font_size: 38.0,
            color: LIGHT_TEAL.into(),
        },
    };

    let lvl = (
        TextBundle {
            text: Text {
                sections: Vec::from([section]),
                ..default()
            },
            ..Default::default()
        },
        Name::new("Level number"),
        LvlText,
    );

    commands.spawn(ui_container).with_children(|commands| {
        commands.spawn(parent_node).with_children(|commands| {
            commands.spawn(health_node);
        });

        commands.spawn(lvl_container).with_children(|commands| {
            commands.spawn(lvl);
        });
    });
}

fn ui_update(
    // mut commands: Commands,
    // mut game_state: ResMut<NextState<GameState>>,
    player_query: Query<(&Health, &CanLevel), With<Player>>,
    mut ui_health_query: Query<&mut Style, With<HealthUiValue>>,
    mut ui_lvl_query: Query<&mut Text, With<LvlText>>,
    assets: Res<AssetServer>,
) {
    let (health, lvl) = player_query.single();

    // Health
    let mut health_block_style = ui_health_query.single_mut();

    health_block_style.width = Val::Percent(health.0);

    // println!("Health: {}", health.0);

    // Level
    let mut text = ui_lvl_query.single_mut();

    let font = assets.load("fonts/patua_one/patuaone.ttf");

    let section = TextSection {
        value: format!("Level: {}", lvl.level),
        style: TextStyle {
            font: font,
            font_size: 38.0,
            color: LIGHT_TEAL.into(),
        },
    };

    text.sections = Vec::from([section]);
}

fn unload(mut ui_query: Query<Entity, With<UiContainer>>, mut commands: Commands) {
    for ui in &mut ui_query.iter_mut() {
        commands.entity(ui).despawn_recursive();
    }
}
