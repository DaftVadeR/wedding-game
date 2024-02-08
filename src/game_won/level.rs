use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::sprite::AnimationIndices;

use crate::game_won::player::Player;
use crate::util_fade::FadeState;
use crate::GameState;
pub struct LevelPlugin;
use rand::Rng;

use super::npc::Npc;
use super::GameWonState;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum GameWonLevelState {
    #[default]
    Unloaded,
    Init,
    Started,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        println!("Load lkevel plugin game won");
        app.add_state::<GameWonLevelState>()
            .add_systems(OnEnter(GameWonLevelState::Init), (setup, draw_path))
            .add_systems(Update, update.run_if(in_state(GameWonLevelState::Started)))
            .add_systems(OnExit(GameState::GameWon), unload);
    }
}

pub const MAP_WIDTH: f32 = 1600.;
pub const MAP_HEIGHT: f32 = 1600.;

pub const CLAMP_WIDTH: f32 = 64.;
pub const CLAMP_HEIGHT: f32 = 832.;
pub const CLAMP_OFFSET: f32 = 128.;
pub const HORIZONTAL_OFFSET_FOR_DOOR: f32 = 51.;

// Used to make sure the level bottom is just below player. So you know to go up.
pub const MAP_VERTICAL_OFFSET: f32 = 320.;

#[derive(Debug, Component)]
pub struct Wall {}

#[derive(Debug, Component)]
pub struct Floor {}

fn unload(
    // mut walls: Query<Entity, With<Wall>>,
    mut floor: Query<Entity, With<Floor>>,
    mut house: Query<Entity, With<House>>,
    mut commands: Commands,
    // mut player: Query<Entity, With<Player>>,
) {
    // for wall in &mut walls.iter_mut() {
    //     commands.entity(wall).despawn_recursive();
    // }

    for floor in &mut floor.iter_mut() {
        commands.entity(floor).despawn_recursive();
    }

    for house in &mut house.iter_mut() {
        commands.entity(house).despawn_recursive();
    }
}

fn update(
    mut house_query: Query<
        (&mut TextureAtlasSprite, &Transform, &mut House),
        (Without<Player>, With<House>),
    >,
    mut player_query: Query<(&mut TextureAtlasSprite, &Transform), (Without<House>, With<Player>)>,
    mut npc_query: Query<&mut TextureAtlasSprite, (Without<House>, With<Npc>, Without<Player>)>,
    time: Res<Time>,
    mut next_won_state: ResMut<NextState<GameWonState>>,
    mut next_fade_state: ResMut<NextState<FadeState>>,
    mut next_level_state: ResMut<NextState<GameWonLevelState>>,
) {
    let (mut texture_atlas_sprite, house_transform, mut house) = house_query.single_mut();
    let (mut player_sprite, player_transform) = player_query.single_mut();
    let mut npc_sprite = npc_query.single_mut();
    // let (mut sprite) = player_update_query.single_mut();

    let detection_area_x = 40.;
    let detection_area_y = 24.;

    let is_near_house_x = player_transform.translation.x > -1. * detection_area_x
        && player_transform.translation.x < detection_area_x + HORIZONTAL_OFFSET_FOR_DOOR;

    let is_near_house_y =
        player_transform.translation.y > house_transform.translation.y - detection_area_y;

    // println!(
    //     "player position: {} {}",
    //     player_transform.translation.x, player_transform.translation.y,
    // );
    // println!(
    //     "door position: {} {}",
    //     house_transform.translation.x, house_transform.translation.y,
    // );
    // println!("is_near_house_x: {}", is_near_house_x,);
    // println!("is_near_house_y: {}", is_near_house_y,);
    // Check if player is in the right spot to open the door.
    if !house.is_open && is_near_house_x && is_near_house_y {
        println!("House opened");
        house.is_open = true;
        texture_atlas_sprite.index = house.animation_indices.last;
        // next_fade_state.set(FadeState::FadeToBlack);
    }

    if house.is_open {
        println!("House is open");
        house.open_timer.tick(time.delta());
        player_sprite.color.set_a(house.open_timer.percent_left());
        npc_sprite.color.set_a(house.open_timer.percent_left());

        if house.open_timer.finished() {
            println!("Change to Congrats From Game Won");
            next_won_state.set(GameWonState::Congrats);

            // next_won_state.set(GameWonState::Congrats);
            // next_level_state.set(GameWonLevelState::Unloaded);
            // next_player_state.set(GameWonPlayerState::Unloaded);
        }
    }

    // Check for collision with player
}

