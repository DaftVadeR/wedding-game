use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::physics::ColliderBundle;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub enum AnimationType {
    RunUp,
    RunDown,
    RunLeft,
    RunRight,
    #[default]
    Idle,
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Default)]
pub struct SpriteAnimation {
    pub current_animation_type: AnimationType,
    pub current_frame_index: usize,
    pub animation_map: HashMap<AnimationType, [usize; 2]>,
    pub flip_animation: Vec<AnimationType>,
    pub timer: Timer,
}

impl SpriteAnimation {
    pub fn get_current_facing_slice(&self) -> Option<&[usize; 2]> {
        self.animation_map.get(&self.current_animation_type)
    }

    pub fn is_flipped(&self) -> bool {
        self.flip_animation.contains(&self.current_animation_type)
    }
}

#[derive(Default, Bundle)]
pub struct SpriteAnimationBundle {
    pub sprite_animation: SpriteAnimation,
    pub timer: AnimationTimer,
}

impl From<EntityInstance> for SpriteAnimationBundle {
    fn from(value: EntityInstance) -> Self {
        match value.identifier.as_ref() {
            "Player" => {
                let mut animation_map: HashMap<AnimationType, [usize; 2]> = HashMap::new();
                animation_map.insert(AnimationType::RunUp, [12, 4]);
                animation_map.insert(AnimationType::RunDown, [16, 6]);
                animation_map.insert(AnimationType::RunLeft, [4, 4]);
                animation_map.insert(AnimationType::RunRight, [4, 4]);
                animation_map.insert(AnimationType::Idle, [0, 1]);
                SpriteAnimationBundle {
                    sprite_animation: SpriteAnimation {
                        animation_map,
                        flip_animation: vec![AnimationType::RunRight],
                        ..default()
                    },
                    timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                }
            }
            _ => SpriteAnimationBundle::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,

    #[from_entity_instance]
    #[bundle]
    pub sprite_animation_bundle: SpriteAnimationBundle,

    #[sprite_sheet_bundle("player_sprite_sheet.png", 24., 24., 44, 1, 0., 0., 0)]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}
