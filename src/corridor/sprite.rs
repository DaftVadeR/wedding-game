use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
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
pub struct Movable {
    pub speed: f32,
    pub direction: Direction,
    pub is_moving: bool,
}

#[derive(Component, Debug)]
pub struct PlayerSpriteSheetAnimatable {
    pub idle_anim_indices: AnimationIndices,
    pub moving_horizontal_anim_indices: AnimationIndices,
    pub moving_up_anim_indices: AnimationIndices,
    pub moving_down_anim_indices: AnimationIndices,
}

#[derive(Component, Debug)]
pub struct SpriteSheetAnimatable {
    pub idle_anim_indices: AnimationIndices,
    pub moving_anim_indices: AnimationIndices,
}

#[derive(Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
