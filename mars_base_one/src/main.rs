use bevy::render::camera::ScalingMode;
use bevy::{ecs::spawn, prelude::*};
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

#[derive(Component)]
struct MyCamera;

#[derive(Component)]
struct Ground;

struct World {
    solid: Vec<bool>,
    width: usize,
    height: usize,
}

impl World {
    fn mapidx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn new(width: usize, height: usize, rng: &mut RandomNumberGenerator) -> Self {
        let mut result = Self {
            width,
            height,
            solid: vec![true; width * height],
        };

        result.clear_tiles(width / 2, height / 2);

        result
    }

    fn spawn(&self, assets: &AssetStore, commands: &mut Commands, loaded_assets: &LoadedAssets) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.solid[y * self.width + x] {
                    let position = Vec2::new(
                        (x as f32 * 24.0) - ((self.width as f32 / 2.0) * 24.0),
                        (y as f32 * 24.0) - ((self.height as f32 / 2.0) * 24.0),
                    );
                    //spawn a solid block
                    spawn_image!(
                        assets,
                        commands,
                        "ground",
                        position.x,
                        position.y,
                        -1.0,
                        loaded_assets,
                        GameElement,
                        Ground,
                        PhysicsPosition::new(Vec2::new(position.x, position.y)),
                        AxisAlignedBoundingBox::new(24.0, 24.0)
                    );
                }
            }
        }
    }

    fn clear_tiles(&mut self, x: usize, y: usize) {
        for offset_x in -1..=1 {
            for offset_y in -1..=1 {
                let x = x as isize + offset_x;
                let y = y as isize + offset_y;
                if x > 0 && x < self.width as isize - 1 && y > 0 && y < self.height as isize {
                    let idx = self.mapidx(x as usize, y as usize);
                    self.solid[idx] = false;
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    add_phase!(app, GamePhase, GamePhase::Playing,
        start => [setup],
        run => [ movement, end_game, physics_clock,
            sum_impulses, apply_gravity, apply_velocity,
            terminal_velocity.after(apply_velocity),
            camera_follow.after(terminal_velocity) ],
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
    .add_plugins(
        AssetManager::new()
            .add_image("ship", "ship.png")?
            .add_image("ground", "ground.png")?,
    )
    .insert_resource(Animations::new())
    .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetStore>,
    loaded_assets: Res<LoadedAssets>,
    mut rng: ResMut<RandomNumberGenerator>,
) {
    let cb = Camera2d;
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize,
        scale: 0.5,
        ..OrthographicProjection::default_2d()
    });
    commands
        .spawn(cb)
        .insert(projection)
        .insert(GameElement)
        .insert(MyCamera);

    spawn_image!(
        assets,
        commands,
        "ship",
        0.0,
        0.0,
        1.0,
        &loaded_assets,
        GameElement,
        Player,
        Velocity::default(),
        PhysicsPosition::new(Vec2::new(0.0, 0.0)) //ApplyGravity
    );

    let world = World::new(200, 200, &mut rng);
    world.spawn(&assets, &mut commands, &loaded_assets);
    commands.insert_resource(StaticQuadTree::new(Vec2::new(10240.0, 7680.0), 6));
}

fn end_game(
    mut state: ResMut<NextState<GamePhase>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(transform) = player_query.single() else {
        return;
    };

    if transform.translation.y < -384.0
        || transform.translation.y > 384.0
        || transform.translation.x < -512.0
        || transform.translation.x > 512.0
    {
        state.set(GamePhase::GameOver);
    }
}
fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    mut impulses: EventWriter<Impulse>,
) {
    let Ok((entity, mut transform)) = player_query.single_mut() else {
        return;
    };

    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.rotate(Quat::from_rotation_z(f32::to_radians(2.0)));
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.rotate(Quat::from_rotation_z(f32::to_radians(-2.0)));
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        impulses.write(Impulse {
            target: entity,
            amount: transform.local_y().as_vec3(),
            absolute: false,
            source: 1,
        });
    }
}

fn terminal_velocity(mut player_query: Query<&mut Velocity, With<Player>>) {
    let Ok(mut velocity) = player_query.single_mut() else {
        return;
    };

    let v2 = velocity.0.truncate();
    if v2.length() > 5.0 {
        let new_v2 = v2.normalize() * 5.0;
        velocity.0.x = new_v2.x;
        velocity.0.y = new_v2.y;
    }
}

fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MyCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MyCamera>, Without<Player>)>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    camera.translation = Vec3::new(player.translation.x, player.translation.y, 10.0);
}
