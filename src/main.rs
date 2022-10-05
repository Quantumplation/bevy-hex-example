use bevy::{
    prelude::*,
    render::{camera::Camera, mesh::Indices, render_resource::PrimitiveTopology},
};
use rand::prelude::*;

mod geometry;
mod hex;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(sample_level)
        .add_system(keyboard_controls)
        .run();
}

fn sample_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // camera
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-5.0, 18., 0.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        });

    commands
        // light
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            point_light: PointLight {
                intensity: 5000.0,
                ..default()
            },
            ..Default::default()
        });

    let colors = [
        Color::rgb(0.286, 0.725, 0.902), // Water #49B9E6 (73, 185, 230)
        Color::rgb(0.698, 0.941, 0.329), // Grass #B2F054 (178, 240, 84)
        Color::rgb(0.722, 0.522, 0.380), // Hills ##B88561 (184, 133, 97)
    ];
    // Generate our hex mesh
    let mesh = meshes.add(generate_hex_mesh()).clone();
    let mut rng = rand::thread_rng();
    for q in -7..7 {
        for r in -7..7 {
            let tile = rng.gen_range(0..10);
            let tile = if tile > 0 && tile < 5 {
                0
            } else if tile >= 5 && tile < 7 {
                1
            } else {
                2
            };
            let color = colors[tile].clone();
            let height = 0.;
            let pos = geometry::center(1.0, &hex::HexCoord::new(q, r), &[0., height, 0.]);

            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(color.into()),
                transform: Transform::from_translation(Vec3::new(pos[0], pos[1], pos[2])),
                ..Default::default()
            });
        }
    }
}

/// Generate a single hex mesh
fn generate_hex_mesh() -> Mesh {
    let mut pts: Vec<[f32; 3]> = vec![];
    let c = hex::HexCoord::new(0, 0);
    geometry::bevel_hexagon_points(&mut pts, 1.0, 0.9, &c);

    let mut normals: Vec<[f32; 3]> = vec![];
    geometry::bevel_hexagon_normals(&mut normals);

    let mut uvs: Vec<[f32; 2]> = vec![];
    for _ in 0..pts.len() {
        uvs.push([0., 0.]);
    }

    let mut indices = vec![];
    geometry::bevel_hexagon_indices(&mut indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, pts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

/* Supporting systems */

/// Move the camera around with the keyboard
pub fn keyboard_controls(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    let (mut transform, _camera) = query.iter_mut().next().unwrap();
    let speed = 10.;
    let forward = Vec3::new(1., 0., 0.);
    let left = Vec3::new(0., 0., -1.);
    let up = Vec3::new(0., 1., 0.);
    let mut pos = transform.translation.clone();
    if input.pressed(KeyCode::W) {
        pos += speed * forward * time.delta_seconds();
    } else if input.pressed(KeyCode::S) {
        pos -= speed * forward * time.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        pos += speed * left * time.delta_seconds();
    } else if input.pressed(KeyCode::D) {
        pos -= speed * left * time.delta_seconds();
    }
    if input.pressed(KeyCode::Q) {
        pos += speed * up * time.delta_seconds();
    } else if input.pressed(KeyCode::E) {
        pos -= speed * up * time.delta_seconds();
    }

    transform.translation = pos;
}
