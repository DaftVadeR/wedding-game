// use super::enemies::{
//     get_enemy_for_type, Cat, EnemyType, Goblin, GoblinShadow, Harmful, Mushroom, Slime,
//     SpawnedEnemy,
// };

use core::num;

use super::level::{MAP_HEIGHT, MAP_WIDTH};
use super::player::Player;
use super::GamePlayState;

use crate::sprite::{
    AnimationIndices, AnimationTimer, DealsDamage, Direction, EnemySpriteSheetAnimatable, Health,
    Movable,
};

use crate::GameState;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::prelude::*;

pub struct EnemySpawnerPlugin;

#[derive(PartialEq, Eq, Default, Debug, Clone, Hash)]
enum EnemyType {
    #[default]
    Basic,
}

#[derive(Debug, Component)]
struct Enemy {}

#[derive(Debug)]
pub struct SpawnWave {
    pub enemy: EnemyType,
}

const SPAWN_DISTANCE: f32 = 500.;

#[derive(Resource)]
pub struct LevelSpawns {
    pub wave_type: SpawnWave,
    pub global_timer: Stopwatch,
    pub wave_timer: Timer,
    pub stage_timer: Timer,
    pub current_stage: usize,
}

impl LevelSpawns {
    pub fn new() -> Self {
        Self {
            global_timer: Stopwatch::new(),
            stage_timer: Timer::from_seconds(60., TimerMode::Repeating),
            wave_timer: Timer::from_seconds(5., TimerMode::Repeating),
            current_stage: 1,
            wave_type: SpawnWave {
                enemy: EnemyType::Basic,
            },
            // percent_trigger: Timer::from_seconds(4.0, TimerMode::Repeating),
            // main_timer: Timer::from_seconds(20.0, TimerMode::Once),
        }
    }
}

impl Default for LevelSpawns {
    fn default() -> Self {
        Self::new()
    }
}
// #[derive()]
// pub struct SpawnStage {
//     pub mobs: Vec<SpawnMob>,
// }

// pub struct SpawnMob {
//     pub enemy: EnemyType,
//     pub count: u32,
// }

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup);
        app.init_resource::<LevelSpawns>();
        // app.add_systems(Update, check_for_spawns);

        app.add_systems(OnEnter(GamePlayState::Init), setup)
            .add_systems(OnExit(GameState::Gameplay), unload)
            .add_systems(
                Update,
                (check_for_spawns).run_if(in_state(GamePlayState::Started)),
            );

        // app.add_systems(Update, animate_sprite);
        // app.add_systems(Update, player_movement);
        // app.add_systems(Update, update_camera_from_player_position);

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
        //     .add_schedule(fixed_update_loop_schedule)
        //     .init_resource::<MainScheduleOrder>()
        //     .add_systems(Main, Main::run_main);
    }
}

pub fn unload(mut query: Query<Entity, With<Player>>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

// // Simple format for defining spawns for a level.
// fn get_first_level_spawns() -> LevelSpawns {
//     LevelSpawns {
//         global_timer: Stopwatch::new(),
//         stage_timer: Timer::from_seconds(60., TimerMode::Repeating),
//         wave_timer: Timer::from_seconds(5., TimerMode::Repeating),
//         current_stage: 0,
//         spawns: vec![
//             SpawnStage {
//                 mobs: vec![SpawnMob {
//                     enemy: EnemyType::Goblin,
//                     count: 5,
//                 }],
//             },
//             SpawnStage {
//                 mobs: vec![
//                     SpawnMob {
//                         enemy: EnemyType::Goblin,
//                         count: 7,
//                     },
//                     // SpawnMob {
//                     //     enemy: EnemyType::Mushroom,
//                     //     count: 2,
//                     // },
//                 ],
//             },
//             SpawnStage {
//                 mobs: vec![
//                     // SpawnMob {
//                     //     enemy: EnemyType::Mushroom,
//                     //     count: 5,
//                     // },
//                     SpawnMob {
//                         enemy: EnemyType::Goblin,
//                         count: 15,
//                     },
//                 ],
//             },
//         ],
//     }
// }

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((Enemy {}));

    // commands.spawn()gcc=
}