fn draw_path(
    mut commands: Commands,
    mut next_level_state: ResMut<NextState<GameWonLevelState>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Game Won level setup");

    // Sprite stuff
    let block_width = 32.;
    let block_height = 32.;
    let block_cols = 2;
    let block_rows = 4;
    let total_atlas_blocks = block_cols * block_rows;

    // Calculate number of loops needed to fill the desired MAP or REGION size.
    let total_tiles_wide = (CLAMP_WIDTH as i32) / (block_width as i32); // In this case - 6
    let total_tiles_high = (CLAMP_HEIGHT as i32) / (block_height as i32); // In this case - 80
    let total_tiles = total_tiles_high * total_tiles_wide;
    let final_x = CLAMP_WIDTH / 2.;
    let final_y = CLAMP_HEIGHT - CLAMP_OFFSET;

    // println!("TOTAL TILES {}", total_tiles);

    let mut rng = rand::thread_rng();

    // Start at -1280, -1280 for 2560 size map.
    let starting_point_x = -1. * (CLAMP_WIDTH / 2.);
    let starting_point_y = -1. * (CLAMP_OFFSET);
    // println!("STARTING POINT Y {}", starting_point_y);
    let mut rolling_x = starting_point_x;
    let mut rolling_y = starting_point_y;

    //commands.spawn(Camera2dBundle::default());
    for index in 0..total_tiles {
        let mut needs_flip = false;
        let mut texture_index = 6;

        if index < 2 {
            // If bottom left or right square.
            if index == 0 {
                // Left
                texture_index = 4;
            } else if index == 1 {
                // Right
                texture_index = 5;
            }
        } else if index > total_tiles - 3 {
            // if top left or right
            if index == total_tiles - 2 {
                // Left
                texture_index = 2;
            } else if index == total_tiles - 1 {
                // Right
                texture_index = 3;
            }
        } else {
            texture_index = 7;

            if index % 2 != 0 {
                needs_flip = true;
            }
        }
        //asd
        // println!("INDEX {} {} {}", index, rolling_x, rolling_y);

        let texture_handle = asset_server.load("sprites/level/dirt4_pipo_new.png");

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(block_width, block_height),
            block_cols,
            block_rows,
            None,
            None,
        );

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // let random_index: usize = rng.gen_range(0..total_atlas_blocks);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: texture_index,
                    anchor: Anchor::BottomLeft,
                    flip_x: needs_flip,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(1., 1., 1.),
                    rotation: Quat::IDENTITY,
                    translation: Vec3::new(rolling_x, rolling_y, 1.),
                    ..default()
                },

                // global_transform: Transform::
                ..default()
            },
            Floor {},
        ));

        rolling_x += block_width;

        if rolling_x >= final_x {
            rolling_x = starting_point_x;
            rolling_y += block_height;
        }

        // if rolling_y >= final_y {
        //     break;
        //     //panic!("Should not get here - loop should finish before this.");
        // }
    }
}

