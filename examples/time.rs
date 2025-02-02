use bevy::prelude::*;
use bevy_pancam::*;
use bevy_smud::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.7, 0.8, 0.7)))
        .add_plugins(DefaultPlugins)
        .add_plugin(SmudPlugin)
        .add_plugin(PanCamPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
    let bevy_shape_shader = asset_server.load("star_bevy.wgsl");

    commands.spawn_bundle(ShapeBundle {
        shape: SmudShape {
            color: Color::rgb(0.36, 0.41, 0.45),
            sdf: bevy_shape_shader,
            frame: Frame::Quad(400.),
            ..default()
        },
        ..default()
    });

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());
}
