mod camera;
mod world;
mod systems;

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use camera::{CameraController, camera_controller_system};
use world::setup_world;
use systems::{ShowFps, toggle_fps, update_fps_display};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ShowFps(false))
        .add_systems(Startup, (setup, setup_world))
        .add_systems(Update, (
            camera_controller_system,
            handle_exit_input,
            toggle_fps,
            update_fps_display,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController {
            speed: 5.0,
            sensitivity: 0.002,
        },
    ));
}

fn handle_exit_input(
    keyboard: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}
