// use crate::player::CharacterLife;
use bevy::prelude::*;

use super::{
    player::{CanLevel, Player},
    weapons::get_available_weapons,
};
use crate::{
    game::GamePlayState,
    main_menu::{
        BLACK, BLUE, BORDER_COLOR, DARK_PURPLE, LIGHT_BLUE, LIGHT_TEAL, PURPLE, PURPLISH, WHITE,
    },
    sprite::{Health, ProjectileCategory, Weapon},
    GameState,
};

pub struct LvlUpUiPlugin;

impl Plugin for LvlUpUiPlugin {
    fn build(&self, app: &mut App) {
        println!("Load game play ui plugin");
        app.add_systems(OnEnter(GamePlayState::LevelUp), on_level_up)
            .add_systems(
                Update,
                weapon_button_select.run_if(in_state(GamePlayState::LevelUp)),
            )
            .add_systems(OnExit(GamePlayState::LevelUp), unload)
            .add_systems(OnExit(GameState::Gameplay), unload);

        // app.add_systems(OnEnterStartup, ui_setup)
        //     .add_systems(PostUpdate, ui_update);
    }
}

#[derive(Component)]
struct LvlUpContainer;

#[derive(Component)]
struct TitleUi;

#[derive(Component)]
struct LvlUpDialogContainer;

#[derive(Component)]
struct UpgradesContainer;

#[derive(Component)]
struct WeaponButtonUI {
    weapon: Weapon,
}

#[derive(Component)]
struct WeaponTitleUi;

#[derive(Component)]
struct WeaponIconUi;

#[derive(Component)]
struct WeaponDescUi;

fn get_dialog_container() -> (NodeBundle, LvlUpDialogContainer, Name) {
    (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                width: Val::Px(1200.),
                height: Val::Px(700.),
                border: UiRect::all(Val::Px(2.)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BORDER_COLOR.into(),
            background_color: PURPLISH.into(),
            ..default()
        },
        LvlUpDialogContainer,
        Name::new("Lvl Up Dialog UI"),
    )
}

fn get_upgrades_container() -> (NodeBundle, UpgradesContainer, Name) {
    (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                flex_basis: Val::Percent(50.),
                flex_direction: FlexDirection::Column,
                margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(20.), Val::Px(0.)),
                ..default()
            },
            ..default()
        },
        UpgradesContainer,
        Name::new("Upgrades List"),
    )
}

fn get_title(lvl: u32, assets: &Res<AssetServer>) -> (TextBundle, Name, TitleUi) {
    let font = assets.load("fonts/patua_one/patuaone.ttf");

    let section = TextSection {
        value: format!("You reached level {}!", lvl),
        style: TextStyle {
            font: font,
            font_size: 38.0,
            color: LIGHT_TEAL.into(),
        },
    };

    (
        TextBundle {
            text: Text {
                sections: Vec::from([section]),
                ..default()
            },
            ..Default::default()
        },
        Name::new("Level number"),
        TitleUi,
    )
}

fn get_weapon_title(
    weapon: &Weapon,
    assets: &Res<AssetServer>,
) -> (TextBundle, Name, WeaponTitleUi) {
    let font = assets.load("fonts/patua_one/patuaone.ttf");

    let section = TextSection {
        value: weapon.name.clone(),
        style: TextStyle {
            font: font,
            font_size: 24.0,
            color: LIGHT_TEAL.into(),
        },
    };

    (
        TextBundle {
            text: Text {
                sections: Vec::from([section]),
                ..default()
            },
            ..Default::default()
        },
        Name::new(weapon.name.clone()),
        WeaponTitleUi,
    )
}

fn get_weapon_desc(weapon: &Weapon, assets: &Res<AssetServer>) -> (TextBundle, Name, WeaponDescUi) {
    let font = assets.load("fonts/patua_one/patuaone.ttf");

    let section = TextSection {
        value: weapon.name.clone(),
        style: TextStyle {
            font: font,
            font_size: 18.0,
            color: WHITE.into(),
        },
    };

    (
        TextBundle {
            text: Text {
                sections: Vec::from([section]),

                ..default()
            },
            ..Default::default()
        },
        Name::new(weapon.name.clone()),
        WeaponDescUi,
    )
}

