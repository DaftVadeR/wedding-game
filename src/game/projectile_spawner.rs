use core::time::Duration;

use bevy::prelude::*;

use crate::sprite::{
    get_translation_for_direction, AnimationIndices, AnimationTimer, DealsDamage, Direction,
    Explosion, ExplosionSpriteSheetAnimatable, Health, Movable, Projectile,
    ProjectileSpriteSheetAnimatable, Weapon, WeaponType,
};
use crate::GameState;

use super::player::Player;
use super::spawner::Enemy;
use super::GamePlayState;

pub struct ProjectileSpawnerPlugin;

impl Plugin for ProjectileSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePlayState::Init), setup)
            .add_systems(OnExit(GameState::Gameplay), unload)
            .add_systems(OnEnter(GamePlayState::Restart), (unload, restart))
            // .add_systems(
            //     Update,
            //     (spawn_weapon_projectiles).run_if(in_state(GamePlayState::Started)),
            // )
            .add_systems(
                Update,
                (
                    update_projectiles,
                    update_projectile_collisions,
                    update_explosions,
                    spawn_weapon_projectiles,
                )
                    .run_if(
                        in_state(GamePlayState::Boss).or_else(in_state(GamePlayState::Started)),
                    ),
            );
    }
}

fn restart(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn spawn_weapon_projectiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // enemy_weapon_query: Query<(&Weapon, &Player), (Without<Enemy>, With<Player>)>,
    mut player_weapon_query: Query<
        (&mut Player, &Transform, &Movable),
        (Without<Enemy>, With<Player>),
    >,
    time: Res<Time>,
) {
    for (mut player, transform, movable) in player_weapon_query.iter_mut() {
        for weapon in player.weapons.iter_mut() {
            weapon.tick_timer.tick(time.delta());

            if !weapon.tick_timer.finished() {
                continue;
            }

            // Only fire when tick timer finished
            match weapon.weapon_type {
                WeaponType::ProjectileStraight => {
                    println!("Firing projectile straight");
                    spawn_simple_straight_projectile(
                        weapon,
                        transform.translation.clone(),
                        get_translation_for_direction(movable.direction, transform.translation.z),
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        &time,
                    );
                }
                WeaponType::ProjectileHoming => {
                    // Alter projectile transform using normalized translation of enemy to player times speed
                }
                WeaponType::SelfAoe => {
                    // Just use aoe attack with 0 distance
                }
                WeaponType::TargetAoe => {
                    //
                }
            }
        }
    }
}

const COLLISION_DISTANCE: f32 = 10.;

fn update_projectile_collisions(
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Health, Entity), (With<Enemy>, Without<Player>)>,
    mut projectile_query: Query<
        (&Transform, &mut Movable, &mut DealsDamage, Entity),
        (With<Projectile>, Without<Player>),
    >,
    time: Res<Time>,
) {
    for (projectile_transform, mut projectile_movable, projectile_damage, projectile) in
        projectile_query.iter_mut()
    {
        // let mut collided = false;

        // // projectile_damage.tick_timer.tick(time.delta());

        // // Check for player collision
        // let distance = enemy_transform
        //     .translation
        //     .distance(player_transform.translation);

        // if distance < COLLISION_DISTANCE {
        //     println!("COLLIDED WITH ENEMY {}", distance);
        //     colliding_enemies.push(ent_original.index());

        //     if enemy_damage.tick_timer.finished() {
        //         enemy_damage.tick_timer.reset();
        //         player_health.total -= enemy_damage.damage;
        //     }

        //     collided = true;
        // }

        for (enemy_transform, mut enemy_health, enemy) in enemy_query.iter_mut() {
            let distance = enemy_transform
                .translation
                .distance(projectile_transform.translation);

            if distance < COLLISION_DISTANCE {
                println!("COLLIDED {}", distance);
                enemy_health.total -= projectile_damage.damage;

                if enemy_health.total <= 0. {
                    commands.entity(enemy).despawn_recursive();
                }

                projectile_movable.is_moving = false;
                commands.entity(projectile).despawn_recursive();

                spawn_explosion_at_position(
                    &assets,
                    &mut texture_atlases,
                    &mut commands,
                    &enemy_transform.translation,
                );

                break;
            }
        }
    }
}

fn spawn_explosion_at_position(
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
    position: &Vec3,
) {
    const PROJECTILE_HEIGHT: f32 = 32.;
    const PROJECTILE_WIDTH: f32 = 32.;

    let anim_indices = AnimationIndices { first: 0, last: 5 };

    let animatable = ExplosionSpriteSheetAnimatable {
        anim_indices: anim_indices.clone(),
    };

    let texture_handle = assets.load("sprites/effects/explosion_anim_spritesheet.png");

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT),
        6,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(anim_indices.first),
            transform: Transform::from_xyz(position.x, position.y, 8.),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        animatable.clone(),
        Explosion { ..default() },
    ));
}

