use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod player;
use player::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(2.5, 0.0, 2.5));
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: 1.0
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5, 0.5, 0.5));
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, -2.0),
        ..default()
    });
    // player
    commands.spawn_bundle(SceneBundle {
        transform: Transform::from_xyz(2.0, 2.0, 2.0),
        ..default()
    })
    .insert(PlayerParent)
    .with_children(|parent| {
        parent.spawn_bundle(SceneBundle {
            scene: asset_server.load("duck.glb#Scene0"),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(0.38))
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(PlayerMove {
            ..default()
        });
        parent.spawn_bundle(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(3.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    });
}
