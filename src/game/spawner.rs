use super::level::{MAP_HEIGHT, MAP_WIDTH};
use super::player::{self, Player};
use super::GamePlayState;

use crate::game::level;
use crate::sprite::{
    AnimationIndices, AnimationTimer, DealsDamage, Direction, EnemySpriteSheetAnimatable, Health,
    Movable,
};

use crate::GameState;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::prelude::*;

#[derive(PartialEq, Eq, Default, Debug, Clone, Hash)]
enum EnemyType {
    #[default]
    Basic,
}

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug)]
pub struct SpawnWave {
    pub enemy: EnemyType,
}

const SPAWN_DISTANCE: f32 = 500.;
const COLLISION_DISTANCE: f32 = 10.;

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
            stage_timer: Timer::from_seconds(15., TimerMode::Repeating),
            wave_timer: Timer::from_seconds(10., TimerMode::Repeating),
            current_stage: 3,
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

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup);
        app.init_resource::<LevelSpawns>();
        // app.add_systems(Update, check_for_spawns);

        app.add_systems(OnEnter(GamePlayState::Init), setup)
            .add_systems(OnExit(GameState::Gameplay), unload)
            .add_systems(OnEnter(GamePlayState::Restart), (unload, restart))
            .add_systems(OnEnter(GamePlayState::Boss), spawn_boss)
            .add_systems(
                Update,
                (
                    check_for_spawns,
                    update_enemy_positions_and_sprites,
                    update_enemy_collisions,
                )
                    .run_if(in_state(GamePlayState::Started)),
            )
            .add_systems(
                Update,
                (update_enemy_positions_and_sprites, update_enemy_collisions)
                    .run_if(in_state(GamePlayState::Boss)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn restart(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.init_resource::<LevelSpawns>();
}

fn check_for_spawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level_spawns: ResMut<LevelSpawns>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GamePlayState>>,
    state: Res<State<GamePlayState>>,
) {
    let transform = player_query.single();

    level_spawns.global_timer.tick(time.delta());
    level_spawns.global_timer.tick(time.delta());
    level_spawns.wave_timer.tick(time.delta());
    level_spawns.stage_timer.tick(time.delta());

    let num_enemies_per_spawn = 5 * level_spawns.current_stage;

    if level_spawns.wave_timer.just_finished() {
        let player_position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y);

        spawn_enemies(
            num_enemies_per_spawn,
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            player_position,
            &level_spawns,
        );
    }

    // Update stage to next stage if another stage exists in array.
    if level_spawns.stage_timer.just_finished() {
        println!("Stage finished");
        level_spawns.current_stage = level_spawns.current_stage + 1;

        if level_spawns.current_stage > 4 && state.get() != &GamePlayState::Boss {
            next_state.set(GamePlayState::Boss);
        }
    }
}

fn spawn_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level_spawns: ResMut<LevelSpawns>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GamePlayState>>,
    state: Res<State<GamePlayState>>,
) {
    let transform = player_query.single();

    if level_spawns.wave_timer.just_finished() {
        let player_position: Vec2 = Vec2::new(transform.translation.x, transform.translation.y);

        spawn_enemies(
            1,
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            player_position,
            &level_spawns,
        );
    }
}