pub fn update_projectiles(
    time: Res<Time>,
    mut projectile_query: Query<
        (
            &mut Transform,
            &mut Movable,
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            Entity,
        ),
        (With<Projectile>, Without<Player>),
    >,
    mut commands: Commands,
) {
    for (
        mut projectile_transform,
        projectile_movable,
        mut projectile_sprite,
        mut projectile_anim_timer,
        entity,
    ) in projectile_query.iter_mut()
    {
        let normal_speed_translation = time.delta_seconds() * projectile_movable.speed;

        let diagonal_speed_translation =
            (normal_speed_translation * normal_speed_translation * 2.).sqrt() / 2.;

        let normalized_translation_for_direction = get_translation_for_direction(
            projectile_movable.direction,
            projectile_transform.translation.z,
        );

        let selected_translation = if normalized_translation_for_direction.x != 0.
            && normalized_translation_for_direction.y != 0.
        {
            diagonal_speed_translation
        } else {
            normal_speed_translation
        };

        let moving = normalized_translation_for_direction * selected_translation;

        projectile_transform.translation.y += moving.y;
        projectile_transform.translation.x += moving.x;

        projectile_anim_timer.tick(time.delta());

        if projectile_anim_timer.just_finished() {
            projectile_sprite.index =
                if projectile_sprite.index >= projectile_movable.current_animation_indices.last {
                    projectile_movable.current_animation_indices.first
                } else {
                    projectile_sprite.index + 1
                }
        }

        if projectile_anim_timer.elapsed() > Duration::from_secs(10) {
            commands.entity(entity).despawn_recursive();
        }

        // Update position
    }
}

pub fn update_explosions(
    time: Res<Time>,
    mut explosions_query: Query<
        (
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &ExplosionSpriteSheetAnimatable,
            Entity,
        ),
        With<Explosion>,
    >,
    mut commands: Commands,
) {
    for (mut explosion_sprite, mut explosion_anim_timer, mut explosion_animatable, entity) in
        explosions_query.iter_mut()
    {
        explosion_anim_timer.tick(time.delta());

        if explosion_anim_timer.just_finished() {
            if explosion_sprite.index >= explosion_animatable.anim_indices.last {
                commands.entity(entity).despawn_recursive();
            } else {
                explosion_sprite.index = explosion_sprite.index + 1;
            }
        }

        // Update position
    }
}

// fn spawn_projectiles_for_weapons(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     mut level_spawns: ResMut<LevelSpawns>,
//     player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
//     time: Res<Time>,
//     mut next_state: ResMut<NextState<GamePlayState>>,
//     state: Res<State<GamePlayState>>,
// ) {
//     commands.spawn((
//         SpriteSheetBundle {
//             texture_atlas: texture_atlas_handle.clone(),
//             sprite: TextureAtlasSprite::new(run_animation_indices.first),
//             transform: Transform::from_xyz(
//                 final_x_pos,
//                 final_y_pos,
//                 (1 + level_spawns.current_stage) as f32,
//             ),
//             ..default()
//         },
//         AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
//         animatable.clone(),
//         Movable {
//             speed: 50.,
//             direction: Direction::Right,
//             is_moving: false,
//             current_animation_indices: idle_animation_indices,
//             is_collided: false,
//             is_state_changed: true,
//         },
//         Health { total: 10. },
//         Enemy,
//         DealsDamage {
//             damage: (10. + (level_spawns.current_stage as f32)),
//             tick_timer: Timer::from_seconds(1., TimerMode::Once),
//         },
//     ));
// }

// fn get_guitar_projectile(
//     texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
//     assets: &Res<AssetServer>,
// ) -> (
//     Handle<TextureAtlas>,
//     EnemySpriteSheetAnimatable,
//     AnimationIndices,
//     AnimationIndices,
// ) {
// }

fn spawn_simple_straight_projectile(
    weapon: &Weapon,
    origin: Vec3,
    direction_translation: Vec3,
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    time: &Res<Time>,
) {
    const PROJECTILE_HEIGHT: f32 = 64.;
    const PROJECTILE_WIDTH: f32 = 64.;

    let moving_anim_indices = AnimationIndices { first: 0, last: 2 };

    let animatable: ProjectileSpriteSheetAnimatable = ProjectileSpriteSheetAnimatable {
        moving_anim_indices: moving_anim_indices.clone(),
    };

    let texture_handle = assets.load(weapon.pic_sprite);

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT),
        3,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(moving_anim_indices.first),
            transform: Transform {
                translation: Vec3 {
                    x: origin.x,
                    y: origin.y,
                    z: 9.,
                },
                scale: Vec3::new(weapon.scale, weapon.scale, 1.),
                ..Default::default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        animatable.clone(),
        Movable {
            speed: 100.,
            direction: Direction::Custom(direction_translation),
            is_moving: true,
            current_animation_indices: moving_anim_indices,
            is_collided: false,
            is_state_changed: true,
        },
        Projectile,
        DealsDamage {
            damage: 10.,
            tick_timer: Timer::from_seconds(1., TimerMode::Once),
        },
    ));
}

pub fn unload(
    mut projectile_query: Query<Entity, With<Projectile>>,
    mut collision_query: Query<Entity, With<Explosion>>,
    // mut level_spawns: ResMut<LevelSpawns>,
    mut commands: Commands,
) {
    for entity in projectile_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in collision_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
