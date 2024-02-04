use crate::character_select::{
    get_character_sprite, SelectedCharacterState, PLAYER_HEIGHT, PLAYER_WIDTH,
};
use crate::corridor::player::{get_character_block, get_indices_for_movable};
use crate::sprite::{AnimationTimer, Direction, Health, Movable, PlayerSpriteSheetAnimatable};
use crate::GameState;

use bevy::prelude::*;

use super::level::{MAP_MOVABLE_HEIGHT, MAP_MOVABLE_WIDTH};
use super::GamePlayState;

const PLAYER_SPEED_DEFAULT: f32 = 100.;

pub struct PlayerPlugin;

#[derive(Debug, Component)]
pub struct CanLevel {
    pub experience: u64,
    pub level: u32,
}

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePlayState::Init), setup)
            .add_systems(OnExit(GameState::Gameplay), unload)
            .add_systems(
                Update,
                (player_movement, update_camera_from_player_position)
                    .run_if(in_state(GamePlayState::Started)),
            );
    }
}

pub fn update_camera_from_player_position(
    query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
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
    state: Res<State<GamePlayState>>,
) {
    if state.get() != &GamePlayState::Started {
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
        -1. * (MAP_MOVABLE_WIDTH / 2.) + PLAYER_WIDTH / 2.,
        MAP_MOVABLE_WIDTH / 2. - PLAYER_WIDTH / 2.,
    );

    transform.translation.y = transform.translation.y.clamp(
        -1. * (MAP_MOVABLE_HEIGHT / 2.) + PLAYER_HEIGHT / 2.,
        MAP_MOVABLE_HEIGHT - PLAYER_HEIGHT / 2.,
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
    }

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
    mut next_state: ResMut<NextState<GamePlayState>>,
    mut state: Res<State<SelectedCharacterState>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning game player plugin");

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
        Health(100.),
        CanLevel {
            experience: 0,
            level: 1,
        },
        Movable {
            speed: PLAYER_SPEED_DEFAULT,
            direction: Direction::Down,
            is_moving: false,
            current_animation_indices: idle_anims,
        },
    ));

    next_state.set(GamePlayState::Started);
}

pub fn unload(mut query: Query<Entity, With<Player>>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