fn get_basic_enemy(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets: &Res<AssetServer>,
) -> (
    Handle<TextureAtlas>,
    EnemySpriteSheetAnimatable,
    AnimationIndices,
    AnimationIndices,
) {
    const ENEMY_WIDTH: f32 = 10.;
    const ENEMY_HEIGHT: f32 = 11.;

    let idle_animation_indices = AnimationIndices { first: 0, last: 1 };
    let run_animation_indices = AnimationIndices { first: 0, last: 2 };

    let animatable: EnemySpriteSheetAnimatable = (EnemySpriteSheetAnimatable {
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

    (
        texture_atlas_handle,
        animatable,
        idle_animation_indices,
        run_animation_indices,
    )
}

fn get_brown_mushroom_enemy(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets: &Res<AssetServer>,
) -> (
    Handle<TextureAtlas>,
    EnemySpriteSheetAnimatable,
    AnimationIndices,
    AnimationIndices,
) {
    const ENEMY_WIDTH: f32 = 16.;
    const ENEMY_HEIGHT: f32 = 16.;

    let idle_animation_indices = AnimationIndices { first: 0, last: 5 };
    let run_animation_indices = AnimationIndices { first: 8, last: 15 };

    let animatable: EnemySpriteSheetAnimatable = (EnemySpriteSheetAnimatable {
        idle_anim_indices: idle_animation_indices.clone(),
        moving_anim_indices: run_animation_indices.clone(),
    });

    let texture_handle = assets.load("sprites/enemy/mushroom_brown/sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
        7,
        8,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    (
        texture_atlas_handle,
        animatable,
        idle_animation_indices,
        run_animation_indices,
    )
}

fn get_slime_enemy(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets: &Res<AssetServer>,
) -> (
    Handle<TextureAtlas>,
    EnemySpriteSheetAnimatable,
    AnimationIndices,
    AnimationIndices,
) {
    const ENEMY_WIDTH: f32 = 16.;
    const ENEMY_HEIGHT: f32 = 16.;

    let idle_animation_indices = AnimationIndices { first: 0, last: 1 };
    let run_animation_indices = AnimationIndices { first: 0, last: 5 };

    let animatable: EnemySpriteSheetAnimatable = (EnemySpriteSheetAnimatable {
        idle_anim_indices: idle_animation_indices.clone(),
        moving_anim_indices: run_animation_indices.clone(),
    });

    let texture_handle = assets.load("sprites/enemy/slime/slime_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
        6,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    (
        texture_atlas_handle,
        animatable,
        idle_animation_indices,
        run_animation_indices,
    )
}

fn get_goblin_enemy(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets: &Res<AssetServer>,
) -> (
    Handle<TextureAtlas>,
    EnemySpriteSheetAnimatable,
    AnimationIndices,
    AnimationIndices,
) {
    const ENEMY_WIDTH: f32 = 16.;
    const ENEMY_HEIGHT: f32 = 16.;

    let idle_animation_indices = AnimationIndices { first: 6, last: 8 };
    let run_animation_indices = AnimationIndices { first: 0, last: 5 };

    let animatable: EnemySpriteSheetAnimatable = (EnemySpriteSheetAnimatable {
        idle_anim_indices: idle_animation_indices.clone(),
        moving_anim_indices: run_animation_indices.clone(),
    });

    let texture_handle = assets.load("sprites/enemy/goblin/goblin_spritesheet_widle.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
        6,
        2,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    (
        texture_atlas_handle,
        animatable,
        idle_animation_indices,
        run_animation_indices,
    )
}

fn get_boss_enemy(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets: &Res<AssetServer>,
) -> (
    Handle<TextureAtlas>,
    EnemySpriteSheetAnimatable,
    AnimationIndices,
    AnimationIndices,
) {
    const ENEMY_WIDTH: f32 = 94.;
    const ENEMY_HEIGHT: f32 = 108.;

    let idle_animation_indices = AnimationIndices {
        first: 12,
        last: 17,
    };
    let run_animation_indices = AnimationIndices { first: 0, last: 11 };

    let animatable: EnemySpriteSheetAnimatable = (EnemySpriteSheetAnimatable {
        idle_anim_indices: idle_animation_indices.clone(),
        moving_anim_indices: run_animation_indices.clone(),
    });

    let texture_handle = assets.load("sprites/enemy/boss/boss.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
        12,
        2,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    (
        texture_atlas_handle,
        animatable,
        idle_animation_indices,
        run_animation_indices,
    )
}

fn get_bat_enemy(
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets: &Res<AssetServer>,
) -> (
    Handle<TextureAtlas>,
    EnemySpriteSheetAnimatable,
    AnimationIndices,
    AnimationIndices,
) {
    const ENEMY_WIDTH: f32 = 16.;
    const ENEMY_HEIGHT: f32 = 16.;

    let idle_animation_indices = AnimationIndices { first: 0, last: 3 };
    let run_animation_indices = AnimationIndices { first: 0, last: 3 };

    let animatable: EnemySpriteSheetAnimatable = (EnemySpriteSheetAnimatable {
        idle_anim_indices: idle_animation_indices.clone(),
        moving_anim_indices: run_animation_indices.clone(),
    });

    let texture_handle = assets.load("sprites/enemy/bat/bat.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
        4,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    (
        texture_atlas_handle,
        animatable,
        idle_animation_indices,
        run_animation_indices,
    )
}

fn spawn_enemies(
    num_enemies: usize,
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    player_position: Vec2,
    level_spawns: &ResMut<LevelSpawns>,
) {
    let (texture_atlas_handle, animatable, idle_animation_indices, run_animation_indices) =
        if level_spawns.current_stage == 1 {
            get_goblin_enemy(texture_atlases, assets)
        } else if level_spawns.current_stage == 2 {
            get_brown_mushroom_enemy(texture_atlases, assets)
        } else if level_spawns.current_stage == 3 {
            get_slime_enemy(texture_atlases, assets)
        } else if level_spawns.current_stage == 4 {
            get_bat_enemy(texture_atlases, assets)
        } else {
            get_boss_enemy(texture_atlases, assets)
        };

    let mut rng: ThreadRng = rand::thread_rng();

    println!("Wave spawn - no enemies - {}", num_enemies);

    for i in 0..num_enemies {
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
                transform: Transform::from_xyz(
                    final_x_pos,
                    final_y_pos,
                    (1 + level_spawns.current_stage) as f32,
                ),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            animatable.clone(),
            Movable {
                speed: 50.,
                direction: Direction::Right,
                is_moving: false,
                current_animation_indices: idle_animation_indices,
                is_collided: false,
                is_state_changed: true,
            },
            Health { total: 10. },
            Enemy,
            DealsDamage {
                damage: (10. + (level_spawns.current_stage as f32)),
                tick_timer: Timer::from_seconds(1., TimerMode::Once),
            },
        ));
    }
}

pub fn get_indices_for_movable(
    movable: &mut Movable,
    animateable: &EnemySpriteSheetAnimatable,
) -> Option<AnimationIndices> {
    let chosen: AnimationIndices;

    if !movable.is_moving {
        chosen = animateable.idle_anim_indices.clone();
    } else {
        chosen = animateable.moving_anim_indices.clone();
    }

    return Some(chosen);
}

fn update_enemy_collisions(
    mut player_query: Query<(&Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    mut enemy_query_collision: Query<
        (&Transform, &mut Movable, &mut DealsDamage, Entity),
        (With<Enemy>, Without<Player>),
    >,
    time: Res<Time>,
) {
    let (player_transform, mut player_health) = player_query.single_mut();

    // How I'll do collision:

    // Two loops, one for primary entity, one nested, for collision checks.

    // So looping through entities twice.

    // In second loop, I just need to check the first loops entity values for

    // - is_collided bool value. If it has already been triggered, ignore and move on. Actually, just break from the loop.
    // - transform translation check for collision, and then set the is_collided value to true on first loops enemy component

    // Do the same for player collision, possibly in the same loop to prevent conflicts between multiple loops by being able to check already changed values immediately.

    // Once collided with the player, stop.
    let mut colliding_enemies: Vec<u32> = vec![];

    // Damage ticks are independent of collision state or movement. As long as in general vicinity, trigger damage tick.
    for (enemy_transform, _, mut enemy_damage, ent_original) in enemy_query_collision.iter_mut() {
        let mut collided = false;

        enemy_damage.tick_timer.tick(time.delta());

        // Check for player collision
        let distance = enemy_transform
            .translation
            .distance(player_transform.translation);

        if distance < COLLISION_DISTANCE {
            println!("COLLIDED WITH PLAYER {}", distance);
            colliding_enemies.push(ent_original.index());

            if enemy_damage.tick_timer.finished() {
                enemy_damage.tick_timer.reset();
                player_health.total -= enemy_damage.damage;
            }

            collided = true;
        }

        // If no player collision, check for fellow enemy collisions
        // if !collided {
        //     for (other_transform, _, ent) in enemy_query_collision.iter() {
        //         if colliding_enemies.contains(&ent.index()) || ent_original.index() == ent.index() {
        //             continue;
        //         }

        //         let distance = enemy_transform
        //             .translation
        //             .distance(other_transform.translation);

        //         if distance < COLLISION_DISTANCE {
        //             println!("COLLIDED {}", distance);
        //             colliding_enemies.push(ent_original.index());
        //             break;
        //         }
        //     }
        // }
    }

    for (_, mut enemy_movable, _, ent) in enemy_query_collision.iter_mut() {
        let old_is_collided = enemy_movable.is_collided;

        if colliding_enemies.contains(&ent.index()) {
            enemy_movable.is_collided = true;
            println!("COLLIDED VALUE SET");
        } else {
            enemy_movable.is_collided = false;
        }

        if old_is_collided != enemy_movable.is_collided {
            enemy_movable.is_state_changed = true;
        } else {
            enemy_movable.is_state_changed = false;
        }
    }
}

pub fn update_enemy_positions_and_sprites(
    time: Res<Time>,
    mut enemy_query: Query<
        (
            &mut Transform,
            &mut Movable,
            &EnemySpriteSheetAnimatable,
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &Health,
            Entity,
        ),
        (With<Enemy>, Without<Player>),
    >,
    player_query: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    mut commands: Commands,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (
            mut enemy_transform,
            mut enemy_movable,
            enemy_animatable,
            mut enemy_sprite,
            mut enemy_timer,
            health,
            entity,
        ) in enemy_query.iter_mut()
        {
            if health.total <= 0. {
                commands.entity(entity).despawn();
            } else {
                let old_z = enemy_transform.translation.z;
                // Begin check to MOVE towards player
                let old_is_moving = enemy_movable.is_moving;

                let normalized_translation =
                    Vec3::normalize(player_transform.translation - enemy_transform.translation);

                let moving = normalized_translation * enemy_movable.speed * time.delta_seconds();

                if !enemy_movable.is_collided {
                    enemy_transform.translation += moving;
                    enemy_transform.translation.z = old_z; // Think its overriding it here, so make sure it stays same as from spawn

                    if normalized_translation.x > 0. {
                        enemy_sprite.flip_x = false;
                        enemy_movable.is_moving = true;
                    } else if normalized_translation.x < 0. {
                        enemy_sprite.flip_x = true;
                        enemy_movable.is_moving = true;
                    } else if normalized_translation.y > 0. || normalized_translation.y < 0. {
                        enemy_movable.is_moving = true;
                    } else {
                        enemy_movable.is_moving = false;
                    }
                } else {
                    println!("COLLIDED VALUE DETECTED - STOPPING");
                    enemy_movable.is_moving = false;
                }

                // println!("TRANSLATION {}", normalized_translation);

                // IMPORTANT - need to compare with prior frame state to make sure not resetting anim unnecessary, but also
                // makes sure to reset on EVERY movement or direction change.
                if enemy_movable.is_state_changed || enemy_movable.is_moving != old_is_moving {
                    let chosen: Option<AnimationIndices> =
                        get_indices_for_movable(&mut enemy_movable, &enemy_animatable);

                    if chosen.is_some() {
                        enemy_movable.current_animation_indices = chosen.unwrap();
                    }

                    enemy_sprite.index = enemy_movable.current_animation_indices.first;
                } else {
                    enemy_timer.tick(time.delta());

                    if enemy_timer.just_finished() {
                        enemy_sprite.index =
                            if enemy_sprite.index >= enemy_movable.current_animation_indices.last {
                                enemy_movable.current_animation_indices.first
                            } else {
                                enemy_sprite.index + 1
                            }
                    }
                }
                enemy_movable.is_state_changed = false;
            }
        }
    }
}

pub fn unload(
    mut query: Query<Entity, With<Enemy>>,
    mut level_spawns: ResMut<LevelSpawns>,
    mut commands: Commands,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<LevelSpawns>();
}
