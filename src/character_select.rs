use crate::main_menu::{
    MyMusic, BLUE, BORDER_COLOR, DARK_PURPLE, LIGHT_BLUE, LIGHT_TEAL, PURPLE, PURPLISH,
};
use crate::util_fade::FadeState;
use crate::GameState;
use bevy::app::{AppExit, Plugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::ui::FocusPolicy;
use bevy::utils::HashMap;
use rand::Rng;

pub struct CharacterSelectPlugin;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum CharacterSelectState {
    #[default]
    Unloaded,
    Init, // Usually used for nested plugins to manage state - leave for now
    Started,
}

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum SelectedCharacterState {
    #[default]
    Ailsa,
    Lisa,
}

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CharacterSelectState>()
            .add_state::<SelectedCharacterState>()
            .add_systems(
                OnEnter(GameState::CharacterSelect),
                (reset_camera, spawn_select_scene),
            )
            .add_systems(OnExit(GameState::CharacterSelect), despawn_select_scene)
            .add_systems(
                Update,
                (character_select_system).run_if(in_state(GameState::CharacterSelect)),
            );
    }
}

#[derive(Debug, Component)]
pub struct SelectMenuUi;

#[derive(Debug, Component)]
pub struct CharacterInnerUi;

#[derive(Debug, Component)]
pub struct CharacterPic;

#[derive(Debug, Component)]
pub struct CharacterTitle;

#[derive(Debug, Component)]
pub struct CharacterDesc;

#[derive(Debug, Component, Clone)]
pub struct CharacterBlock {
    pub name: String,
    pub pic_sprite: &'static str,
    pub desc: String,
    pub selected_character_state: SelectedCharacterState,
}

pub fn get_ailsa_character() -> CharacterBlock {
    CharacterBlock {
        name: "Ailsa".to_string(),
        desc: "A bard who uses magic and a guitar for her instrument of choice, the most deadly of weapon combinations! The guitar is used to make sure they're dead afterwards - she doesn't actually need it for the magic part..."
            .to_string(),
        pic_sprite: "sprites/player/lisa-old.png",
        selected_character_state: SelectedCharacterState::Ailsa,
    }
}

pub fn get_lisa_character() -> CharacterBlock {
    CharacterBlock {
        name: "Lisa".to_string(),
        desc: "A friendly cleric who smites those not worthy of the grace of Paul Simon's greatness! A disarming smile and calm demeanor belie the terrifying badass within.".to_string(),
        pic_sprite: "sprites/player/lisa-old.png",
        selected_character_state: SelectedCharacterState::Ailsa,
    }
}

fn get_character_blocks() -> HashMap<SelectedCharacterState, CharacterBlock> {
    let mut characters = HashMap::new();

    characters.insert(SelectedCharacterState::Ailsa, get_ailsa_character());

    characters.insert(SelectedCharacterState::Lisa, get_lisa_character());

    characters
}

fn spawn_select_scene(
    mut commands: Commands,
    mut next_state: ResMut<NextState<CharacterSelectState>>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Loading character select plugin");

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
                flex_direction: FlexDirection::Row,
                // align_items: AlignItems::Center,
                // justify_content: JustifyContent::Center,
                ..default()
            },

            background_color: DARK_PURPLE.into(),
            ..default()
        },
        SelectMenuUi,
    );

    let characters = get_character_blocks();

    let ailsa_block = characters
        .get(&SelectedCharacterState::Ailsa)
        .unwrap()
        .clone();

    let lisa_block = characters
        .get(&SelectedCharacterState::Lisa)
        .unwrap()
        .clone();

    let ailsa_pic = get_character_pic(&ailsa_block, &mut texture_atlases, &assets);
    let lisa_pic = get_character_pic(&lisa_block, &mut texture_atlases, &assets);
    let ailsa_title = get_character_title(&ailsa_block, &assets);
    let lisa_title = get_character_title(&lisa_block, &assets);
    let ailsa_desc = get_character_desc(&ailsa_block, &assets);
    let lisa_desc = get_character_desc(&lisa_block, &assets);

    let ailsa_container = get_character_container();
    let lisa_container = get_character_container();

    let ailsa_inner_container = get_character_inner_container(ailsa_block);
    let lisa_inner_container = get_character_inner_container(lisa_block);

    commands.spawn(menu_parent).with_children(|commands| {
        commands.spawn(ailsa_container).with_children(|commands| {
            commands
                .spawn(ailsa_inner_container)
                .with_children(|commands| {
                    commands.spawn(ailsa_pic);
                    commands.spawn(ailsa_title);
                    commands.spawn(ailsa_desc);
                });
        });
        commands.spawn(lisa_container).with_children(|commands| {
            commands
                .spawn(lisa_inner_container)
                .with_children(|commands| {
                    commands.spawn(lisa_pic);
                    commands.spawn(lisa_title);
                    commands.spawn(lisa_desc);
                });
        });
    });

    next_state.set(CharacterSelectState::Started);
}

