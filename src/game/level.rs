use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::player::Player;
// use crate::util_fade::FadeState;
use crate::GameState;

pub struct LevelPlugin;
use rand::Rng;

use super::GamePlayState;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        println!("Load game play level plugin");
        app.add_systems(OnEnter(GamePlayState::Init), setup)
            .add_systems(Update, update.run_if(in_state(GamePlayState::Started)))
            .add_systems(OnExit(GameState::Gameplay), unload);
    }
}

pub const MAP_WIDTH: f32 = 3200.;
pub const MAP_HEIGHT: f32 = 3200.;
pub const MAP_MOVABLE_WIDTH: f32 = 2560.;
pub const MAP_MOVABLE_HEIGHT: f32 = 2560.;

#[derive(Debug, Component)]
pub struct Floor {}

fn unload(mut floor: Query<Entity, With<Floor>>, mut commands: Commands) {
    for floor in &mut floor.iter_mut() {
        commands.entity(floor).despawn_recursive();
    }
}

fn update(
    mut player_query: Query<(&mut TextureAtlasSprite, &Transform), With<Player>>,
    // time: Res<Time>,
    // mut next_won_state: ResMut<NextState<GameWonState>>
) {
    let (mut player_sprite, player_transform) = player_query.single_mut();

    // Check for collision with player
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Game play level setup");

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
    let final_y = MAP_HEIGHT / 2.;

    println!("TOTAL TILES {}", total_tiles);

    let mut rng = rand::thread_rng();

    // Start at -1280, -1280 for 2560 size map.
    let starting_point_x = -1. * final_x;
    let starting_point_y = -1. * final_y;

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
}
