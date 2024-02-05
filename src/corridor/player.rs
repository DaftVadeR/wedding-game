use bevy::prelude::*;

use crate::character_select::{
    get_ailsa_character, get_character_sprite, get_lisa_character, CharacterBlock,
    SelectedCharacterState, PLAYER_HEIGHT, PLAYER_WIDTH,
};
use crate::corridor::level::{MAP_HEIGHT, MAP_VERTICAL_OFFSET};

use crate::sprite::{
    AnimationIndices, AnimationTimer, Direction, Movable, PlayerSpriteSheetAnimatable,
};

use crate::GameState;

use super::level::MAP_WIDTH;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum CorridorPlayerState {
    #[default]
    Unloaded,
    Init,
    Started,
}

pub const PLAYER_SPEED_DEFAULT: f32 = 100.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CorridorPlayerState>()
            .add_systems(OnEnter(CorridorPlayerState::Init), setup)
            .add_systems(OnExit(GameState::Corridor), unload)
            .add_systems(
                Update,
                (
                    // animate_sprite,
                    player_movement,
                    update_camera_from_player_position,
                )
                    .run_if(in_state(CorridorPlayerState::Started)),
            );
    }
}

pub fn unload(
    mut query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<CorridorPlayerState>>,
    mut state: ResMut<State<CorridorPlayerState>>,
    mut commands: Commands,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_camera_from_player_position(
    query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut next_state: ResMut<NextState<CorridorPlayerState>>,
    mut state: ResMut<State<CorridorPlayerState>>,
) {
    if state.get() != &CorridorPlayerState::Started {
        return;
    }

    let player_transform = query.single();

    let mut camera_transform = camera_query.single_mut();

    //
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

pub fn player_movement(
    mut player: Query<
        (
            &mut Movable,
            &mut TextureAtlasSprite,
            &mut Transform,
            &mut AnimationTimer,
            &PlayerSpriteSheetAnimatable,
        ),
        With<Player>,
    >,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<CorridorPlayerState>>,
    mut state: ResMut<State<CorridorPlayerState>>,
) {
    if state.get() != &CorridorPlayerState::Started {
        return;
    }

    let (mut movable, mut sprite, mut transform, mut timer, animateable) = player.single_mut();

    let normal_translation = time.delta_seconds() * movable.speed;

    let diagonal_translation = (normal_translation * normal_translation * 2.).sqrt() / 2.;

    let mut key_pressed = false;

    let old_direction = movable.direction.clone();
    let old_is_moving = movable.is_moving.clone();

    // Top and bottom with checks for diagonal.
    if input.pressed(KeyCode::W) {
        key_pressed = true;

        if input.pressed(KeyCode::D) {
            movable.direction = Direction::UpRight;
            sprite.flip_x = false;
            transform.translation.y += diagonal_translation;
            transform.translation.x += diagonal_translation;
        } else if input.pressed(KeyCode::A) {
            movable.direction = Direction::UpLeft;
            sprite.flip_x = true;
            transform.translation.y += diagonal_translation;
            transform.translation.x -= diagonal_translation;
        } else {
            movable.direction = Direction::Up;
            transform.translation.y += normal_translation;
        }
    } else if input.pressed(KeyCode::S) {
        key_pressed = true;
        if input.pressed(KeyCode::D) {
            sprite.flip_x = false;
            movable.direction = Direction::DownRight;
            transform.translation.y -= diagonal_translation;
            transform.translation.x += diagonal_translation;
        } else if input.pressed(KeyCode::A) {
            sprite.flip_x = true;
            movable.direction = Direction::DownLeft;
            transform.translation.y -= diagonal_translation;
            transform.translation.x -= diagonal_translation;
        } else {
            movable.direction = Direction::Down;
            transform.translation.y -= normal_translation;
        }
    } else if input.pressed(KeyCode::A) {
        key_pressed = true;
        transform.translation.x -= normal_translation;
        sprite.flip_x = true;
        movable.direction = Direction::Left;
    } else if input.pressed(KeyCode::D) {
        key_pressed = true;
        transform.translation.x += normal_translation;
        sprite.flip_x = false;
        movable.direction = Direction::Right;
    }

    transform.translation.x = transform.translation.x.clamp(
        -1. * (MAP_WIDTH / 2.) + PLAYER_WIDTH / 2.,
        MAP_WIDTH / 2. - PLAYER_WIDTH / 2.,
    );

    transform.translation.y = transform.translation.y.clamp(
        -1. * (MAP_VERTICAL_OFFSET) + PLAYER_HEIGHT / 2.,
        MAP_HEIGHT - MAP_VERTICAL_OFFSET - PLAYER_HEIGHT / 2.,
    );

    movable.is_moving = key_pressed;

    // IMPORTANT - need to compare with prior frame state to make sure not resetting anim unnecessary, but also
    // makes sure to reset on EVERY movement or direction change.
    if movable.direction != old_direction || movable.is_moving != old_is_moving {
        let chosen = get_indices_for_movable(&movable, &animateable, &sprite);

        if chosen.is_some() {
            movable.current_animation_indices = chosen.unwrap();
        }

        sprite.index = movable.current_animation_indices.first;
    } else {
        timer.tick(time.delta());

        if timer.just_finished() {
            sprite.index = if sprite.index >= movable.current_animation_indices.last {
                movable.current_animation_indices.first
            } else {
                sprite.index + 1
            }
        }
    }
}

pub fn get_indices_for_movable(
    movable: &Movable,
    animateable: &PlayerSpriteSheetAnimatable,
    sprite: &TextureAtlasSprite,
) -> Option<AnimationIndices> {
    let diagonal_up_options = vec![Direction::UpLeft, Direction::UpRight];
    let diagonal_down_options = vec![Direction::DownLeft, Direction::DownRight];
    let horizontal = vec![Direction::Left, Direction::Right];

    let chosen: AnimationIndices;

    if !movable.is_moving {
        // THE PROBLEM>?
        chosen = animateable.idle_anim_indices.clone();
    } else {
        if diagonal_up_options.contains(&movable.direction) {
            chosen = animateable.moving_up_horiz_anim_indices.clone();
        } else if diagonal_down_options.contains(&movable.direction) {
            chosen = animateable.moving_down_horiz_anim_indices.clone();
        } else if movable.direction == Direction::Up {
            chosen = animateable.moving_up_anim_indices.clone();
        } else if movable.direction == Direction::Down {
            chosen = animateable.moving_down_anim_indices.clone();
        } else if horizontal.contains(&movable.direction) {
            chosen = animateable.moving_horizontal_anim_indices.clone();
        } else {
            return None;
        }
    }

    return Some(chosen);
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Movable, &mut AnimationTimer, &mut TextureAtlasSprite)>,
    mut next_state: ResMut<NextState<CorridorPlayerState>>,
    mut state: ResMut<State<CorridorPlayerState>>,
) {
    let (movable, mut timer, mut sprite) = query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
        sprite.index = if sprite.index >= movable.current_animation_indices.last {
            movable.current_animation_indices.first
        } else {
            sprite.index + 1
        }
    }
}

