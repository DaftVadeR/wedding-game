use bevy::app::Plugin;
use bevy::prelude::*;

use crate::GameState;

mod ui;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameUiPlugin)
            // .add_plugins(LevelPlugin)
            // .add_plugins(PlayerPlugin)
            .add_systems(OnEnter(GameState::StartingGameplay), spawn_game_stuff)
            .add_systems(OnExit(GameState::StartingGameplay), despawn_game_stuff);
    }
}

fn spawn_game_stuff(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn despawn_game_stuff(mut commands: Commands, asset_server: Res<AssetServer>) {}

#[derive(Component)]
pub struct GameLevel;
