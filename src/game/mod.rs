mod level;
mod player;
mod projectile_spawner;
// mod potato_anim;
// mod potato_enemy;
mod spawner;
// mod potato_spawner;
mod game_over;
mod lvl_up_ui;
mod ui;
pub mod weapons;

use bevy::app::Plugin;
use bevy::audio::{PlaybackMode, Volume, VolumeLevel};
use bevy::prelude::*;

use crate::GameState;

use self::level::LevelPlugin;
use self::player::PlayerPlugin;

use self::game_over::GameOverPlugin;
use self::lvl_up_ui::LvlUpUiPlugin;
use self::projectile_spawner::ProjectileSpawnerPlugin;
use self::spawner::EnemySpawnerPlugin;
use self::ui::GameUiPlugin;

pub struct GameplayPlugin;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum GamePlayState {
    #[default]
    Unloaded,
    Init,
    Started,
    LevelUp,
    Boss,
    GameOver,
    Restart,
}

#[derive(Component)]
pub struct MyMusic;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameUiPlugin)
            .add_plugins(LvlUpUiPlugin)
            .add_plugins(LevelPlugin)
            .add_state::<GamePlayState>()
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemySpawnerPlugin)
            .add_plugins(ProjectileSpawnerPlugin)
            .add_plugins(GameOverPlugin)
            .add_systems(
                OnEnter(GameState::Gameplay),
                (reset_camera, spawn_game_stuff),
            )
            .add_systems(OnEnter(GamePlayState::Restart), (reset_camera, restart))
            .add_systems(OnExit(GameState::Gameplay), despawn_game_stuff);
    }
}

fn restart(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut next_gameplay_state: ResMut<NextState<GamePlayState>>,
    // mut next_level_state: ResMut<NextState<CorridorLevelState>>,
) {
    next_gameplay_state.set(GamePlayState::Init);
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
                volume: Volume::Absolute(VolumeLevel::new(0.3)),
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
