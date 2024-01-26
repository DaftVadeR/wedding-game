use bevy::prelude::*;

use crate::sprite::AnimationIndices;

#[derive(Bundle)]
struct SlimeBundle {
    enemy: Slime,
}

#[derive(Bundle)]
struct MushroomBundle {
    enemy: Mushroom,
}

#[derive(Bundle)]
struct CatBundle {
    enemy: Cat,
}

#[derive(Bundle)]
struct GoblinBundle {
    enemy: Goblin,
}

#[derive(Bundle)]
struct GoblinShadowBundle {
    enemy: GoblinShadow,
}

#[derive(Debug, Component)]
pub struct Harmful {
    pub damage: f32,
}

#[derive(Debug, Component)]
pub struct Goblin;

#[derive(Debug, Component)]
pub struct Cat;

#[derive(Debug, Component)]
pub struct GoblinShadow;

#[derive(Debug, Component)]
pub struct Mushroom;

#[derive(Debug, Component)]
pub struct Slime;

pub struct GoblinSpawn;
pub struct GoblinShadowSpawn;
pub struct CatSpawn;
pub struct MushroomSpawn;
pub struct SlimeSpawn;

/*#[derive(Component, Clone, Copy)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

impl Enemy {
    fn new<T: EnemyType>() -> Bundle {}
}*/

#[derive(PartialEq, Eq)]
pub enum EnemyType {
    Goblin,
    GoblinShadow,
    Cat,
    Mushroom,
    Slime,
}

pub struct EnemyToSpawn {
    pub sprite_handle: Handle<TextureAtlas>,
    pub idle_animation_indices: AnimationIndices,
    pub run_animation_indices: AnimationIndices,
    pub translation: Vec3,
}

trait TestExtendComponent: Component {}

pub fn get_enemy_for_type(
    enemy: &EnemyType,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Box<dyn SpawnedEnemy> {
    let to_spawn: Box<dyn SpawnedEnemy> = match enemy {
        EnemyType::Goblin => Box::new(GoblinSpawn {}),
        EnemyType::GoblinShadow => Box::new(GoblinShadowSpawn {}),
        EnemyType::Cat => Box::new(CatSpawn {}),
        EnemyType::Mushroom => Box::new(GoblinSpawn {}),
        EnemyType::Slime => Box::new(GoblinSpawn {}),
    };

    to_spawn
}

trait MyComponent: Component {
    fn new_entity() -> Self;
}

struct EnemySpawn<T: MyComponent> {
    enemy_type: T,
}

impl<T: MyComponent> EnemySpawn<T> {
    fn new_entity() -> T {
        T::new_entity()
    }
}

impl<Goblin> EnemySpawn<Goblin> {}

pub trait SpawnedEnemy {
    fn new_entity() -> EnemyBundle;

    // fn get_bundle() -> Bundle;
    fn get_sprite_location(&self) -> String;
    fn get_sprite_indices(&self) -> (usize, usize);
    fn get_sprite_size(&self) -> Vec2;
    fn get_sprite_grid(&self) -> (usize, usize);
    fn get_damage(&self) -> f32;
    fn get_health(&self) -> f32;
    fn get_speed(&self) -> f32;
}

impl SpawnedEnemy for GoblinSpawn {
    fn new_entity(&self) {
        Goblin
    }

    fn get_sprite_location(&self) -> String {
        "enemy/goblin/goblin_spritesheet.png".into()
    }
    fn get_damage(&self) -> f32 {
        10.
    }
    fn get_health(&self) -> f32 {
        20.
    }
    fn get_sprite_size(&self) -> Vec2 {
        Vec2::new(16., 16.)
    }
    fn get_sprite_indices(&self) -> (usize, usize) {
        (0, 5)
    }
    fn get_sprite_grid(&self) -> (usize, usize) {
        (6, 1)
    }
    fn get_speed(&self) -> f32 {
        80.
    }
}

impl SpawnedEnemy for GoblinShadowSpawn {
    fn new_entity(&self) {
        GoblinShadow
    }

    fn get_sprite_location(&self) -> String {
        "enemy/goblin-shadow/run_spritesheet.png".into()
    }
    fn get_damage(&self) -> f32 {
        10.
    }
    fn get_health(&self) -> f32 {
        20.
    }
    fn get_sprite_size(&self) -> Vec2 {
        Vec2::new(16., 16.)
    }
    fn get_sprite_indices(&self) -> (usize, usize) {
        (0, 5)
    }
    fn get_sprite_grid(&self) -> (usize, usize) {
        (6, 1)
    }
    fn get_speed(&self) -> f32 {
        80.
    }
}

impl SpawnedEnemy for CatSpawn {
    fn new_entity() -> dyn Component {
        Cat
    }
    fn get_sprite_location(&self) -> String {
        "enemy/cat/cat-spritesheet.png".into()
    }
    fn get_damage(&self) -> f32 {
        10.
    }
    fn get_health(&self) -> f32 {
        20.
    }
    fn get_sprite_size(&self) -> Vec2 {
        Vec2::new(16., 16.)
    }
    fn get_sprite_indices(&self) -> (usize, usize) {
        (0, 5)
    }
    fn get_sprite_grid(&self) -> (usize, usize) {
        (6, 1)
    }
    fn get_speed(&self) -> f32 {
        80.
    }
}
