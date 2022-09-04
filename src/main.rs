use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::WorldInspectorPlugin;
use num::clamp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(move_camera)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: 1.0
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(PanCamera {
        ..Default::default()
    });
}

#[derive(Component)]
struct PanCamera {
    pub max_speed: f32,
    pub accel: f32,
    pub speed_x: f32,
    pub speed_z: f32,
}

impl Default for PanCamera {
    fn default() -> Self {
        PanCamera {
            max_speed: 3.0,
            accel: 10.0,
            speed_x: 0.0,
            speed_z: 0.0,
        }
    }
}

//move camera with WASD keys
fn move_camera(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut PanCamera, &mut Transform), With<PanCamera>>,
) {
    for (mut pan_camera, mut transform) in query.iter_mut() {
        //set target speeds based on keys pressed
        let mut target_speed_x = 0.0;
        let mut target_speed_z = 0.0;
        if keys.pressed(KeyCode::W) {
            target_speed_x -= pan_camera.max_speed;
            target_speed_z -= pan_camera.max_speed;
        }
        if keys.pressed(KeyCode::A) {
            target_speed_x -= pan_camera.max_speed;
            target_speed_z += pan_camera.max_speed;
        }
        if keys.pressed(KeyCode::S) {
            target_speed_x += pan_camera.max_speed;
            target_speed_z += pan_camera.max_speed;
        }
        if keys.pressed(KeyCode::D) {
            target_speed_x += pan_camera.max_speed;
            target_speed_z -= pan_camera.max_speed;
        }
        //adjust diagonal movement speed
        if target_speed_x != 0.0 && target_speed_z != 0.0 {
            target_speed_x /= (2 as f32).sqrt();
            target_speed_z /= (2 as f32).sqrt();
        }
        //accelerate x speed to target
        if (pan_camera.speed_x - target_speed_x).abs() < pan_camera.accel * time.delta_seconds() {
            pan_camera.speed_x = target_speed_x;
        } else if pan_camera.speed_x < target_speed_x {
            pan_camera.speed_x += pan_camera.accel * time.delta_seconds();
        } else {
            pan_camera.speed_x -= pan_camera.accel * time.delta_seconds();
        }
        //accelerate z speed to target
        if (pan_camera.speed_z - target_speed_z).abs() < pan_camera.accel * time.delta_seconds() {
            pan_camera.speed_z = target_speed_z;
        } else if pan_camera.speed_z < target_speed_z {
            pan_camera.speed_z += pan_camera.accel * time.delta_seconds();
        } else {
            pan_camera.speed_z -= pan_camera.accel * time.delta_seconds();
        }
        //clamp speeds and apply translation
        pan_camera.speed_x = clamp(pan_camera.speed_x, -pan_camera.max_speed, pan_camera.max_speed);
        pan_camera.speed_z = clamp(pan_camera.speed_z, -pan_camera.max_speed, pan_camera.max_speed);
        transform.translation.x += pan_camera.speed_x * time.delta_seconds();
        transform.translation.z += pan_camera.speed_z * time.delta_seconds();
    }
}
