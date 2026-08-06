mod modules;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use modules::player::PlayerPlugin;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE)) 
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                file_path: "src/assets".into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
