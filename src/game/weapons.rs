use crate::sprite::{AnimationIndices, ProjectileSpriteSheetAnimatable};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponsEnum {
    #[default]
    Guitar,
    Horse,
}

impl WeaponsEnum {
    pub const VALUES: [WeaponsEnum; 2] = [WeaponsEnum::Guitar, WeaponsEnum::Horse];
}

#[derive(PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum ProjectileState {
    #[default]
    Dispatched,
}

#[derive(Component)]
pub struct Projectile {
    pub props: ProjectileProps,
}

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum ExplosionType {
    #[default]
    Simple,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Explosion {
    pub explosion_type: ExplosionType,
}

#[derive(Component, Default, Debug, Clone)]
pub struct DamageEffect {
    pub explosion_type: ExplosionType,
}

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum ProjectileCategory {
    #[default]
    ProjectileStraight,
    ProjectileHoming,
    SelfAoe,
    TargetAoe,
}

#[derive(Debug, Clone)]
pub struct ProjectileProps {
    pub projectile_category: ProjectileCategory,
    pub projectile_sprite: &'static str,
    pub projectile_sprite_indices: AnimationIndices,
    pub projectile_sprite_scale: f32,
    pub projectile_sprite_height: f32,
    pub projectile_sprite_width: f32,
    pub projectile_sprite_rows: usize,
    pub projectile_sprite_cols: usize,
    pub projectile_aoe_radius: f32,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub desc: String,
    pub variant: WeaponsEnum,
    pub tick_timer: Timer,
    pub projectile_props: ProjectileProps,
}

pub fn get_available_weapons(player_weapons: &Vec<Weapon>, num_weapons: usize) -> Vec<Weapon> {
    let mut new_weapons: Vec<Weapon> = vec![];
    let mut rng = rand::thread_rng();

    let player_weapon_types = get_collated_weapons_for_player(&player_weapons);
    let mut available_weapon_types = get_filtered_weapons(&player_weapon_types);

    if player_weapon_types.len() >= WeaponsEnum::VALUES.len() || available_weapon_types.len() == 0 {
        return new_weapons;
    }

    // Get random weapon and add to array
    for i in 0..num_weapons {
        if available_weapon_types.len() == 0 {
            break;
        }

        let random_index = rng.gen_range(0..player_weapon_types.len());
        let selected = available_weapon_types.get(random_index).expect("Can't get weapon type at index for some reason - possibly due to available_weapon_types mutation issue.");
        new_weapons.push(get_weapon_for_type(selected));

        available_weapon_types.remove(random_index);
    }

    new_weapons
}

fn get_collated_weapons_for_player(weapons: &Vec<Weapon>) -> Vec<WeaponsEnum> {
    let mut collated_weapons: Vec<WeaponsEnum> = Vec::new();

    for weapon in weapons {
        collated_weapons.push(weapon.variant);
    }

    collated_weapons
}

fn get_filtered_weapons(weapons: &Vec<WeaponsEnum>) -> Vec<WeaponsEnum> {
    let mut filtered_weapons: Vec<WeaponsEnum> = Vec::new();

    for val in WeaponsEnum::VALUES {
        let mut found = false;

        for weapon in weapons {
            if *weapon == val {
                found = true;
                break;
            }
        }

        if !found {
            filtered_weapons.push(val);
        }
    }

    filtered_weapons
}

// PULLS THE OWNERSHIP OF THE WEAPON FROM THE PASSED IN VECTOR - KEEP IN MIND
fn get_weapon_by_variant(variant: &WeaponsEnum, weapons: &Vec<Weapon>) -> Option<Weapon> {
    for weapon in weapons {
        if weapon.variant == *variant {
            return Some(weapon.clone());
        }
    }
    None
}

// // PULLS THE OWNERSHIP OF THE WEAPON FROM THE PASSED IN VECTOR - KEEP IN MIND
// fn get_player_weapon_by_variant(variant: &WeaponsEnum, weapons: &Vec<Weapon>) -> Option<Weapon> {
//     for weapon in weapons {
//         if weapon.variant == *variant {
//             return Some(weapon);
//         }
//     }
//     None
// }

fn get_random_weapon(rng: &mut ThreadRng) -> WeaponsEnum {
    let random_index = rng.gen_range(0..WeaponsEnum::VALUES.len());
    WeaponsEnum::VALUES[random_index]
}

fn get_weapon_for_type(weapon_type: &WeaponsEnum) -> Weapon {
    match weapon_type {
        WeaponsEnum::Guitar => get_guitar_weapon(),
        WeaponsEnum::Horse => get_horse_weapon(),
    }
}

fn get_vector_with_weapons_for_types(types: Vec<WeaponsEnum>) -> Vec<Weapon> {
    let mut weapons: Vec<Weapon> = Vec::new();

    for weapon_type in types {
        weapons.push(get_weapon_for_type(&weapon_type));
    }

    weapons
}

// pub fn get_weapons() -> HashMap<WeaponsEnum, Weapon> {
//     let mut weapons = HashMap::default();

// }

pub fn get_guitar_weapon() -> Weapon {
    Weapon {
        name: "Guitar of death".into(),
        desc: "It hits hard. But throwing guitars takes a while...".into(),
        tick_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
        variant: WeaponsEnum::Guitar,
        projectile_props: ProjectileProps {
            // Pass to projectile for duration of lifetime
            projectile_sprite: "sprites/weapons/guitar_pixelated_small.png",
            projectile_sprite_scale: 0.4,
            projectile_category: ProjectileCategory::ProjectileStraight,
            projectile_sprite_indices: AnimationIndices { first: 0, last: 2 },
            projectile_sprite_height: 64.,
            projectile_sprite_width: 64.,
            projectile_sprite_rows: 1,
            projectile_sprite_cols: 3,
            projectile_aoe_radius: 0.,
        },
    }
}

pub fn get_horse_weapon() -> Weapon {
    Weapon {
        name: "Flatulent Horses".into(),
        desc: "Devastating area of attack ability. \"Never underestimate horses.\" - Lisa".into(),
        tick_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
        variant: WeaponsEnum::Horse,
        projectile_props: ProjectileProps {
            projectile_sprite: "sprites/weapons/horse.png",
            projectile_sprite_scale: 0.4,
            projectile_category: ProjectileCategory::TargetAoe,
            projectile_sprite_indices: AnimationIndices { first: 9, last: 11 },
            projectile_sprite_height: 96.,
            projectile_sprite_width: 96.,
            projectile_sprite_rows: 4,
            projectile_sprite_cols: 3,
            projectile_aoe_radius: 80.,
        },
    }
}

pub fn get_weapon_sprite(
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    weapon: &Weapon,
) -> (Handle<TextureAtlas>, ProjectileSpriteSheetAnimatable) {
    let animatable: ProjectileSpriteSheetAnimatable = ProjectileSpriteSheetAnimatable {
        moving_anim_indices: weapon.projectile_props.projectile_sprite_indices.clone(),
    };

    let texture_handle = assets.load(weapon.projectile_props.projectile_sprite);

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(
            weapon.projectile_props.projectile_sprite_width,
            weapon.projectile_props.projectile_sprite_height,
        ),
        weapon.projectile_props.projectile_sprite_cols,
        weapon.projectile_props.projectile_sprite_rows,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    (texture_atlas_handle, animatable)
}
