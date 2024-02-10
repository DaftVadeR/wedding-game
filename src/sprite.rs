use bevy::prelude::*;

use crate::game::weapons::WeaponsEnum;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Custom(Vec3),
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Component, Debug)]
pub struct DealsDamage {
    pub damage: f32,
    pub tick_timer: Timer,
}

#[derive(Component, Debug)]
pub struct Movable {
    pub speed: f32,
    pub direction: Direction,
    pub is_moving: bool,
    pub current_animation_indices: AnimationIndices,
    pub is_collided: bool,
    pub is_state_changed: bool,
}

#[derive(Component, Debug)]
pub struct PlayerSpriteSheetAnimatable {
    pub idle_anim_indices: AnimationIndices,
    pub moving_horizontal_anim_indices: AnimationIndices,
    pub moving_up_anim_indices: AnimationIndices,
    pub moving_down_anim_indices: AnimationIndices,
    pub moving_down_horiz_anim_indices: AnimationIndices,
    pub moving_up_horiz_anim_indices: AnimationIndices,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnemySpriteSheetAnimatable {
    pub idle_anim_indices: AnimationIndices,
    pub moving_anim_indices: AnimationIndices,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectileSpriteSheetAnimatable {
    pub moving_anim_indices: AnimationIndices,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExplosionSpriteSheetAnimatable {
    pub anim_indices: AnimationIndices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Debug)]
pub struct Health {
    pub total: f32,
}

pub fn get_translation_for_direction(direction: Direction, default_z: f32) -> Vec3 {
    match direction {
        Direction::Custom(vec) => Vec3::new(vec.x, vec.y, default_z),
        Direction::Left => Vec3::new(-1., 0., default_z),
        Direction::Right => Vec3::new(1., 0., default_z),
        Direction::Up => Vec3::new(0., 1., default_z),
        Direction::Down => Vec3::new(0., -1., default_z),
        Direction::UpLeft => Vec3::new(-1., 1., default_z),
        Direction::UpRight => Vec3::new(1., 1., default_z),
        Direction::DownLeft => Vec3::new(-1., -1., default_z),
        Direction::DownRight => Vec3::new(1., -1., default_z),
    }
}

// pub fn get_rotation_for_direction(direction: Direction, default_z: f32) ->  {
//     let custom_direction_translation = get_translation_for_direction(direction, default_z);

// }
