use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a wireframe floor
    let mut floor_mesh = Mesh::new(PrimitiveTopology::LineList);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let size = 10;
    let divisions = 10;

    // Create grid vertices
    for i in 0..=divisions {
        let x = (i as f32 / divisions as f32) * size as f32 - (size as f32 / 2.0);
        for j in 0..=divisions {
            let z = (j as f32 / divisions as f32) * size as f32 - (size as f32 / 2.0);
            vertices.push([x, 0.0, z]);
        }
    }

    // Create indices for lines
    let vertices_per_row = divisions + 1;
    // Horizontal lines
    for i in 0..=divisions {
        for j in 0..divisions {
            let current = i * vertices_per_row + j;
            indices.extend_from_slice(&[current, current + 1]);
        }
    }
    // Vertical lines
    for i in 0..divisions {
        for j in 0..=divisions {
            let current = i * vertices_per_row + j;
            indices.extend_from_slice(&[current, current + vertices_per_row]);
        }
    }

    floor_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    floor_mesh.set_indices(Some(Indices::U32(indices)));

    // Spawn the floor mesh
    commands.spawn(PbrBundle {
        mesh: meshes.add(floor_mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Add light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
} 