fn check_for_spawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level_spawns: ResMut<LevelSpawns>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let transform = player_query.single();

    level_spawns.global_timer.tick(time.delta());
    level_spawns.global_timer.tick(time.delta());
    level_spawns.wave_timer.tick(time.delta());
    level_spawns.stage_timer.tick(time.delta());

    let num_enemies_per_spawn = 5 * level_spawns.current_stage;

    if level_spawns.wave_timer.just_finished() {
        println!("Wave finished");
        // if level_spawns.spawns.len() > level_spawns.current_stage {
        // let waves = &level_spawns.spawns[level_spawns.current_stage];
        let player_position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y);
        // // println!("Spawn wave {}", level_spawns.current_stage);
        // spawn_enemy_wave(
        //     level_spawns.current_stage,
        //     waves,
        //     &mut commands,
        //     &asset_server,
        //     &mut texture_atlases,
        //     player_position,
        // );
        spawn_enemies(
            num_enemies_per_spawn,
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            player_position,
            &level_spawns,
        )
        // }
    }

    // Update stage to next stage if another stage exists in array.
    if level_spawns.stage_timer.just_finished() {
        println!("Stage finished");
        let new_stage = level_spawns.current_stage + 1;

        // if level_spawns.spawns.len() > new_stage {
        // level_spawns.current_stage = new_stage;
        // } else {
        // println!("No stage to progress to");
        // }
    }
}

const ENEMY_WIDTH: f32 = 10.;
const ENEMY_HEIGHT: f32 = 11.;

fn spawn_enemies(
    num_enemies: usize,
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    player_position: Vec2,
    level_spawns: &ResMut<LevelSpawns>,
) {
    level_spawns.current_stage;

    let idle_animation_indices = AnimationIndices { first: 0, last: 1 };
    let run_animation_indices = AnimationIndices { first: 0, last: 2 };

    let animatable = (EnemySpriteSheetAnimatable {
        idle_anim_indices: idle_animation_indices.clone(),
        moving_anim_indices: run_animation_indices.clone(),
    });

    let texture_handle = assets.load("sprites/enemy/basic.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
        3,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut rng = rand::thread_rng();

    println!("Wave spawn - no enemies - {}", num_enemies);

    for i in 1..num_enemies {
        let rnd_x: f32 = rng.gen_range(0. ..SPAWN_DISTANCE);
        let rnd_y: f32 = SPAWN_DISTANCE - rnd_x;
        let switch: bool = rng.gen_bool(0.5); // Randomly use negative number so enemies spawn on both
                                              // negative and positive x+y axis.
        let x_pos = player_position.x + if switch { rnd_x * -1. } else { rnd_x };
        let y_pos = player_position.y + if switch { rnd_y * -1. } else { rnd_y };

        let final_x_pos = x_pos.clamp(-1. * MAP_WIDTH / 2., MAP_WIDTH / 2.);
        let final_y_pos = y_pos.clamp(-1. * MAP_HEIGHT / 2., MAP_HEIGHT / 2.);
        println!("Spawn enemy no {} at {} x {}", i, final_x_pos, final_y_pos);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(run_animation_indices.first),
                transform: Transform::from_xyz(final_x_pos, final_y_pos, 2.),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            animatable.clone(),
            Movable {
                speed: 50.,
                direction: Direction::Right,
                is_moving: false,
                current_animation_indices: idle_animation_indices,
            },
            Health(10.),
            DealsDamage {
                damage: (10. + (level_spawns.current_stage as f32)),
            },
        ));
    }
}

