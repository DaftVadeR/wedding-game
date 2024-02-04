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
    pub current_animation_indices: AnimationIndices,
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

#[derive(Component, Debug)]
pub struct EnemySpriteSheetAnimatable {
    pub idle_anim_indices: AnimationIndices,
    pub moving_anim_indices: AnimationIndices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Debug)]
pub struct Health(pub f32);
