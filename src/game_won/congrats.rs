use bevy::app::Plugin;

use bevy::prelude::*;

use crate::main_menu::{BLACK, BORDER_COLOR, DARK_PURPLE, LIGHT_TEAL};

use super::GameWonState;

pub struct CongratsPlugin;

impl Plugin for CongratsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameWonState::Congrats),
            (reset_camera, spawn_message),
        );
    }
}

#[derive(Component)]
pub struct CongratsUi;

#[derive(Component)]
pub struct MessageContainerUi;

#[derive(Component)]
pub struct FlowerImageContainer;

#[derive(Component)]
pub struct FlowerImage;

fn spawn_message(mut commands: Commands, assets: Res<AssetServer>) {
    // let font = assets.load("fonts/spectral/spectral_medium.ttf");

    let mut color: Color = BLACK.into();
    color.set_a(0.7);

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
            background_color: color.into(),
            ..default()
        },
        CongratsUi,
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
                width: Val::Px(532.),
                height: Val::Px(190.),
                // justify_content: JustifyContent::Center,
                // align_items: AlignItems::Center,
                // align_self: AlignSelf::Center,
                ..default()
            },
            image: UiImage::new(assets.load("sprites/ui/2.png")),
            ..default()
        },
        // FlowerImage {},
    );

    let panel = (NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect {
                top: Val::Px(10.),
                left: Val::Px(50.),
                right: Val::Px(50.),
                bottom: Val::Px(50.),
            },
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            // justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    },);

    let mut panel_color: Color = DARK_PURPLE.into();
    panel_color.set_a(0.7);

    let panel_inner = (NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect {
                top: Val::Px(50.),
                left: Val::Px(50.),
                right: Val::Px(50.),
                bottom: Val::Px(50.),
            },
            border: UiRect::all(Val::Px(2.)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        border_color: BORDER_COLOR.into(),
        background_color: color.into(),
        ..default()
    },);

    let font = assets.load("fonts/spectral/spectral_medium.ttf");
    let msg = "I'm not really the best person for this type of message, but I earnestly wish you both the best of luck in your life together. You've shared many adventures, and are sure to experience many more, I'm sure.\n
I also hope that you got to this point in the game, as creating this took a helluva long time! XD\n
May life remain indefinitely exciting for you both.\n
All the best, Ross.";

    let section: TextSection = TextSection {
        value: msg.into(),
        style: TextStyle {
            font: font,
            font_size: 42.0,
            color: LIGHT_TEAL.into(),
        },
    };

    let text_msg = TextBundle {
        text: Text {
            sections: Vec::from([section]),
            ..default()
        },
        ..Default::default()
    };

    commands.spawn(menu_parent).with_children(|commands| {
        commands
            .spawn(image_top_container)
            .with_children(|commands| {
                commands.spawn(image_top);
                commands.spawn(panel).with_children(|commands| {
                    commands.spawn(panel_inner).with_children(|commands| {
                        commands.spawn(text_msg);
                    });
                });
            });
    });
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

// fn despawn_main_menu_ui(mut commands: Commands, ui_query: Query<Entity, With<CongratsUi>>) {
//     for ui in &ui_query {
//         commands.entity(ui).despawn_recursive();
//     }
//     // for music in &music_query {
//     //     commands.entity(music).despawn_recursive();
//     // }
//     // for exit in &eb_query {
//     //     commands.entity(exit).despawn_recursive();
//     // }
//     // for start in &sb_query {
//     //     commands.entity(start).despawn_recursive();
//     // }
// }
