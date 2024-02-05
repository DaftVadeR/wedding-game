use bevy::app::Plugin;
use bevy::audio::{PlaybackMode, Volume, VolumeLevel};
use bevy::prelude::*;

use crate::GameState;

use self::level::LevelPlugin;
use self::player::PlayerPlugin;

use self::spawner::EnemySpawnerPlugin;
use self::ui::GameUiPlugin;

mod level;
mod player;
// mod potato_anim;
// mod potato_enemy;
mod spawner;
// mod potato_spawner;
mod ui;

pub struct GameplayPlugin;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum GamePlayState {
    #[default]
    Unloaded,
    Init,
    Started,
    LevelUp,
    GameOver,
}

#[derive(Component)]
pub struct MyMusic;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameUiPlugin)
            .add_plugins(LevelPlugin)
            .add_state::<GamePlayState>()
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemySpawnerPlugin)
            .add_systems(
                OnEnter(GameState::Gameplay),
                (reset_camera, spawn_game_stuff),
            )
            .add_systems(OnExit(GameState::Gameplay), despawn_game_stuff);
    }
}

fn spawn_game_stuff(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut next_gameplay_state: ResMut<NextState<GamePlayState>>,
    // mut next_level_state: ResMut<NextState<CorridorLevelState>>,
) {
    println!("Loading game plugin");

    commands.spawn((
        AudioBundle {
            source: assets.load("music/gameplay.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Absolute(VolumeLevel::new(0.5)),
                ..default()
            },
        },
        MyMusic,
    ));

    next_gameplay_state.set(GamePlayState::Init);
}

fn despawn_game_stuff(
    mut commands: Commands,
    music_query: Query<Entity, With<MyMusic>>,
    assets: Res<AssetServer>,
) {
    for music in &music_query {
        commands.entity(music).despawn_recursive();
    }
}

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