fn setup(
    mut commands: Commands,
    mut next_level_state: ResMut<NextState<GameWonLevelState>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Game won level setup");

    // Sprite stuff
    let block_width = 32.;
    let block_height = 32.;
    let block_cols = 8;
    let block_rows = 4;
    let total_atlas_blocks = block_cols * block_rows;

    // Calculate number of loops needed to fill the desired MAP or REGION size.
    let total_tiles_wide = (MAP_WIDTH as i32) / (block_width as i32); // In this case - 6
    let total_tiles_high = (MAP_HEIGHT as i32) / (block_height as i32); // In this case - 80
    let total_tiles = total_tiles_high * total_tiles_wide;
    let final_x = MAP_WIDTH / 2.;
    let final_y = MAP_HEIGHT - MAP_VERTICAL_OFFSET;

    println!("TOTAL TILES {}", total_tiles);

    let mut rng = rand::thread_rng();

    // Start at -1280, -1280 for 2560 size map.
    let starting_point_x = -1. * (MAP_WIDTH / 2.);
    let starting_point_y = -1. * (MAP_VERTICAL_OFFSET);
    // println!("STARTING POINT Y {}", starting_point_y);
    let mut rolling_x = starting_point_x;
    let mut rolling_y = starting_point_y;

    //commands.spawn(Camera2dBundle::default());
    for index in 0..total_tiles {
        // println!("INDEX {} {} {}", index, rolling_x, rolling_y);

        let texture_handle = asset_server.load("sprites/level/tx_tileset_grass.png");

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(block_width, block_height),
            block_cols,
            block_rows,
            None,
            None,
        );

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let random_index: usize = rng.gen_range(0..total_atlas_blocks);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: random_index,
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform {
                    // scale: Vec3::new(1., 1., 1.),
                    rotation: Quat::IDENTITY,
                    translation: Vec3::new(rolling_x, rolling_y, -0.4),
                    ..default()
                },
                // global_transform: Transform::
                ..default()
            },
            Floor {},
        ));

        rolling_x += block_width;

        if rolling_x >= final_x {
            rolling_x = starting_point_x;
            rolling_y += block_height;
        }

        // if rolling_y >= final_y {
        //     break;
        //     //panic!("Should not get here - loop should finish before this.");
        // }
    }

    draw_house(
        -1. * HORIZONTAL_OFFSET_FOR_DOOR, // Account for path position and shift house to match door position
        CLAMP_HEIGHT - CLAMP_OFFSET,
        &mut commands,
        &asset_server,
        &mut texture_atlases,
    );

    next_level_state.set(GameWonLevelState::Started);
}

#[derive(Debug, Component)]
pub struct House {
    is_open: bool,
    animation_indices: AnimationIndices,
    open_timer: Timer,
}

impl House {
    pub fn new() -> Self {
        Self {
            is_open: false,
            open_timer: Timer::from_seconds(2., TimerMode::Once),
            animation_indices: AnimationIndices { first: 0, last: 1 },
        }
    }
}

fn draw_house(
    position_x: f32,
    position_y: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let block_width = 168.;
    let block_height = 279.;

    let texture_handle = asset_server.load("sprites/level/house_states.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(block_width, block_height),
        2,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let house = House::new();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: house.animation_indices.first,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform {
                rotation: Quat::IDENTITY,
                translation: Vec3::new(position_x - block_width / 2., position_y, -0.1),
                ..default()
            },
            ..default()
        },
        house,
    ));
}

// /// This system ticks the `Timer` on the entity with the `PrintOnCompletionTimer`
// /// component using bevy's `Time` resource to get the delta between each update.
// fn print_when_completed(time: Res<Time>, mut query: Query<&mut PrintOnCompletionTimer>) {
//     for mut timer in &mut query {
//         if timer.tick(time.delta()).just_finished() {
//             info!("Entity timer just finished");
//         }
//     }
// }

// /// This system controls ticking the timer within the countdown resource and
// /// handling its state.
// fn countdown(time: Res<Time>, mut door: Query<&Door>) {
//     countdown.main_timer.tick(time.delta());

//     // The API encourages this kind of timer state checking (if you're only checking for one value)
//     // Additionally, `finished()` would accomplish the same thing as `just_finished` due to the
//     // timer being repeating, however this makes more sense visually.
//     if countdown.percent_trigger.tick(time.delta()).just_finished() {
//         if !countdown.main_timer.finished() {
//             // Print the percent complete the main timer is.
//             info!(
//                 "Timer is {:0.0}% complete!",
//                 countdown.main_timer.fraction() * 100.0
//             );
//         } else {
//             // The timer has finished so we pause the percent output timer
//             countdown.percent_trigger.pause();
//             info!("Paused percent trigger timer");
//         }
//     }
// }
