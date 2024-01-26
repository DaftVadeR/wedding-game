use bevy::app::{AppExit, Plugin};
use bevy::prelude::*;

use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui)
            .add_systems(OnExit(GameState::GameOver), despawn_game_play_items)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_ui);
    }
}

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct GameOverButtonUI;

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
        background_color: Color::DARK_GRAY.into(),
        ..default()
    };

    let button = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(15.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        GameOverButtonUI,
    );

    let title_text = TextBundle::from_section(
        "Game Over!",
        TextStyle {
            font: font.clone(),
            font_size: 64.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let button_text = TextBundle::from_section(
        "Back to Menu",
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    commands.spawn(menu_parent).with_children(|commands| {
        commands.spawn(menu_title).with_children(|commands| {
            commands.spawn(title_text);
            commands.spawn(button).with_children(|commands| {
                commands.spawn(button_text);
            });
        });
    });
}

/* Despawns enemies, player, ui and projectiles for new game to start */
fn despawn_game_play_items(mut commands: Commands, entities: Query<Entity>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_game_over_ui(mut commands: Commands, ui: Query<Entity, With<GameOverUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}
