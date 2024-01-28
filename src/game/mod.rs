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
            .add_systems(
                OnEnter(GameState::Gameplay),
                (reset_camera, spawn_game_stuff),
            )
            .add_systems(OnExit(GameState::Gameplay), despawn_game_stuff);
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

pub fn reset_camera(
    // query: Query<(&Transform) /*(With<Player>)*/>,
    mut camera_query: Query<&mut Transform, With<Camera> /*, Without<Player>*/>,
) {
    // let player_transform = query.single();
    let mut camera_transform = camera_query.single_mut();

    //
    camera_transform.translation = Vec3::new(0., 0., 0.);
}