fn get_character_container() -> impl Bundle {
    (NodeBundle {
        style: Style {
            flex_basis: Val::Percent(50.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    },)
}

fn get_character_inner_container(character: CharacterBlock) -> impl Bundle {
    (
        ButtonBundle {
            style: Style {
                height: Val::Px(500.),
                width: Val::Px(500.),
                flex_direction: FlexDirection::Column,
                // align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // justify_items: JustifyItems::Center,
                padding: UiRect::all(Val::Px(30.)),
                border: UiRect::all(Val::Px(5.)),
                ..default()
            },
            background_color: PURPLE.into(),
            border_color: BLUE.into(),
            ..default()
        },
        character,
    )
}

fn get_character_pic(
    character: &CharacterBlock,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets: &Res<AssetServer>,
) -> impl Bundle {
    let block_width = 23.;
    let block_height = 36.;

    let texture_handle = assets.load(character.pic_sprite);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(block_width, block_height),
        9,
        9,
        Some(Vec2::new(1., 1.)),
        None, // Some(Vec2::new(1., 1.)),
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    (
        AtlasImageBundle {
            style: Style {
                width: Val::Px(51.),
                height: Val::Px(80.),

                // justify_content: JustifyContent::Center,
                // align_items: AlignItems::Center,
                // align_self: AlignSelf::Center,
                margin: UiRect::bottom(Val::Px(40.)),
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            texture_atlas_image: UiTextureAtlasImage {
                index: 0,

                ..default()
            },
            ..default()
        },
        CharacterPic {},
    )
}

fn get_character_title(character: &CharacterBlock, assets: &Res<AssetServer>) -> TextBundle {
    let font = assets.load("fonts/spectral/spectral_bold.ttf");

    let section = TextSection {
        value: character.name.clone(),
        style: TextStyle {
            font: font,
            font_size: 48.0,
            color: LIGHT_TEAL.into(),
        },
    };

    TextBundle {
        text: Text {
            sections: Vec::from([section]),
            ..default()
        },
        ..Default::default()
    }
}

fn get_character_desc(character: &CharacterBlock, assets: &Res<AssetServer>) -> TextBundle {
    let font = assets.load("fonts/spectral/spectral_medium.ttf");

    let section = TextSection {
        value: character.desc.clone(),
        style: TextStyle {
            font: font,
            font_size: 24.0,
            color: LIGHT_TEAL.into(),
        },
    };

    TextBundle {
        text: Text {
            sections: Vec::from([section]),
            ..default()
        },
        ..Default::default()
    }
}

fn despawn_select_scene(
    mut commands: Commands,
    ui_query: Query<Entity, With<SelectMenuUi>>,
    // music_query: Query<Entity, With<MyMusic>>,
    // sb_query: Query<Entity, With<StartButtonUI>>,
    // eb_query: Query<Entity, With<ExitButtonUI>>,
) {
    println!("Despawning character select");

    for ui in &ui_query {
        commands.entity(ui).despawn_recursive();
    }
}

fn character_select_system(
    mut block_click_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &CharacterBlock,
        ),
        With<Node>,
    >,
    mut commands: Commands,
    music_query: Query<Entity, With<MyMusic>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_character_state: ResMut<NextState<SelectedCharacterState>>,
    // mut next_fade_state: ResMut<NextState<FadeState>>,
) {
    for (interaction, mut color, mut border_color, character) in &mut block_click_query {
        // println!(
        //     "Checking interaction for something {:?}",
        //     character.selected_character_state
        // );

        match *interaction {
            Interaction::Pressed => {
                println!(
                    "Pressed character block: {:?}",
                    character.selected_character_state
                );

                // *color = PRESSED_BUTTON_COLOR.into();

                next_character_state.set(character.selected_character_state.clone());
                // next_fade_state.set(FadeState::FadeToGame);
                next_game_state.set(GameState::Corridor);

                for music in &music_query {
                    commands.entity(music).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                println!(
                    "Hovered character block: {:?}",
                    character.selected_character_state
                );

                *color = BLUE.into();
                *border_color = BORDER_COLOR.into();
            }
            Interaction::None => {
                // println!(
                //     "Back to default character block: {:?}",
                //     character.selected_character_state
                // );

                *color = PURPLE.into();
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
