use bevy::prelude::*;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy_rapier3d::prelude::*;

//keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

//component for storing the player's movement data
#[derive(Component)]
pub struct PlayerMove {
    pub sensitivity: f32,
    pub speed: f32,
    pub speed_x: f32,
    pub speed_z: f32,
    pub accel: f32,
    pub jump_v: f32,
    pub jumping: bool,
}

impl Default for PlayerMove {
    fn default() -> Self {
        Self {
            sensitivity: 0.00015,
            speed: 2.0,
            speed_x: 0.0,
            speed_z: 0.0,
            accel: 10.0,
            jump_v: 2.0,
            jumping: false,
        }
    }
}

//grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

//grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

//spawns camera entity with proper components
fn setup_player(
    mut commands: Commands,
) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y(0.25, 0.25))
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Velocity {
            ..default()
        })
        .insert(GravityScale(0.5))
        .insert(PlayerMove {
            ..default()
        });
}

//move player with keyboard input
fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut query: Query<(&mut PlayerMove, &mut Velocity, &mut Transform, ), With<PlayerMove>>,
) {
    if let Some(window) = windows.get_primary() {
        for (mut player_move, mut velocity, mut transform) in query.iter_mut() {

            //set target speeds from keys pressed
            let mut move_vel = Vec3::ZERO;
            let mut target_x = 0.0;
            let mut target_z = 0.0;
            for key in keys.get_pressed() {
                if window.cursor_locked() {
                    match key {
                        KeyCode::W => target_z += player_move.speed,
                        KeyCode::A => target_x -= player_move.speed,
                        KeyCode::S => target_z -= player_move.speed,
                        KeyCode::D => target_x += player_move.speed,
                        KeyCode::Space => {
                            if !player_move.jumping {
                                velocity.linvel.y = player_move.jump_v;
                                player_move.jumping = true;
                            }
                        },
                        _ => (),
                    }
                }
            }
            //adjust diagonal movement speed
            if target_x != 0.0 && target_z != 0.0 {
                target_x /= (2 as f32).sqrt();
                target_z /= (2 as f32).sqrt();
            }
            //counter-strafing (set speed to 0 if target flips direction)
            if player_move.speed_x > 0.0 && target_x < 0.0
            || player_move.speed_x < 0.0 && target_x > 0.0 {
                player_move.speed_x = 0.0;
            }
            if player_move.speed_z > 0.0 && target_z < 0.0
            || player_move.speed_z < 0.0 && target_z > 0.0 {
                player_move.speed_z = 0.0;
            }
            //accelerate in left/right direction
            if (player_move.speed_x - target_x).abs() < player_move.accel * time.delta_seconds() {
                player_move.speed_x = target_x;
            } else if player_move.speed_x < target_x {
                player_move.speed_x += player_move.accel * time.delta_seconds();
            } else {
                player_move.speed_x -= player_move.accel * time.delta_seconds();
            }
            //accelerate in forwards/backwards direction
            if (player_move.speed_z - target_z).abs() < player_move.accel * time.delta_seconds() {
                player_move.speed_z = target_z;
            } else if player_move.speed_z < target_z {
                player_move.speed_z += player_move.accel * time.delta_seconds();
            } else {
                player_move.speed_z -= player_move.accel * time.delta_seconds();
            }
            //apply translations
            let local_z = transform.local_z();
            move_vel += Vec3::new(local_z.z, 0.0, -local_z.x).normalize() * player_move.speed_x;
            move_vel += Vec3::new(-local_z.x, 0.0, -local_z.z).normalize() * player_move.speed_z;
            transform.translation.x += move_vel.x * time.delta_seconds();
            transform.translation.z += move_vel.z * time.delta_seconds();
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

//handles looking around if cursor is locked
fn player_look(
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<(&PlayerMove, &mut Transform), With<PlayerMove>>,
) {
    if let Some(window) = windows.get_primary() {
        let mut delta_state = state.as_mut();

        for (player_move, mut transform) in query.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                if window.cursor_locked() {
                    let window_scale = window.height().min(window.width());
                    delta_state.pitch -=
                        (player_move.sensitivity * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (player_move.sensitivity * ev.delta.x * window_scale).to_radians();
                }
                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);
                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

//plugin to bundle everything
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_startup_system(setup_player)
            .add_startup_system(initial_grab_cursor)
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);
    }
}
