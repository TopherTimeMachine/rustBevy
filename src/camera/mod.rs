use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct CameraController {
    pub speed: f32,
    pub sensitivity: f32,
}

pub fn camera_controller_system(
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &CameraController)>,
    time: Res<Time>,
) {
    let (mut transform, controller) = query.single_mut();

    // Handle keyboard input
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) {
        direction += transform.forward();
    }
    if keyboard.pressed(KeyCode::S) {
        direction += transform.back();
    }
    if keyboard.pressed(KeyCode::A) {
        direction += transform.left();
    }
    if keyboard.pressed(KeyCode::D) {
        direction += transform.right();
    }

    // Apply movement
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        transform.translation += direction * controller.speed * time.delta_seconds();
    }

    // Handle mouse input
    for ev in mouse_motion.iter() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw -= ev.delta.x * controller.sensitivity;
        pitch -= ev.delta.y * controller.sensitivity;
        pitch = pitch.clamp(-1.54, 1.54);
        
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
} 