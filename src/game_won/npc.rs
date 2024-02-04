use bevy::prelude::*;

use crate::character_select::{
    get_ailsa_character, get_character_sprite, get_lisa_character, CharacterBlock,
    SelectedCharacterState,
};

use crate::corridor::player::get_character_block;
use crate::game_won::level::HORIZONTAL_OFFSET_FOR_DOOR;
use crate::sprite::{
    AnimationIndices, AnimationTimer, Direction, Movable, PlayerSpriteSheetAnimatable,
};

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum GameWonNpcState {
    #[default]
    Unloaded,
    Init,
    Started,
}

use crate::GameState;

use super::level::House;
use super::player::GameWonLevelState;

pub struct NpcPlugin;

#[derive(Debug, Component)]
pub struct CanLevel {
    pub experience: u64,
    pub level: u32,
}

#[derive(Component)]
pub struct Npc;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameWonNpcState>()
            .add_systems(OnEnter(GameWonLevelState::Started), setup)
            .add_systems(OnExit(GameState::GameWon), unload)
            .add_systems(
                Update,
                (update_character).run_if(in_state(GameWonNpcState::Started)),
            );
    }
}

pub fn unload(
    mut query: Query<Entity, With<Npc>>,
    mut next_state: ResMut<NextState<GameWonNpcState>>,
    mut commands: Commands,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    // next_state.set(GameWonNpcState::Unloaded);
}

pub fn update_character(
    mut player: Query<(&Movable, &mut TextureAtlasSprite, &mut AnimationTimer), With<Npc>>,
    time: Res<Time>,
    state: ResMut<State<GameWonNpcState>>,
) {
    if state.get() != &GameWonNpcState::Started {
        return;
    }

    let (movable, mut sprite, mut timer) = player.single_mut();

    timer.tick(time.delta());

    if timer.just_finished() {
        sprite.index = if sprite.index >= movable.current_animation_indices.last {
            movable.current_animation_indices.first
        } else {
            sprite.index + 1
        }
    }
}

fn setup(
    mut commands: Commands,
    mut assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<GameWonNpcState>>,
    state: Res<State<SelectedCharacterState>>,
    mut house_query: Query<&Transform, With<House>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning gamewon player plugin");

    let house_transform = house_query.single_mut();
    let npc_state: SelectedCharacterState;

    if state.get() == &SelectedCharacterState::Lisa {
        npc_state = SelectedCharacterState::Ailsa;
    } else {
        npc_state = SelectedCharacterState::Lisa;
    }

    let (character, animatable) = get_character_block(&npc_state);
    let texture_atlas_handle = get_character_sprite(&character, &mut texture_atlases, &mut assets);

    let idle_anims = animatable.idle_anim_indices.clone();

    // Spawn character 30 px left of door pos.
    let spawn_x = -1. * HORIZONTAL_OFFSET_FOR_DOOR + 15.;

    let spawn_y = house_transform.translation.y + 10.;

    println!("NPC POSITION: {} {}", spawn_x, spawn_y);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(idle_anims.first),
            transform: Transform::from_xyz(spawn_x, spawn_y, 1.),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        Npc,
        animatable,
        Movable {
            speed: 0.,
            direction: Direction::Down,
            is_moving: false,
            current_animation_indices: idle_anims,
        },
    ));

    next_state.set(GameWonNpcState::Started);
}
