use bevy::prelude::*;

#[derive(Component)]
struct Dragon;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let dragon_image = asset_server.load("dragon.png");
    commands
        .spawn(Sprite::from_image(dragon_image))
        .insert(Dragon);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
