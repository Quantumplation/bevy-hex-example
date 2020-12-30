#![feature(exclusive_range_pattern)]
use bevy::{input::system::exit_on_esc_system, prelude::*, render::{camera::Camera, mesh::Indices, pipeline::PrimitiveTopology}};
use rand::prelude::*;

mod geometry;
mod hex;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_startup_system(sample_level.system())
        .add_system(keyboard_controls.system())
        .add_system(water_ripple.system())
        .run();
}

fn sample_level(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-10.0, 15., 0.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

    let colors = [
        Color::rgb(0.286, 0.725, 0.902), // Water #49B9E6 (73, 185, 230)
        Color::rgb(0.698, 0.941, 0.329), // Grass #B2F054 (178, 240, 84)
        Color::rgb(0.722, 0.522, 0.380), // Hills ##B88561 (184, 133, 97)
    ];
    // Generate our hex mesh
    let mesh = meshes.add(generate_hex_mesh());
    let mut rng = rand::thread_rng();
    for q in -15..15 {
        for r in -15..15 {
            let tile = match rng.gen_range(0..10) {
                0..5 => 0,
                5..7 => 1,
                _ => 2,
            };
            let color = colors[tile].clone();
            let height = match tile {
                0 => 0.,
                1 => 0.5 + rng.gen_range(-0.2..0.2),
                2 => 2. + rng.gen_range(-0.5..0.5),
                _ => unreachable!()
            };
            let pos = geometry::center(
                1.0,
                &hex::HexCoord::new(q, r),
                &[0., height, 0.],
            );
            add_hex(
                Vec3::new(pos[0], pos[1], pos[2]),
                color,
                mesh.clone(),
                commands,
                &mut materials,
            );
            if tile == 0 {
                commands.with(Water);
            }
        }
    }
}

/// Spawn a hex in the world
fn add_hex(
    position: Vec3,
    color: Color,
    mesh: Handle<Mesh>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh,
        material: materials.add(color.into()),
        transform: Transform::from_translation(position),
        ..Default::default()
    });
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
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, pts);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
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

pub struct Water;
/// Ripple water tiles slightly
pub fn water_ripple(time: Res<Time>, mut q: Query<&mut Transform, With<Water>>) {
    let time = time.seconds_since_startup() as f32;
    for mut t in q.iter_mut() {
        let (x, z) = (t.translation.x, t.translation.z);

        let ripple1 = (time / 2. + (x / 3.) + (z / 3.)).sin() * 0.1 - 0.05;
        let ripple2 = (time + (x / 3.) - (z / 4.)).cos() * 0.1 - 0.05;
        let ripple3 = (time * 2. + (x / 5.) - (z / 7.)).sin() * 0.1 - 0.05;
        t.translation = Vec3::new(
            x,
            ripple1 + ripple2 + ripple3,
            z,
        );
    }
}