// fn spawn_enemy_wave(
//     current_stage_index: usize,
//     stage: &SpawnStage,
//     commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
//     mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
//     player_pos: Vec2,
// ) {
//     for mob in &stage.mobs {
//         for index in 0..mob.count {
//             println!("Spawn enemy no {} for stage {}", index, current_stage_index);
//             let to_spawn = get_enemy_for_type(&mob.enemy, asset_server, texture_atlases);
//             let texture_handle = asset_server.load(to_spawn.get_sprite_location());
//             let texture_atlas = TextureAtlas::from_grid(
//                 texture_handle,
//                 to_spawn.get_sprite_size(),
//                 to_spawn.get_sprite_grid().0,
//                 to_spawn.get_sprite_grid().1,
//                 None,
//                 None,
//             );

//             // texture_atlas.
//             // let texture_atlas_run =
//             //     TextureAtlas::from_grid(texture_handle_run, Vec2::new(16.0, 16.0), 6, 1, None, None);

//             let texture_atlas_handle = texture_atlases.add(texture_atlas);

//             // Spawns enemy
//             spawn_enemy_at_player(
//                 to_spawn,
//                 mob,
//                 texture_atlas_handle,
//                 texture_atlases,
//                 commands,
//                 asset_server,
//                 current_stage_index,
//                 player_pos,
//             );
//         }
//     }
// }

// fn spawn_enemy_at_player(
//     to_spawn: Box<dyn SpawnedEnemy>,
//     mob: &SpawnMob,
//     texture_atlas_handle: Handle<TextureAtlas>,
//     mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
//     mut commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
//     current_stage: usize,
//     player_pos: Vec2,
// ) {
//     // let texture_atlas_run_handle = texture_atlases.add(texture_atlas_run);

//     // Use only the subset of sprites in the sheet that make up the run animation
//     // Unimportant, just set to running anim ones until I use idle for something on enemies.
//     let idle_animation_indices = AnimationIndices {
//         first: to_spawn.get_sprite_indices().0,
//         last: to_spawn.get_sprite_indices().1 + 1,
//     };

//     // Run
//     let run_animation_indices = AnimationIndices {
//         first: to_spawn.get_sprite_indices().0,
//         last: to_spawn.get_sprite_indices().1,
//     };

//     let mut rng = rand::thread_rng();
//     let rnd_x: f32 = rng.gen_range(0. ..SPAWN_DISTANCE);
//     let rnd_y: f32 = SPAWN_DISTANCE - rnd_x;
//     let switch: bool = rng.gen_bool(0.5); // Randomly use negative number so enemies spawn on both
//                                           // negative and positive x+y axis.
//     let x_pos = player_pos.x + if switch { rnd_x * -1. } else { rnd_x };
//     let y_pos = player_pos.y + if switch { rnd_y * -1. } else { rnd_y };

//     let final_x_pos = x_pos.clamp(-1. * MAP_WIDTH, MAP_WIDTH);
//     let final_y_pos = y_pos.clamp(-1. * MAP_HEIGHT, MAP_HEIGHT);

//     spawn_enemy(
//         texture_atlas_handle,
//         idle_animation_indices,
//         run_animation_indices,
//         Vec3::new(final_x_pos, final_y_pos, 1.),
//         commands,
//         current_stage,
//         to_spawn,
//         mob,
//     );
// }

// fn spawn_goblin() -> Goblin {
//     return Goblin {};
// }

// fn spawn_enemy(
//     texture_atlas_handle: Handle<TextureAtlas>,
//     idle_animation_indices: AnimationIndices,
//     run_animation_indices: AnimationIndices,
//     translation: Vec3,
//     commands: &mut Commands,
//     current_stage: usize,
//     to_spawn: Box<dyn SpawnedEnemy<dyn Component>>,
//     mob: &SpawnMob,
// ) {
//     // let enemy: Box<dyn Component>;
//     // if mob.enemy == EnemyType::Goblin {
//     //     enemy = Goblin
//     // } else if mob.enemy == EnemyType::GoblinShadow {
//     //     enemy = GoblinShadow
//     // } else if mob.enemy == EnemyType::Cat {
//     //     Cat
//     // } else if mob.enemy == EnemyType::Mushroom {
//     //     Mushroom
//     // } else if mob.enemy == EnemyType::Slime {
//     //     Slime
//     // } else {
//     //     panic!("fuck you");
//     // };
// }
