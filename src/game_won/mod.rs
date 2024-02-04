use bevy::app::Plugin;
use bevy::audio::{PlaybackMode, Volume, VolumeLevel};
use bevy::prelude::*;

use crate::character_select::SelectedCharacterState;
use crate::game_won::player::{GameWonLevelState, GameWonPlayerState};
use crate::main_menu::MyMusic;
use crate::GameState;

pub struct GameWonPlugin;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum GameWonState {
    #[default]
    Unloaded,
    Init,
    Started,
    Congrats,
}

mod congrats;
mod level;
mod level_items;
mod npc;
mod player;

// use congrats::CongratsPlugin;
use level::LevelPlugin;
use npc::NpcPlugin;
use player::PlayerPlugin;

use self::congrats::CongratsPlugin;
use self::npc::GameWonNpcState;

impl Plugin for GameWonPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameWonState>()
            .add_systems(OnEnter(GameState::GameWon), (reset_camera, spawn_game_won))
            .add_plugins(LevelPlugin)
            .add_plugins(NpcPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(CongratsPlugin);
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

fn spawn_game_won(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut next_character_state: ResMut<NextState<SelectedCharacterState>>,
    mut next_player_state: ResMut<NextState<GameWonPlayerState>>,
    mut next_level_state: ResMut<NextState<GameWonLevelState>>,
    mut next_npc_state: ResMut<NextState<GameWonNpcState>>,
) {
    // TESTING PURPOSES - TODO: REMOVE
    // next_character_state.set(SelectedCharacterState::Ailsa);

    commands.spawn((
        AudioBundle {
            source: assets.load("music/menu.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Absolute(VolumeLevel::new(0.5)),
                ..default()
            },
        },
        MyMusic,
    ));

    println!("Loading game won plugin");
    next_player_state.set(GameWonPlayerState::Init);
    next_level_state.set(GameWonLevelState::Init);
    next_npc_state.set(GameWonNpcState::Init);
}

// fn despawn_corridor(mut commands: Commands, asset_server: Res<AssetServer>) {}
fn despawn_game_won(mut commands: Commands, entities: Query<Entity>) {
    // for entity in &entities {
    //     commands.entity(entity).despawn_recursive();
    // }
}