fn get_lvl_up_container() -> (NodeBundle, LvlUpContainer, Name) {
    (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                left: Val::Px(0.),
                top: Val::Px(0.),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            background_color: BLACK.into(),
            border_color: BORDER_COLOR.into(),
            ..default()
        },
        LvlUpContainer,
        Name::new("Level Up UI Container"),
    )
}

fn get_weapon_button(weapon: &Weapon) -> (ButtonBundle, WeaponButtonUI) {
    (
        ButtonBundle {
            style: Style {
                height: Val::Px(500.),
                width: Val::Px(500.),
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(10.),
                // align_content: AlignContent::Center,
                // justify_items: JustifyItems::Center,
                padding: UiRect::all(Val::Px(5.)),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            background_color: LIGHT_BLUE.into(),
            border_color: PURPLISH.into(),
            ..default()
        },
        WeaponButtonUI {
            weapon: weapon.clone(),
        },
    )
}

fn on_level_up(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut player_query: Query<(&Player, &CanLevel), With<Player>>,
) {
    let (player, lvl) = player_query.single_mut();

    let weapon_new = get_available_weapons(&player.weapons, 1);

    let mut btns: Vec<(ButtonBundle, WeaponButtonUI)> = vec![];

    commands
        .spawn(get_lvl_up_container())
        .with_children(|commands| {
            commands
                .spawn(get_dialog_container())
                .with_children(|commands| {
                    commands.spawn(get_title(lvl.level, &assets));
                    commands
                        .spawn(get_upgrades_container())
                        .with_children(|commands| {
                            for weapon in weapon_new {
                                let btn = get_weapon_button(&weapon);

                                commands.spawn(btn).with_children(|commands| {
                                    commands.spawn(get_weapon_title(&weapon, &assets));
                                    commands.spawn(get_weapon_desc(&weapon, &assets));
                                });
                            }
                        });
                });
        });
}

fn weapon_button_select(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &WeaponButtonUI,
        ),
        (With<Button>, With<WeaponButtonUI>),
    >,
    mut next_play_state: ResMut<NextState<GamePlayState>>,
    // mut next_fade_state: ResMut<NextState<FadeState>>,
    mut player_query: Query<(&mut Player), With<Player>>,
) {
    let mut player = player_query.single_mut();

    for (interaction, mut color, mut border_color, weapon_btn) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = LIGHT_BLUE.into();
                *border_color = LIGHT_BLUE.into();

                player.weapons.push(weapon_btn.weapon.clone());

                // Revert to normal play.
                next_play_state.set(GamePlayState::Started);

                // ADD WEAPON TO PLAYER WEAPONS
                // next_fade_state.set(FadeState::FadeToGame);
                // next_game_state.set(GameState::CharacterSelect);
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

// fn get_available_weapons(assets: &Res<AssetServer>) -> Vec<(TextBundle, Name, LvlText)> {
//     let font = assets.load("fonts/patua_one/patuaone.ttf");

//     let section = TextSection {
//         value: format!("Level: {}", 0.),
//         style: TextStyle {
//             font: font,
//             font_size: 38.0,
//             color: LIGHT_TEAL.into(),
//         },
//     };

//     (
//         TextBundle {
//             text: Text {
//                 sections: Vec::from([section]),
//                 ..default()
//             },
//             ..Default::default()
//         },
//         Name::new("Level number"),
//         LvlText,
//     )
// }

// fn get_available_weapon(weapons: Vec<ProjectileCategory>) -> TextBundle {}

fn unload(mut ui_query: Query<Entity, With<LvlUpContainer>>, mut commands: Commands) {
    for ui in &mut ui_query.iter_mut() {
        commands.entity(ui).despawn_recursive();
    }
}
