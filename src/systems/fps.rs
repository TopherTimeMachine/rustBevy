use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

#[derive(Resource)]
pub struct ShowFps(pub bool);

#[derive(Component)]
pub struct FpsText;

pub fn toggle_fps(
    keyboard: Res<Input<KeyCode>>,
    mut show_fps: ResMut<ShowFps>,
) {
    if keyboard.just_pressed(KeyCode::F) {
        show_fps.0 = !show_fps.0;
    }
}

pub fn update_fps_display(
    mut commands: Commands,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    show_fps: Res<ShowFps>,
    mut query: Query<Entity, With<FpsText>>,
) {
    // Remove existing FPS text if it exists and shouldn't be shown
    if let Ok(fps_entity) = query.get_single() {
        if !show_fps.0 {
            commands.entity(fps_entity).despawn();
            return;
        }
    }

    // Add FPS text if it should be shown and doesn't exist
    if show_fps.0 && query.is_empty() {
        commands.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    "...",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::GREEN,
                        ..default()
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            }),
            FpsText,
        ));
    }

    // Update FPS text if it exists
    if let Ok(fps_entity) = query.get_single() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                commands.entity(fps_entity).insert(TextBundle::from_sections([
                    TextSection::new(
                        "FPS: ",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        format!("{value:.1}"),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::GREEN,
                            ..default()
                        },
                    ),
                ]));
            }
        }
    }
} 