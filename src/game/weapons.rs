use bevy::prelude::*;

use crate::sprite::{Weapon, WeaponType};

pub fn get_guitar_weapon() -> Weapon {
    Weapon {
        name: "Guitar of death".into(),
        tick_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
        weapon_type: WeaponType::ProjectileStraight,
        pic_sprite: "sprites/weapons/guitar_pixelated_small.png",
        scale: 0.4,
    }
}
