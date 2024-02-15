use core::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::game::level::{MAP_HEIGHT, MAP_WIDTH};
use crate::game::weapons::{Explosion, WeaponsEnum};
use crate::sprite::{
    get_translation_for_direction, AnimationIndices, AnimationTimer, Direction,
    EffectSpriteSheetAnimatable, Health, Movable, ProjectileDealsDamage,
    ProjectileSpriteSheetAnimatable,
};
use crate::GameState;

use super::player::{CanLevel, Player};
use super::spawner::{Enemy, GivesExperience};
use super::weapons::{
    get_weapon_sprite, DamageEffect, Projectile, ProjectileAimMethod, ProjectileCategory,
    ProjectileProps, Weapon,
};
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

fn get_closest_enemy(
    enemy_query: &Query<(&Transform, Entity), (With<Enemy>, Without<Player>)>,
    player_transform: &Transform,
    projectile: &ProjectileProps,
) -> Option<(Vec3, f32)> {
    let mut closest: Option<(Vec3, f32)> = None;

    for (enemy_transform, enemy) in enemy_query.iter() {
        let distance = enemy_transform
            .translation
            .distance(player_transform.translation);

        if closest.is_none() || (closest.is_some() && distance < closest.unwrap().1) {
            closest = Some((enemy_transform.translation, distance));
        }
    }

    // Check if close enough. Don't want enemies off screen being hit
    if closest.is_some() {
        let max_distance_from_player: f32 = projectile.projectile_aim_range;

        let distance = closest.unwrap().0.distance(player_transform.translation);

        if distance > max_distance_from_player {
            return None;
        }
    }

    closest
}

fn get_random_enemy_position(
    rng: &mut rand::prelude::ThreadRng,
    enemy_query: &Query<(&Transform, Entity), (With<Enemy>, Without<Player>)>,
    player_transform: &Transform,
    projectile: &ProjectileProps,
) -> Option<Vec3> {
    let max_distance_from_player: f32 = projectile.projectile_aim_range;

    let mut enemies_in_distance: Vec<(u32, Vec3)> = vec![];

    for (enemy_transform, entity) in enemy_query.iter() {
        let distance = enemy_transform
            .translation
            .distance(player_transform.translation);

        if distance < max_distance_from_player {
            enemies_in_distance.push((entity.index(), enemy_transform.translation));
        }
    }

    if enemies_in_distance.len() == 0 {
        return None;
    }

    let random_index = rng.gen_range(0..enemies_in_distance.len());

    Some(enemies_in_distance[random_index].1)

    // for eb_query in 0..(enemies_in_distance.len() / 2) {

    //     random_enemy = match enemy_query.iter().nth(random_index) {
    //         Some((transform, _)) => Some(transform.translation.clone()),
    //         None => {
    //             continue;
    //         }
    //     }
    // }

    // random_enemy
}

fn get_random_nearby_position(
    rng: &mut rand::prelude::ThreadRng,
    player_transform: &Transform,
    projectile: &ProjectileProps,
) -> Vec3 {
    let rnd_x: f32 = rng.gen_range(0. ..projectile.projectile_aim_range);
    let rnd_y: f32 = rng.gen_range(0. ..(projectile.projectile_aim_range * 0.75));

    // negative and positive x+y axis.
    let x_pos = player_transform.translation.x
        + (if rng.gen_bool(0.5) {
            rnd_x * -1.
        } else {
            rnd_x
        });
    let y_pos = player_transform.translation.y
        + (if rng.gen_bool(0.5) {
            rnd_y * -1.
        } else {
            rnd_y
        });

    let final_x_pos = x_pos.clamp(-1. * MAP_WIDTH / 2., MAP_WIDTH / 2.);
    let final_y_pos = y_pos.clamp(-1. * MAP_HEIGHT / 2., MAP_HEIGHT / 2.);

    // let random_x_initial: usize = rng.gen_range(0..400);
    // let random_y_initial: usize = rng.gen_range(0..300);

    // let is_negative_x = rng.gen_bool(0.5);
    // let is_negative_y = rng.gen_bool(0.5);

    // let modifier_x = if is_negative_x { -1. } else { 1. };
    // let modifier_y = if is_negative_y { -1. } else { 1. };

    // let random_x = player_transform.translation.x - (modifier_x * random_x_initial as f32);
    // let random_y = player_transform.translation.y - (modifier_y * random_y_initial as f32);

    let origin = Vec3::new(final_x_pos, final_y_pos, 9.); // 9 due to projectile assumption

    origin
}

