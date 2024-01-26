use bevy::app::Plugin;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

use crate::GameState;

pub struct CorridorPlugin;

// mod level;
mod player;
mod sprite;

// use level::LevelPlugin;
use player::PlayerPlugin;

impl Plugin for CorridorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
        app.add_systems(
            OnEnter(GameState::StartingCorridor),
            (reset_camera, spawn_corridor),
        )
        .add_systems(OnExit(GameState::StartingCorridor), despawn_corridor);
        // .add_plugins(LevelPlugin);
    }
}

pub fn reset_camera(
    // query: Query<(&Transform) /*(With<Player>)*/>,
    mut camera_query: Query<&mut Transform, With<Camera> /*, Without<Player>*/>,
) {
    // let player_transform = query.single();
    let mut camera_transform = camera_query.single_mut();

    //
    camera_transform.translation = Vec3::new(0., 0., 0.);
}

fn spawn_corridor(mut commands: Commands, asset_server: Res<AssetServer>) {}

// fn despawn_corridor(mut commands: Commands, asset_server: Res<AssetServer>) {}
fn despawn_corridor(mut commands: Commands, entities: Query<Entity>) {
    // for entity in &entities {
    //     commands.entity(entity).despawn_recursive();
    // }
}

#[derive(Component)]
pub struct CorridorUI;
