use bevy::prelude::*;

use crate::character_select::{
    get_ailsa_character, get_lisa_character, CharacterBlock, SelectedCharacterState,
};
use crate::corridor::level::{MAP_HEIGHT, MAP_VERTICAL_OFFSET};
use crate::corridor::sprite::{
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

const PLAYER_SPEED_DEFAULT: f32 = 100.;
const PLAYER_WIDTH: f32 = 23.;
const PLAYER_HEIGHT: f32 = 23.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct GameplayOnly;

#[derive(Debug, Component)]
pub struct CanLevel {
    pub experience: u64,
    pub level: u32,
}

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
                    animate_sprite,
                    player_movement,
                    update_camera_from_player_position,
                )
                    .run_if(in_state(CorridorPlayerState::Started)),
            );
        //
        // app.add_systems(Startup, /*OnEnter(GameState::StartingLoop),*/ spawn_player);
        /*.add_systems(
            (
                player_movement,
                player_exp_start_pickup,
                player_gain_exp,
                player_level_up,
                player_game_over,
            )
            .in_set(OnUpdate(GameState::Gameplay)),
        );*/
        // // simple "facilitator" schedules benefit from simpler single threaded scheduling
        // let mut main_schedule = Schedule::new(Main);
        // main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        // let mut fixed_update_loop_schedule = Schedule::new(RunFixedUpdateLoop);
        // fixed_update_loop_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

        // app.add_schedule(main_schedule)
        //     .add_schedule(fixed_updat    e_loop_schedule)
        //     .init_resource::<MainScheduleOrder>()
        //     .add_systems(Main, Main::run_main);
    }
}

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum CorridorLevelState {
    #[default]
    Unloaded,
    Init,
    Started,
}

