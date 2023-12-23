use bevy::{
    prelude::*,
    render::{camera::Camera, mesh::Indices, render_resource::PrimitiveTopology},
};
use rand::prelude::*;

use bevy_hex_example::{geometry, hex};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, sample_level)
        .add_systems(Update, (keyboard_controls, water_ripple))
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
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-10.0, 15., 0.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        });

    commands
        // light
        .spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
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
    for q in -15..15 {
        for r in -15..15 {
            let tile = rng.gen_range(0..10);
            let tile = if (1..5).contains(&tile) {
                0
            } else if (5..7).contains(&tile) {
                1
            } else {
                2
            };
            let color = colors[tile];
            let height = match tile {
                0 => 0.,
                1 => 0.5 + rng.gen_range(-0.2..0.2),
                2 => 2. + rng.gen_range(-0.5..0.5),
                _ => unreachable!(),
            };
            let pos = geometry::center(1.0, &hex::HexCoord::new(q, r), &[0., height, 0.]);

            let mut cmd = commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(color.into()),
                transform: Transform::from_translation(Vec3::new(pos[0], pos[1], pos[2])),
                ..Default::default()
            });

            if tile == 0 {
                cmd.insert(Water);
            }
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
    if let Some((mut transform, _camera)) = query.iter_mut().next() {
        let speed = 10.;
        let forward = Vec3::new(1., 0., 0.);
        let left = Vec3::new(0., 0., -1.);
        let up = Vec3::new(0., 1., 0.);
        let mut pos = transform.translation;
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
}

#[derive(Component)]
pub struct Water;
/// Ripple water tiles slightly
pub fn water_ripple(time: Res<Time>, mut q: Query<&mut Transform, With<Water>>) {
    let time = time.elapsed_seconds();
    for mut t in &mut q {
        let (x, z) = (t.translation.x, t.translation.z);

        let ripple1 = (time / 2. + (x / 3.) + (z / 3.)).sin() * 0.1 - 0.05;
        let ripple2 = (time + (x / 3.) - (z / 4.)).cos() * 0.1 - 0.05;
        let ripple3 = (time * 2. + (x / 5.) - (z / 7.)).sin() * 0.1 - 0.05;
        t.translation = Vec3::new(x, ripple1 + ripple2 + ripple3, z);
    }
}
