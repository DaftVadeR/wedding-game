use bevy::app::Plugin;
use bevy::prelude::*;

use crate::corridor::level::CorridorLevelState;
use crate::corridor::player::CorridorPlayerState;

use crate::GameState;
pub struct CorridorPlugin;

mod level;
pub mod player;

use level::LevelPlugin;
use player::PlayerPlugin;

impl Plugin for CorridorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Corridor), (reset_camera, spawn_corridor))
            .add_plugins(LevelPlugin)
            .add_plugins(PlayerPlugin);
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

fn spawn_corridor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_player_state: ResMut<NextState<CorridorPlayerState>>,
    mut next_level_state: ResMut<NextState<CorridorLevelState>>,
) {
    println!("Loading corridor plugin");
    next_player_state.set(CorridorPlayerState::Init);
    next_level_state.set(CorridorLevelState::Init);
}

// fn despawn_corridor(mut commands: Commands, asset_server: Res<AssetServer>) {}
fn despawn_corridor(mut commands: Commands, entities: Query<Entity>) {
    // for entity in &entities {
    //     commands.entity(entity).despawn_recursive();
    // }
}