fn spawn_projectile_for_aim_method(
    weapon: &Weapon,
    player_transform: &Transform,
    player_movable: &Movable,
    enemy_query: &Query<(&Transform, Entity), (With<Enemy>, Without<Player>)>,
    rng: &mut rand::prelude::ThreadRng,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    // Default to player facing direction - ProjectileAimMethod::PlayerFacing
    let mut origin: Vec3 = player_transform.translation.clone();
    let mut direction: Vec3 =
        get_translation_for_direction(player_movable.direction, player_transform.translation.z);

    // Only fire when tick timer finished
    match weapon.projectile_props.projectile_aim_method {
        ProjectileAimMethod::RandomEnemy => {
            let random_enemy: Option<Vec3> = get_random_enemy_position(
                rng,
                enemy_query,
                player_transform,
                &weapon.projectile_props,
            );

            direction = get_translation_for_direction(Direction::Custom(Vec3::new(0., 1., 0.)), 9.);

            match random_enemy {
                Some(vec) => {
                    origin = vec;
                }
                None => {
                    origin =
                        get_random_nearby_position(rng, player_transform, &weapon.projectile_props);
                }
            };
        }
        ProjectileAimMethod::NearestEnemy => {
            // Uses the default origin transform but alters the direction to point at nearest enemy

            // Alter projectile transform using normalized translation of enemy to player times speed
            println!("Firing projectile aoe");
            // Get closest enemy translation
            let closest =
                get_closest_enemy(enemy_query, player_transform, &weapon.projectile_props);

            match closest {
                Some((vec, _)) => {
                    let normalized_translation =
                        Vec3::normalize(vec - player_transform.translation);

                    direction = get_translation_for_direction(
                        Direction::Custom(normalized_translation),
                        9.,
                    );
                }
                None => {
                    println!("No closest enemy")
                }
            };
        }
        ProjectileAimMethod::Random => {
            // Uses the default direction but alters the origin transform
            origin = get_random_nearby_position(rng, player_transform, &weapon.projectile_props);
            direction = get_translation_for_direction(Direction::Custom(Vec3::new(0., 1., 0.)), 9.);
        }
        // Otherwise, use defaults for both transform (player origin) and direction (player direction)
        _ => {}
    }

    spawn_sprite(
        weapon,
        direction,
        origin,
        texture_atlases,
        &asset_server,
        commands,
    );
}

fn spawn_static_projectile(
    weapon: &Weapon,
    player_transform: &Transform,
    player_movable: &Movable,
    enemy_query: &Query<(&Transform, Entity), (With<Enemy>, Without<Player>)>,
    rng: &mut rand::prelude::ThreadRng,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    // Default to player facing direction - ProjectileAimMethod::PlayerFacing
    let mut origin: Vec3 = player_transform.translation.clone();
    let mut direction: Vec3 =
        get_translation_for_direction(player_movable.direction, player_transform.translation.z);

    // Only fire when tick timer finished
    match weapon.projectile_props.projectile_aim_method {
        ProjectileAimMethod::NearestEnemy => {
            // Uses the default origin transform but alters the direction to point at nearest enemy

            // Alter projectile transform using normalized translation of enemy to player times speed
            println!("Firing projectile aoe");
            // Get closest enemy translation
            let closest =
                get_closest_enemy(enemy_query, player_transform, &weapon.projectile_props);

            match closest {
                Some((vec, _)) => {
                    let normalized_translation =
                        Vec3::normalize(vec - player_transform.translation);

                    direction = get_translation_for_direction(
                        Direction::Custom(normalized_translation),
                        player_transform.translation.z,
                    );
                }
                None => {
                    println!("No closest enemy")
                }
            };
        }
        ProjectileAimMethod::Random => {
            // Uses the default direction but alters the origin transform
            let random_x: usize = rng.gen_range(0..900);
            let random_y: usize = rng.gen_range(0..900);

            let is_negative_x = rng.gen_bool(0.5);
            let is_negative_y = rng.gen_bool(0.5);

            let modifier_x = if is_negative_x { -1. } else { 1. };
            let modifier_y = if is_negative_y { -1. } else { 1. };

            origin = Vec3::new(
                modifier_x * random_x as f32,
                modifier_y * random_y as f32,
                9.,
            );
        }
        // Otherwise, use defaults for both transform (player origin) and direction (player direction)
        _ => {}
    }

    spawn_sprite(
        weapon,
        direction,
        origin,
        texture_atlases,
        &asset_server,
        commands,
    );
}

