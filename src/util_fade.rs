use bevy::prelude::*;

pub struct FadePlugin;

#[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
pub enum FadeState {
    #[default]
    Gone,
    FadeToBlack,
    FadeToGame,
}

#[derive(Debug, Component)]
pub struct Fadeout {
    in_timer: Timer,
    // hold_timer: Timer,
    out_timer: Timer,
}

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<FadeState>()
            .add_systems(Startup, init_fadeout)
            .add_systems(
                Update,
                fade_to_black.run_if(in_state(FadeState::FadeToBlack)),
            )
            .add_systems(Update, fade_to_game.run_if(in_state(FadeState::FadeToGame)));
    }
}

fn init_fadeout(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.0)),
            z_index: ZIndex::Global(9999),
            ..default()
        },
        Fadeout {
            in_timer: Timer::from_seconds(1.0, TimerMode::Once),
            out_timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        Name::new("Fadeout"),
    ));

    println!("Spawned fadeout");
}

fn fade_to_black(
    mut query: Query<(&mut Fadeout, &mut BackgroundColor)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<FadeState>>,
) {
    // println!("Update fadetoblack");
    for (mut fadeout, mut color) in &mut query.iter_mut() {
        fadeout.in_timer.tick(time.delta());

        if fadeout.in_timer.just_finished() {
            println!("Finished fadeout in");
            color.0.set_a(1.);
            fadeout.in_timer.reset();
            next_state.set(FadeState::FadeToGame);
        } else {
            color.0.set_a(fadeout.in_timer.percent());
        }
    }
}

fn fade_to_game(
    mut query: Query<(&mut Fadeout, &mut BackgroundColor)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<FadeState>>,
) {
    // println!("Update fadetogame");
    for (mut fadeout, mut color) in &mut query.iter_mut() {
        fadeout.out_timer.tick(time.delta());

        if fadeout.out_timer.just_finished() {
            println!("Finished fadeout out");
            color.0.set_a(0.);
            fadeout.out_timer.reset();
            next_state.set(FadeState::Gone);
        } else {
            color.0.set_a(1. - fadeout.out_timer.percent());
        }
    }
}
