use bevy::prelude::*;
use my_library::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
    #[default]
    Loading,
    MainMenu,
    Playing,
    GameOver,
}

#[derive(Component)]
struct GameElement;

#[derive(Component)]
struct Player;

fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    add_phase!(app, GamePhase, GamePhase::Playing,
        start => [setup],
        run => [end_game, physics_clock, sum_impulses, apply_gravity, apply_velocity],
        exit => [cleanup::<GameElement>]
    );

    app.add_event::<Impulse>();
    app.add_event::<PhysicsTick>();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mars Base One".to_string(),
            resolution: bevy::window::WindowResolution::new(1280.0, 720.0),
            ..default()
        }),

        ..default()
    }))
    .add_plugins(RandomPlugin)
    .add_plugins(GameStatePlugin::new(
        GamePhase::MainMenu,
        GamePhase::Playing,
        GamePhase::GameOver,
    ))
    .add_plugins(AssetManager::new())
    .insert_resource(Animations::new())
    .run();

    Ok(())
}

fn setup(mut commands: Commands, assets: Res<AssetStore>, loaded_assets: Res<LoadedAssets>) {
    commands.spawn(Camera::default()).insert(GameElement);
}

fn end_game(mut state: ResMut<NextState<GamePhase>>, assets: Res<AssetStore>) {
    //state.set(GamePhase::GameOver);
}