fn spawn_weapon_projectiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    enemy_query: Query<(&Transform, Entity), (With<Enemy>, Without<Player>)>,
    mut player_weapon_query: Query<
        (&mut Player, &Transform, &Movable),
        (Without<Enemy>, With<Player>),
    >,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (mut player, player_transform, movable) in player_weapon_query.iter_mut() {
        for weapon in player.weapons.iter_mut() {
            weapon.tick_timer.tick(time.delta());

            if !weapon.tick_timer.finished() {
                continue;
            }

            match weapon.projectile_props.projectile_category {
                ProjectileCategory::Projectile
                | ProjectileCategory::ProjectileAoe
                | ProjectileCategory::Instant
                | ProjectileCategory::InstantAoe => {
                    println!("Firing projectile towards enemy");
                    spawn_projectile_for_aim_method(
                        weapon,
                        &player_transform,
                        &movable,
                        &enemy_query,
                        &mut rng,
                        &asset_server,
                        &mut texture_atlases,
                        &mut commands,
                    );
                }
                //
                // ProjectileCategory::Instant => {
                //     println!("Firing projectile towards enemy");
                //     spawn_projectile_for_aim_method(
                //         weapon,
                //         &player_transform,
                //         &movable,
                //         &enemy_query,
                //         &mut rng,
                //         &asset_server,
                //         &mut texture_atlases,
                //         &mut commands,
                //     );
                // }
                _ => {
                    println!("Fire alternate type");
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageType {
    #[default]
    Normal,
    Fire,
    Water,
    Earth,
    Lightning,
    Psychological,
}

fn update_projectile_collisions(
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut enemy_query: Query<
        (&Transform, &mut Health, &GivesExperience, &Enemy, Entity),
        (With<Enemy>, Without<Player>),
    >,
    mut projectile_query: Query<
        (
            &Transform,
            &mut Movable,
            &mut ProjectileDealsDamage,
            &Projectile,
            Entity,
        ),
        (With<Projectile>, Without<Player>),
    >,
    mut player_query: Query<(&mut CanLevel, &Player), With<Player>>,
    mut next_play_state: ResMut<NextState<GamePlayState>>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let (mut lvl, player) = player_query.single_mut();

    for (
        projectile_transform,
        mut projectile_movable,
        mut projectile_damage,
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

        if projectile_damage.is_triggered {
            continue;
        }

        let mut damage_events: Vec<DamageEvent> = Vec::new();

        for (enemy_transform, _, _, _, enemy) in enemy_query.iter() {
            let distance = enemy_transform
                .translation
                .distance(projectile_transform.translation);

            // IF normal projectile, no checking for aoe - just kill initial collided enemy and remove projectile entity on FIRST enemy hit.

            // If aoe projectile, add distance from aoe radius to collision distance. Isn't simplified to be on all projectiles as fundamentally different in that the normal projectile goes on FIRST hit,
            // but doesn't immediately disappear after the first collision - only after the loop has been finished do we despawn it.

            let collision_distance = (projectile.props.projectile_sprite_width
                * projectile.props.projectile_sprite_scale)
                / 2.;

            // Collided
            if distance < collision_distance {
                if projectile.props.projectile_category == ProjectileCategory::ProjectileAoe
                    || projectile.props.projectile_category == ProjectileCategory::InstantAoe
                {
                    // Collided with an enemy. Now trigger damage to everything in area.
                    let damage_radius = projectile.props.projectile_aoe_radius + collision_distance;

                    for (enemy_transform, _, _, _, enemy) in enemy_query.iter() {
                        let aoe_distance = projectile_transform
                            .translation
                            .distance(enemy_transform.translation);

                        // IF normal projectile, no checking for aoe - just kill initial collided enemy and remove projectile entity on FIRST enemy hit.

                        // If aoe projectile, add distance from aoe radius to collision distance. Isn't simplified to be on all projectiles as fundamentally different in that the normal projectile goes on FIRST hit,
                        // but doesn't immediately disappear after the first collision - only after the loop has been finished do we despawn it.

                        if projectile.props.projectile_aoe_radius > 0.
                            && aoe_distance < projectile.props.projectile_aoe_radius
                        {
                            println!("Collided with enemy due to {} distance from projectile being less than projectile_aoe_radius", aoe_distance);
                            damage_events.push(DamageEvent {
                                damage: projectile_damage.damage
                                    * projectile.props.projectile_aoe_damage_scale,
                                entity_id: enemy.index(),
                                damage_type: projectile.props.projectile_damage_type,
                            });
                        }
                    }
                } else {
                    damage_events.push(DamageEvent {
                        damage: projectile_damage.damage,
                        entity_id: enemy.index(),
                        damage_type: projectile.props.projectile_damage_type,
                    });
                }

                break;
            }
        }

        // Apply dama ge events
        for event in damage_events.iter() {
            for (enemy_transform, mut enemy_health, exp, enemy, entity) in enemy_query.iter_mut() {
                if entity.index() == event.entity_id {
                    enemy_health.total -= event.damage;
                    println!("applying dmg event {} {}", event.damage, enemy_health.total);

                    if enemy_health.total <= 0. {
                        println!("Enemy died {} {}", entity.index(), enemy_health.total);
                        if add_player_experience(exp.experience, &mut lvl) {
                            println!("Player leveled up to {}", lvl.level);
                            if player.weapons.len() < WeaponsEnum::VALUES.len() {
                                next_play_state.set(GamePlayState::LevelUp);
                            }
                        }

                        if enemy.is_boss {
                            next_state.set(GameState::GameWon);
                            next_play_state.set(GamePlayState::Unloaded);
                            return;
                        }

                        commands.entity(entity).despawn_recursive();

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
                }
            }
        }

        if damage_events.len() > 0
            && projectile.props.projectile_category != ProjectileCategory::Instant
            && projectile.props.projectile_category != ProjectileCategory::InstantAoe
        {
            projectile_movable.is_moving = false;
            projectile_damage.is_triggered = true;
            commands.entity(projectile_entity).despawn_recursive();
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
        if projectile.props.projectile_aim_method == ProjectileAimMethod::PlayerDirection
            || projectile.props.projectile_aim_method == ProjectileAimMethod::NearestEnemy
        {
            if projectile.props.projectile_category == ProjectileCategory::Projectile
                || projectile.props.projectile_category == ProjectileCategory::ProjectileAoe
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
        } else {
            // follow enemy - true homing
            // TODO if not too complicated
            // add enemy instance to projectile - use transform on entity to update projectile position so it homes in
            // Delete enemy/transform from projectile if enemy despawned and add new one - might be easy way to do this in bevy.
        }

        projectile_anim_timer.tick(time.delta());

        if projectile_anim_timer.just_finished() {
            projectile_sprite.index =
                if projectile_sprite.index >= projectile_movable.current_animation_indices.last {
                    if projectile.props.projectile_category == ProjectileCategory::Instant
                        || projectile.props.projectile_category == ProjectileCategory::InstantAoe
                    {
                        // despawn after animation finished - only fir instant weapons
                        commands.entity(entity).despawn_recursive();
                    }
                    projectile_movable.current_animation_indices.first
                } else {
                    projectile_sprite.index + 1
                };
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

// fn spawn_simple_straight_projectile(
//     weapon: &Weapon,
//     origin: Vec3,
//     direction_translation: Vec3,
//     commands: &mut Commands,
//     assets: &Res<AssetServer>,
//     texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
//     time: &Res<Time>,
// ) {
//     let (texture_atlas_handle, animatable) = get_weapon_sprite(assets, texture_atlases, weapon);

//     // let normalized_direction = direction_translation.normalize();
//     // Get rotation based on direction, including custom direction
//     // let rotation_z = direction_translation.y.atan2(direction_translation.x);

//     commands.spawn((
//         SpriteSheetBundle {
//             texture_atlas: texture_atlas_handle.clone(),
//             sprite: TextureAtlasSprite::new(animatable.moving_anim_indices.first),
//             transform: Transform {
//                 translation: Vec3 {
//                     x: origin.x,
//                     y: origin.y,
//                     z: 9.,
//                 },
//                 scale: Vec3::new(
//                     weapon.projectile_props.projectile_sprite_scale,
//                     weapon.projectile_props.projectile_sprite_scale,
//                     1.,
//                 ),
//                 // rotation: Quat::from_rotation_x(180.),
//                 ..Default::default()
//             },
//             ..default()
//         },
//         AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
//         animatable.clone(),
//         Movable {
//             speed: 100.,
//             direction: Direction::Custom(direction_translation),
//             is_moving: true,
//             current_animation_indices: animatable.moving_anim_indices.clone(),
//             is_collided: false,
//             is_state_changed: true,
//         },
//         Projectile {
//             props: weapon.projectile_props.clone(),
//         },
//         DealsDamage {
//             damage: 10.,
//             tick_timer: Timer::from_seconds(1., TimerMode::Once),
//         },
//     ));
// }

pub fn get_rotation_from_direction(direction: Vec3, offset: f32) -> Quat {
    let rotation = direction.y.atan2(direction.x) - offset; // if facing right - 0. If facing up - 90 degrees, or std::f32::consts::FRAC_PI_2.

    Quat::from_rotation_z(rotation)
}

// fn spawn_homing_projectile(
//     weapon: &Weapon,
//     origin: Vec3,
//     direction_translation: Vec3,
//     commands: &mut Commands,
//     assets: &Res<AssetServer>,
//     texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
//     time: &Res<Time>,
// ) {
//     spawn_sprite(
//         &texture_atlas_handle,
//         &animatable,
//         &origin,
//         &rotation_quat,
//         &weapon,
//         commands,
//     );
// }

fn spawn_sprite(
    weapon: &Weapon,
    direction_translation: Vec3,
    origin: Vec3,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    assets: &Res<AssetServer>,
    commands: &mut Commands,
) {
    let (texture_atlas_handle, animatable) = get_weapon_sprite(assets, texture_atlases, weapon);

    let rotation_quat = get_rotation_from_direction(
        direction_translation,
        weapon.projectile_props.projectile_rotation_offset,
    );

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: animatable.moving_anim_indices.first,
                anchor: weapon.projectile_props.projectile_sprite_anchor,
                ..default()
            },
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

                rotation: rotation_quat,
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
        ProjectileDealsDamage {
            damage: weapon.projectile_props.projectile_base_damage,
            is_triggered: false,
        },
    ));

    // commands.spawn((
    //     SpriteSheetBundle {
    //         texture_atlas: texture_atlas_handle.clone(),
    //         sprite: TextureAtlasSprite::new(animatable.anim_indices.first),
    //         transform: Transform {
    //             translation: Vec3 {
    //                 x: origin.x,
    //                 y: origin.y,
    //                 z: 9.,
    //             },
    //             scale: Vec3::new(*scale, *scale, 1.),
    //             rotation: rotation.clone(),
    //         },
    //         ..default()
    //     },
    //     AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    //     animatable.clone(),
    //     Movable {
    //         speed: 100.,
    //         direction: Direction::Custom(Vec3::new(0., 0., 0.)),
    //         is_moving: true,
    //         current_animation_indices: animatable.anim_indices.clone(),
    //         is_collided: false,
    //         is_state_changed: true,
    //     },
    //     Projectile {
    //         props: Projectile::default(),
    //     },
    //     DealsDamage {
    //         damage: 10.,
    //         tick_timer: Timer::from_seconds(1., TimerMode::Once),
    //     },
    // ));
}

// fn spawn_target_aoe_projectile(
//     weapon: &Weapon,
//     origin: Vec3,
//     direction_translation: Vec3,
//     commands: &mut Commands,
//     assets: &Res<AssetServer>,
//     texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
//     time: &Res<Time>,
// ) {
//     let (texture_atlas_handle, animatable) = get_weapon_sprite(assets, texture_atlases, weapon);

//     let rotation = direction_translation.y.atan2(direction_translation.x)
//         - weapon.projectile_props.projectile_rotation_offset; // std::f32::consts::FRAC_PI_2;

//     println!("Rotation: {}", rotation);
//     println!(
//         "Rotation from_rotation: {}",
//         Quat::from_rotation_z(rotation)
//     );

//     commands.spawn((
//         SpriteSheetBundle {
//             texture_atlas: texture_atlas_handle.clone(),
//             sprite: TextureAtlasSprite::new(animatable.moving_anim_indices.first),
//             transform: Transform {
//                 translation: Vec3 {
//                     x: origin.x,
//                     y: origin.y,
//                     z: 9.,
//                 },
//                 scale: Vec3::new(
//                     weapon.projectile_props.projectile_sprite_scale,
//                     weapon.projectile_props.projectile_sprite_scale,
//                     1.,
//                 ),
//                 // Get rotation from normalized direction translation
//                 rotation: Quat::from_rotation_z(rotation), // ..Default::default(),
//             },
//             ..default()
//         },
//         AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
//         animatable.clone(),
//         Movable {
//             speed: 100.,
//             direction: Direction::Custom(direction_translation),
//             is_moving: true,
//             current_animation_indices: animatable.moving_anim_indices.clone(),
//             is_collided: false,
//             is_state_changed: true,
//         },
//         Projectile {
//             props: weapon.projectile_props.clone(),
//         },
//         DealsDamage {
//             damage: 10.,
//             tick_timer: Timer::from_seconds(1., TimerMode::Once),
//         },
//     ));
// }

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