pub fn get_character_block(
    state: &SelectedCharacterState,
) -> (CharacterBlock, PlayerSpriteSheetAnimatable) {
    let character: CharacterBlock;

    if state == &SelectedCharacterState::Ailsa {
        character = get_ailsa_character();
    } else {
        character = get_lisa_character();
    }

    // Use only the subset of sprites in the sheet that make up the run animation
    let idle_animation_indices = AnimationIndices { first: 0, last: 1 };
    let run_down_animation_indices = AnimationIndices { first: 9, last: 17 };
    let run_horizontal_animation_indices = AnimationIndices {
        first: 27,
        last: 35,
    };
    let run_up_animation_indices = AnimationIndices {
        first: 45,
        last: 53,
    };

    let run_up_horiz_animation_indices = AnimationIndices {
        first: 36,
        last: 44,
    };
    let run_down_horiz_animation_indices = AnimationIndices {
        first: 18,
        last: 26,
    };

    let animatable: PlayerSpriteSheetAnimatable = PlayerSpriteSheetAnimatable {
        idle_anim_indices: idle_animation_indices,
        moving_horizontal_anim_indices: run_horizontal_animation_indices,
        moving_up_anim_indices: run_up_animation_indices,
        moving_down_anim_indices: run_down_animation_indices,
        moving_up_horiz_anim_indices: run_up_horiz_animation_indices,
        moving_down_horiz_anim_indices: run_down_horiz_animation_indices,
    };

    (character, animatable)
}

fn setup(
    mut commands: Commands,
    mut assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<CorridorPlayerState>>,
    state: Res<State<SelectedCharacterState>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning corridor player plugin");

    let (character, animatable) = get_character_block(state.get());
    let texture_atlas_handle = get_character_sprite(&character, &mut texture_atlases, &mut assets);

    let idle_anims = animatable.idle_anim_indices.clone();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(idle_anims.first),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        Player,
        animatable,
        Movable {
            speed: PLAYER_SPEED_DEFAULT,
            direction: Direction::Down,
            is_moving: false,
            current_animation_indices: idle_anims,
        },
    ));

    next_state.set(CorridorPlayerState::Started);
}
