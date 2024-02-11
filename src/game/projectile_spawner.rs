use core::time::Duration;

use bevy::prelude::*;

use crate::game::weapons::{Explosion, WeaponsEnum};
use crate::sprite::{
    get_translation_for_direction, AnimationIndices, AnimationTimer, DealsDamage, Direction,
    EffectSpriteSheetAnimatable, Health, Movable,
};
use crate::GameState;

use super::player::{CanLevel, Player};
use super::spawner::{Enemy, GivesExperience};
use super::weapons::{get_weapon_sprite, DamageEffect, Projectile, ProjectileCategory, Weapon};
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
                    update_explosions_damage_effects,
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
            match weapon.projectile_props.projectile_category {
                ProjectileCategory::ProjectileStraight => {
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
                ProjectileCategory::ProjectileHoming => {
                    // Alter projectile transform using normalized translation of enemy to player times speed
                }
                ProjectileCategory::SelfAoe => {
                    // Just use aoe attack with 0 distance
                }
                ProjectileCategory::TargetAoe => {
                    println!("Firing projectile aoe");
                    spawn_target_aoe_projectile(
                        weapon,
                        transform.translation.clone(),
                        get_translation_for_direction(movable.direction, transform.translation.z),
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        &time,
                    );
                }
            }
        }
    }
}

struct DamageEvent {
    damage: f32,
    entity_id: u32,
    damage_type: DamageType,
}
pub enum DamageType {
    Normal,
    Aoe,
}

fn update_projectile_collisions(
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut enemy_query: Query<
        (&Transform, &mut Health, &GivesExperience, Entity),
        (With<Enemy>, Without<Player>),
    >,
    mut projectile_query: Query<
        (
            &Transform,
            &mut Movable,
            &mut DealsDamage,
            &Projectile,
            Entity,
        ),
        (With<Projectile>, Without<Player>),
    >,
    mut player_query: Query<(&mut CanLevel, &Player), With<Player>>,
    mut next_play_state: ResMut<NextState<GamePlayState>>,
    time: Res<Time>,
) {
    let (mut lvl, player) = player_query.single_mut();

    for (
        projectile_transform,
        mut projectile_movable,
        projectile_damage,
        projectile,
        projectile_entity,
    ) in projectile_query.iter_mut()
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

        let mut damage_events: Vec<DamageEvent> = Vec::new();

        for (enemy_transform, _, _, enemy) in enemy_query.iter() {
            let distance = enemy_transform
                .translation
                .distance(projectile_transform.translation);

            // IF normal projectile, no checking for aoe - just kill initial collided enemy and remove projectile entity on FIRST enemy hit.

            // If aoe projectile, add distance from aoe radius to collision distance. Isn't simplified to be on all projectiles as fundamentally different in that the normal projectile goes on FIRST hit,
            // but doesn't immediately disappear after the first collision - only after the loop has been finished do we despawn it.

            let collision_distance = (projectile.props.projectile_sprite_width
                * projectile.props.projectile_sprite_scale)
                / 2.;

            if distance < collision_distance {
                // println!("COLLIDED {}", distance);
                if projectile.props.projectile_category == ProjectileCategory::ProjectileStraight {
                    damage_events.push(DamageEvent {
                        damage: projectile_damage.damage,
                        entity_id: enemy.index(),
                        damage_type: DamageType::Normal,
                    });

                    break;
                } else if projectile.props.projectile_category == ProjectileCategory::TargetAoe {
                    // Collided with an enemy. Now trigger damage to everything in area.
                    // let damage_radius = projectile.props.projectile_aoe_radius + collision_distance;

                    for (enemy_transform, _, _, enemy) in enemy_query.iter() {
                        let aoe_distance = enemy_transform
                            .translation
                            .distance(projectile_transform.translation);

                        // IF normal projectile, no checking for aoe - just kill initial collided enemy and remove projectile entity on FIRST enemy hit.

                        // If aoe projectile, add distance from aoe radius to collision distance. Isn't simplified to be on all projectiles as fundamentally different in that the normal projectile goes on FIRST hit,
                        // but doesn't immediately disappear after the first collision - only after the loop has been finished do we despawn it.

                        if aoe_distance < projectile.props.projectile_aoe_radius {
                            damage_events.push(DamageEvent {
                                damage: projectile_damage.damage,
                                entity_id: enemy.index(),
                                damage_type: DamageType::Aoe,
                            });
                        }
                    }

                    // if enemy_health.total <= 0. {
                    //     commands.entity(enemy).despawn_recursive();

                    //     spawn_explosion_at_position(
                    //         &assets,
                    //         &mut texture_atlases,
                    //         &mut commands,
                    //         &enemy_transform.translation,
                    //     );

                    //     if add_player_experience(exp.experience, &mut lvl) {
                    //         println!("Player leveled up to {}", lvl.level);
                    //         next_play_state.set(GamePlayState::LevelUp);
                    //     }
                    // } else {
                    //     spawn_damage_effect_at_position(
                    //         &assets,
                    //         &mut texture_atlases,
                    //         &mut commands,
                    //         &enemy_transform.translation,
                    //     );
                    // }
                }

                break;
            }
        }

        // Apply dama ge events
        for event in damage_events {
            for (enemy_transform, mut enemy_health, exp, enemy) in enemy_query.iter_mut() {
                if enemy.index() == event.entity_id {
                    enemy_health.total -= event.damage;

                    if enemy_health.total <= 0. {
                        if add_player_experience(exp.experience, &mut lvl) {
                            println!("Player leveled up to {}", lvl.level);
                            if player.weapons.len() < WeaponsEnum::VALUES.len() {
                                next_play_state.set(GamePlayState::LevelUp);
                            }
                        }

                        commands.entity(enemy).despawn_recursive();

                        spawn_explosion_at_position(
                            &assets,
                            &mut texture_atlases,
                            &mut commands,
                            &enemy_transform.translation,
                        );
                    } else {
                        spawn_damage_effect_at_position(
                            &assets,
                            &mut texture_atlases,
                            &mut commands,
                            &enemy_transform.translation,
                        );
                    }

                    projectile_movable.is_moving = false;
                    commands.entity(projectile_entity).despawn_recursive();
                }
            }
        }
    }
}

