use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::sprite::AnimationIndices;

use crate::game_won::player::{GameWonPlayerState, Player};
use crate::util_fade::FadeState;
use crate::GameState;

pub struct LevelPlugin;
use rand::Rng;

use super::player::GameWonLevelState;
use super::GameWonState;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameWonLevelState>()
            .add_systems(OnEnter(GameWonLevelState::Init), setup)
            .add_systems(Update, update.run_if(in_state(GameWonLevelState::Started)))
            .add_systems(OnExit(GameState::GameWon), unload);
    }
}

pub const MAP_WIDTH: f32 = 1000.;
pub const MAP_HEIGHT: f32 = 1000.;

// Used to make sure the level bottom is just below player. So you know to go up.
pub const MAP_VERTICAL_OFFSET: f32 = 300.;

#[derive(Debug, Component)]
pub struct Wall {}

#[derive(Debug, Component)]
pub struct Floor {}

fn unload(
    mut walls: Query<Entity, With<Wall>>,
    mut floor: Query<Entity, With<Floor>>,
    mut house: Query<Entity, With<House>>,
    mut commands: Commands,
    // mut player: Query<Entity, With<Player>>,
) {
    for wall in &mut walls.iter_mut() {
        commands.entity(wall).despawn_recursive();
    }

    for floor in &mut floor.iter_mut() {
        commands.entity(floor).despawn_recursive();
    }

    for house in &mut house.iter_mut() {
        commands.entity(house).despawn_recursive();
    }
}

fn update(
    mut house_query: Query<(&mut TextureAtlasSprite, &Transform, &mut House)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut next_won_state: ResMut<NextState<GameWonState>>,
    mut next_fade_state: ResMut<NextState<FadeState>>,
) {
    let (mut texture_atlas_sprite, house_transform, mut house) = house_query.single_mut();
    let player_transform = player_query.single();

    let detection_area = 32.;

    let is_near_house_x = player_transform.translation.x > -1. * detection_area
        && player_transform.translation.x < detection_area;

    let is_near_house_y =
        player_transform.translation.y > MAP_HEIGHT - MAP_VERTICAL_OFFSET - detection_area;

    println!(
        "player position: {} {}",
        player_transform.translation.x, player_transform.translation.y,
    );
    println!(
        "door position: {} {}",
        house_transform.translation.x, house_transform.translation.y,
    );
    // Check if player is in the right spot to open the door.
    if is_near_house_x && is_near_house_y {
        if !house.is_open {
            println!("Door opened");
            house.is_open = true;
            texture_atlas_sprite.index = house.animation_indices.last;
            next_fade_state.set(FadeState::FadeToBlack);
        }
    }

    if house.is_open {
        println!("Door is open");
        if house.open_timer.finished() {
            println!("Change to Congrats From Game Won");
            next_won_state.set(GameWonState::Congrats);
            // next_level_state.set(GameWonLevelState::Unloaded);
            // next_player_state.set(GameWonPlayerState::Unloaded);
        } else {
            println!("Door tick timer");
            house.open_timer.tick(time.delta());
        }
    }

    // Check for collision with player
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
    println!("STARTING POINT Y {}", starting_point_y);
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
        final_x - MAP_WIDTH / 2.,
        final_y,
        &mut commands,
        &asset_server,
        &mut texture_atlases,
    );

    next_level_state.set(GameWonLevelState::Started);
}

fn draw_side_walls(
    starting_point_x: f32,
    starting_point_y: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let block_width = 12.;
    let block_height = 96.;
    let left_wall_x = starting_point_x;
    let wall_y = starting_point_y;
    let right_wall_x = starting_point_x + MAP_WIDTH - block_width;
    let num_walls = (MAP_HEIGHT / block_height) as i32;

    let animation_indices = AnimationIndices { first: 0, last: 1 };

    // Use only the subset of sprites in the sheet that make up the run animation

    let mut rolling_y = starting_point_y;

    // Left walls
    for index in 0..num_walls {
        let texture_handle = asset_server.load("sprites/level/walls-vertical.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(block_width, block_height),
            2,
            1,
            None,
            None,
        );

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: animation_indices.first,
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform {
                    rotation: Quat::IDENTITY,
                    translation: Vec3::new(left_wall_x, rolling_y, -0.3),
                    ..default()
                },
                ..default()
            },
            Wall {},
        ));

        rolling_y += block_height;
    }

    let mut rolling_y = starting_point_y;

    // Right walls
    for index in 0..num_walls {
        let texture_handle = asset_server.load("sprites/level/walls-vertical.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(block_width, block_height),
            2,
            1,
            None,
            None,
        );

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: animation_indices.last,
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform {
                    rotation: Quat::IDENTITY,
                    translation: Vec3::new(right_wall_x, rolling_y, -0.3),
                    ..default()
                },
                ..default()
            },
            Wall {},
        ));

        rolling_y += block_height;
    }
}

fn draw_bottom_walls(
    starting_wall_x: f32,
    starting_wall_y: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let block_width = 96.;
    let block_height = 8.;
    let num_walls = (MAP_WIDTH / block_width) as i32;

    // Use only the subset of sprites in the sheet that make up the run animation

    let mut rolling_x = starting_wall_x;

    // Left walls
    for index in 0..num_walls {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/level/walls-south.png"),
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform {
                    rotation: Quat::IDENTITY,
                    translation: Vec3::new(rolling_x, starting_wall_y, -0.2),
                    ..default()
                },
                ..default()
            },
            Wall {},
        ));

        rolling_x += block_width;
    }
}

fn draw_top_walls(
    starting_wall_x: f32,
    starting_wall_y: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let block_width = 64.;
    let block_height = 64.;
    let sheet_width = 320.;
    let num_walls = (MAP_WIDTH / block_width) as i32;
    let total_atlas_blocks = (sheet_width / block_width) as usize - 2; // -1 (index) -1 (remove window item last index)

    // Use only the subset of sprites in the sheet that make up the run animation

    let mut rolling_x = starting_wall_x;

    let mut rng = rand::thread_rng();

    for index in 0..num_walls {
        let texture_handle = asset_server.load("sprites/level/walls-north.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(block_width, block_height),
            total_atlas_blocks,
            1,
            None,
            None,
        );

        let random_index: usize = rng.gen_range(0..total_atlas_blocks);

        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: random_index,
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform {
                    rotation: Quat::IDENTITY,
                    translation: Vec3::new(rolling_x, starting_wall_y, -0.2),
                    ..default()
                },
                ..default()
            },
            Wall {},
        ));

        rolling_x += block_width;
    }
}

#[derive(Debug, Component)]
struct House {
    is_open: bool,
    animation_indices: AnimationIndices,
    open_timer: Timer,
}

impl House {
    pub fn new() -> Self {
        Self {
            is_open: false,
            open_timer: Timer::from_seconds(1., TimerMode::Once),
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
    let block_width = 64.;
    let block_height = 64.;

    let texture_handle = asset_server.load("sprites/level/door-north.png");
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
