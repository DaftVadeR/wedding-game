use crate::main_menu::MyMusic;
use crate::util_fade::FadeState;
use crate::GameState;
use bevy::app::{AppExit, Plugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

pub struct CharacterSelectPlugin;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum CharacterSelectState {
    #[default]
    Unloaded,
    Init,
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
pub struct SelectMenuUI {}

#[derive(Debug, Component)]
pub struct CharacterBlock {
    name: String,
    pic_sprite: Handle<TextureAtlas>,
    desc: String,
    selected_character_state: SelectedCharacterState,
}

fn spawn_select_scene(
    mut commands: Commands,
    mut next_state: ResMut<NextState<CharacterSelectState>>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Loading character select plugin");

    // let font = assets.load("fonts/spectral/spectral_bold.ttf");

    let ailsa = CharacterBlock {
        name: "Ailsa".to_string(),
        desc: "A bard who uses magic and a guitar, or something, to catch her foes unprepared!"
            .to_string(),
        pic_sprite: assets.load("sprites/player/ailsa.png"),
        selected_character_state: SelectedCharacterState::Ailsa,
    };

    let lisa = CharacterBlock {
        name: "Lisa".to_string(),
        desc: "A cleric who smites those not worthy of the grace of... Elvis!".to_string(),
        pic_sprite: assets.load("sprites/player/lisa.png"),
        selected_character_state: SelectedCharacterState::Ailsa,
    };

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
        SelectMenuUI {},
    );

    let character_box = get_character(SelectedCharacterState::Ailsa);

    next_state.set(CharacterSelectState::Started);
}

fn get_character(name: String) -> impl Bundle {
    (
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
        SelectMenuUI {},
    )
}

fn despawn_select_scene(
    mut exit: EventWriter<AppExit>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("Despawning character select");
}

const NORMAL_BUTTON: Color = Color::CRIMSON;
const HOVERED_BUTTON: Color = Color::PURPLE;
const PRESSED_BUTTON: Color = Color::RED;

fn character_select_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CharacterBlock),
        (With<Node>, With<CharacterBlock>),
    >,
    mut commands: Commands,
    music_query: Query<Entity, With<MyMusic>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_character_state: ResMut<NextState<SelectedCharacterState>>,
    mut next_fade_state: ResMut<NextState<FadeState>>,
) {
    for (interaction, mut color, character) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_character_state.set(character.selected_character_state.clone());
                next_fade_state.set(FadeState::FadeToGame);
                next_game_state.set(GameState::Corridor);

                for music in &music_query {
                    commands.entity(music).despawn_recursive();
                }
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
