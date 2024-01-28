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
    commands.spawn(
        (SpriteBundle {
            material: asset_server.load("sprites/level/tx_tileset_grass.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        }),
    );
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