// Return true if lvled up
fn add_player_experience(experience: u64, lvl: &mut CanLevel) -> bool {
    lvl.experience += experience;

    if lvl.experience >= lvl.level_step {
        lvl.experience = lvl.experience - lvl.level_step;
        lvl.level += 1;

        return true;
    }

    return false;
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

    let animatable = EffectSpriteSheetAnimatable {
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
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        animatable.clone(),
        Explosion { ..default() },
    ));
}

fn spawn_damage_effect_at_position(
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
    position: &Vec3,
) {
    const PROJECTILE_HEIGHT: f32 = 100.;
    const PROJECTILE_WIDTH: f32 = 100.;

    let anim_indices = AnimationIndices { first: 0, last: 12 };

    let animatable = EffectSpriteSheetAnimatable {
        anim_indices: anim_indices.clone(),
    };

    let texture_handle = assets.load("sprites/effects/bloodspurt.png");

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT),
        6,
        6,
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
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        animatable,
        DamageEffect { ..default() },
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
            &Projectile,
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
        projectile,
        entity,
    ) in projectile_query.iter_mut()
    {
        if projectile.props.projectile_category == ProjectileCategory::ProjectileStraight
            || projectile.props.projectile_category == ProjectileCategory::TargetAoe
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

            let moving: Vec3 = normalized_translation_for_direction * selected_translation;

            projectile_transform.translation.y += moving.y;
            projectile_transform.translation.x += moving.x;
        }

        projectile_anim_timer.tick(time.delta());

        if projectile_anim_timer.just_finished() {
            projectile_sprite.index =
                if projectile_sprite.index >= projectile_movable.current_animation_indices.last {
                    projectile_movable.current_animation_indices.first
                } else {
                    projectile_sprite.index + 1
                }
        }

        if projectile_anim_timer.elapsed() > Duration::from_secs(20) {
            commands.entity(entity).despawn_recursive();
        }

        // Update position
    }
}

pub fn update_explosions_damage_effects(
    time: Res<Time>,
    mut explosions_query: Query<(
        &mut TextureAtlasSprite,
        &mut AnimationTimer,
        &EffectSpriteSheetAnimatable,
        Entity,
    )>,
    // mut damage_effects_query: Query<
    //     (
    //         &mut TextureAtlasSprite,
    //         &mut AnimationTimer,
    //         &EffectSpriteSheetAnimatable,
    //         Entity,
    //     ),
    //     With<DamageEffect>,
    // >,
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
    let (texture_atlas_handle, animatable) = get_weapon_sprite(assets, texture_atlases, weapon);

    // let normalized_direction = direction_translation.normalize();
    // Get rotation based on direction, including custom direction
    // let rotation_z = direction_translation.y.atan2(direction_translation.x);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(animatable.moving_anim_indices.first),
            transform: Transform {
                translation: Vec3 {
                    x: origin.x,
                    y: origin.y,
                    z: 9.,
                },
                scale: Vec3::new(
                    weapon.projectile_props.projectile_sprite_scale,
                    weapon.projectile_props.projectile_sprite_scale,
                    1.,
                ),
                // rotation: Quat::from_rotation_x(180.),
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
            current_animation_indices: animatable.moving_anim_indices.clone(),
            is_collided: false,
            is_state_changed: true,
        },
        Projectile {
            props: weapon.projectile_props.clone(),
        },
        DealsDamage {
            damage: 10.,
            tick_timer: Timer::from_seconds(1., TimerMode::Once),
        },
    ));
}

fn spawn_target_aoe_projectile(
    weapon: &Weapon,
    origin: Vec3,
    direction_translation: Vec3,
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    time: &Res<Time>,
) {
    let (texture_atlas_handle, animatable) = get_weapon_sprite(assets, texture_atlases, weapon);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(animatable.moving_anim_indices.first),
            transform: Transform {
                translation: Vec3 {
                    x: origin.x,
                    y: origin.y,
                    z: 9.,
                },
                scale: Vec3::new(
                    weapon.projectile_props.projectile_sprite_scale,
                    weapon.projectile_props.projectile_sprite_scale,
                    1.,
                ),
                // Get rotation from normalized direction translation
                rotation: Quat::from_rotation_z(
                    direction_translation.y.atan2(direction_translation.x)
                        - std::f32::consts::FRAC_PI_2,
                ), // ..Default::default(),
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        animatable.clone(),
        Movable {
            speed: 100.,
            direction: Direction::Custom(direction_translation),
            is_moving: true,
            current_animation_indices: animatable.moving_anim_indices.clone(),
            is_collided: false,
            is_state_changed: true,
        },
        Projectile {
            props: weapon.projectile_props.clone(),
        },
        DealsDamage {
            damage: 10.,
            tick_timer: Timer::from_seconds(1., TimerMode::Once),
        },
    ));
}

pub fn unload(
    mut projectile_query: Query<Entity, With<Projectile>>,
    mut explosion_query: Query<Entity, With<Explosion>>,
    mut damage_effect_query: Query<Entity, With<DamageEffect>>,
    // mut level_spawns: ResMut<LevelSpawns>,
    mut commands: Commands,
) {
    for entity in projectile_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in explosion_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in damage_effect_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
