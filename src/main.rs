use std::hash::Hash;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

use bevy::render::render_resource::FilterMode;
use bevy::render::texture::{ImageFilterMode, ImageLoaderSettings};
use bevy::winit::WinitSettings;
use bevy::{input::common_conditions::input_toggle_active, time::Stopwatch};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use character_select::CharacterSelectPlugin;
use corridor::CorridorPlugin;
use game_won::GameWonPlugin;
use main_menu::MainMenuPlugin;
use util_fade::FadePlugin;

mod character_select;
mod corridor;
mod util_fade;
// mod game;
// mod game_over;
mod game_won;
mod main_menu;
mod sprite;

const WIDTH: f32 = 1600.0;
const HEIGHT: f32 = 900.0;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    CharacterSelect,
    Corridor,
    Gameplay,
    LevelUp,
    GameOver,
    GameWon,
}

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        // .insert_resource(Msaa { samples: 1 })
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Valiant Duo".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((
            FadePlugin,
            MainMenuPlugin,
            CharacterSelectPlugin,
            CorridorPlugin,
            GameWonPlugin,
            /*GamePlugin, GameOver*/
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        // .insert_resource(WinitSettings::desktop_app())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        projection: OrthographicProjection {
            // far: 1000.,
            near: -1000.,
            scale: 0.35,
            ..default()
        },
        ..default()
    });
}
