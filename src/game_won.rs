use crate::GameState;
use bevy::app::{AppExit, Plugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

pub struct GameWonPlugin;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum GameWonState {
    #[default]
    Unloaded,
    Init,
    Started,
}

impl Plugin for GameWonPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameWonState>()
            .add_systems(
                OnEnter(GameState::GameWon),
                (reset_camera, spawn_game_won_scene),
            )
            .add_systems(OnExit(GameState::GameWon), despawn_game_won_scene);
    }
}

#[derive(Debug, Component)]
pub struct Floor {}

const MAP_WIDTH: f32 = 960.;
const MAP_HEIGHT: f32 = 1440.;
const MAP_VERTICAL_OFFSET: f32 = 480.;

fn spawn_game_won_scene(
    mut commands: Commands,
    mut next_game_won_state: ResMut<NextState<GameWonState>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Loading game won plugin");
    // commands.spawn(
    //     (SpriteBundle {
    //         texture: asset_server.load("sprites/level/door-north.png"),
    //         sprite: Sprite {
    //             anchor: Anchor::Center,
    //             ..default()
    //         },
    //         transform: Transform {
    //             rotation: Quat::IDENTITY,
    //             translation: Vec3::new(0., 0., -1.),
    //             ..default()
    //         },
    //         ..default()
    //     }),
    // );

    // Sprite stuff
    let block_width = 32.;
    let block_height = 32.;
    let block_cols = 8;
    let block_rows = 8;
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
    let starting_point_y = -1. * MAP_VERTICAL_OFFSET;
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

    // SpriteBundle {
    //     texture: asset_server.load("sprites/level/tx_tileset_grass.png"),
    //     transform: Transform::from_xyz(0., 0., 1),
    //     ..Default::default()
    // },

    next_game_won_state.set(GameWonState::Started);
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

fn despawn_game_won_scene(
    mut exit: EventWriter<AppExit>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("Exiting game won plugin and game");
    exit.send(AppExit);
}

#[derive(Component)]
pub struct NpcComponent;