pub fn unload(
    mut query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<CorridorPlayerState>>,
    mut state: ResMut<State<CorridorPlayerState>>,
    mut commands: Commands,
) {
    for (entity) in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_camera_from_player_position(
    query: Query<(&Transform), (With<Player>)>,
    mut camera_query: Query<(&mut Transform), (With<Camera>, Without<Player>)>,
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

    let (mut movable, mut sprite, mut transform, animateable) = player.single_mut();

    let normal_translation = time.delta_seconds() * movable.speed;

    let diagonal_translation = (normal_translation * normal_translation * 2.).sqrt() / 2.;

    let mut key_pressed = false;

    // Top and bottom with checks for diagonal.
    if input.pressed(KeyCode::W) {
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
        key_pressed = true;
    } else if input.pressed(KeyCode::S) {
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
        key_pressed = true;
    } else if input.pressed(KeyCode::A) {
        transform.translation.x -= normal_translation;
        sprite.flip_x = true;
        movable.direction = Direction::Left;
        key_pressed = true;
    } else if input.pressed(KeyCode::D) {
        transform.translation.x += normal_translation;
        sprite.flip_x = false;
        movable.direction = Direction::Right;
        key_pressed = true;
    }

    transform.translation.x = transform.translation.x.clamp(
        -1. * (MAP_WIDTH / 2.) + PLAYER_WIDTH / 2.,
        MAP_WIDTH / 2. - PLAYER_WIDTH / 2.,
    );

    transform.translation.y = transform.translation.y.clamp(
        -1. * (MAP_VERTICAL_OFFSET) + PLAYER_HEIGHT / 2.,
        (MAP_HEIGHT - MAP_VERTICAL_OFFSET - PLAYER_HEIGHT / 2.),
    );

    // If it changed
    if movable.is_moving != key_pressed {
        movable.is_moving = key_pressed;

        sprite.index = (if movable.is_moving {
            get_indices_for_movable(&movable, &animateable)
        } else {
            &animateable.idle_anim_indices
        })
        .first;
    }
}

pub fn get_indices_for_movable<'a>(
    movable: &'a Movable,
    animateable: &'a PlayerSpriteSheetAnimatable,
) -> &'a AnimationIndices {
    if !movable.is_moving {
        &animateable.idle_anim_indices
    } else {
        if movable.direction == Direction::Up {
            &animateable.moving_up_anim_indices
        } else if movable.direction == Direction::Down {
            &animateable.moving_down_anim_indices
        } else {
            &animateable.moving_horizontal_anim_indices
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &PlayerSpriteSheetAnimatable,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Movable,
    )>,
    mut next_state: ResMut<NextState<CorridorPlayerState>>,
    mut state: ResMut<State<CorridorPlayerState>>,
) {
    if state.get() != &CorridorPlayerState::Started {
        return;
    }

    for (animateable, mut timer, mut sprite, movable) in &mut query {
        let indices = get_indices_for_movable(movable, animateable);
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index >= indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

// fn get_knight_sprite() {

//     let texture_handle = asset_server.load("sprites/player/knight_all_anims_spritesheet.png");

//     // let builder = TextureAtlasBuilder::default().initial_size(Vec2 { x: 96., y: 32. });
//     // builder.add_texture(, texture)

//     let texture_atlas = TextureAtlas::from_grid(
//         texture_handle,
//         Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
//         6,
//         2,
//         None,
//         None,
//     );

//     // texture_atlas.
//     // let texture_atlas_run =
//     //     TextureAtlas::from_grid(texture_handle_run, Vec2::new(16.0, 16.0), 6, 1, None, None);

//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//     // let texture_atlas_run_handle = texture_atlases.add(texture_atlas_run);

//     // Use only the subset of sprites in the sheet that make up the run animation
//     let idle_animation_indices = AnimationIndices { first: 0, last: 5 };
//     let run_animation_indices = AnimationIndices { first: 6, last: 11 };
// }

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<CorridorPlayerState>>,
    state: Res<State<SelectedCharacterState>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning corridor player plugin");
    // let texture_handle = asset_server.load("player/knight_idle_spritesheet.png");
    // let texture_handle_run = asset_server.load("player/knight_run_spritesheet.png");
    // println!("Loading player spritesheet");

    let character: CharacterBlock;

    if state.get() == &SelectedCharacterState::Ailsa {
        character = get_ailsa_character();
    } else {
        character = get_lisa_character();
    }

    let texture_handle = asset_server.load(character.pic_sprite);

    // let builder = TextureAtlasBuilder::default().initial_size(Vec2 { x: 96., y: 32. });
    // builder.add_texture(, texture)

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        4,
        4,
        None,
        None,
    );

    // texture_atlas.
    // let texture_atlas_run =
    //     TextureAtlas::from_grid(texture_handle_run, Vec2::new(16.0, 16.0), 6, 1, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // let texture_atlas_run_handle = texture_atlases.add(texture_atlas_run);

    // Use only the subset of sprites in the sheet that make up the run animation
    let idle_animation_indices = AnimationIndices { first: 0, last: 1 };
    let run_down_animation_indices = AnimationIndices { first: 4, last: 7 };
    let run_horizontal_animation_indices = AnimationIndices { first: 8, last: 11 };
    let run_up_animation_indices = AnimationIndices {
        first: 12,
        last: 15,
    };

    // Spawn Level
    // let map_bottom_y_pos = -1. * (MAP_HEIGHT / 2.) + MAP_VERTICAL_OFFSET;

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(idle_animation_indices.first),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        Player,
        PlayerSpriteSheetAnimatable {
            idle_anim_indices: idle_animation_indices,
            moving_horizontal_anim_indices: run_horizontal_animation_indices,
            moving_up_anim_indices: run_up_animation_indices,
            moving_down_anim_indices: run_down_animation_indices,
        },
        Movable {
            speed: PLAYER_SPEED_DEFAULT,
            direction: Direction::Right,
            is_moving: false,
        },
    ));

    next_state.set(CorridorPlayerState::Started);
}
