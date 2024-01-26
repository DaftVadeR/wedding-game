use bevy::app::{AppExit, Plugin};
use bevy::prelude::*;

use crate::GameState;

pub struct GameWonPlugin;

impl Plugin for GameWonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameWon), spawn_game_won_scene)
            .add_systems(OnExit(GameState::GameWon), despawn_game_won_scene);
    }
}

fn spawn_game_won_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(Camera2dBundle {
    //     transform: Transform::from_xyz(0., 0., 0.),
    //     projection: OrthographicProjection {
    //         far: 1000.,
    //         near: -1000.,
    //         scale: 0.5,
    //         ..default()
    //     },
    //     ..default()
    // });
    // commands.spawn()gcc=
}

fn despawn_game_won_scene(
    mut exit: EventWriter<AppExit>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    exit.send(AppExit);
}

#[derive(Component)]
pub struct NpcComponent;
