use bevy::prelude::*;
use num::clamp;
use libm::atan2;

#[derive(Component)]
pub struct PlayerMove {
    pub max_speed: f32,
    pub accel: f32,
    pub speed_x: f32,
    pub speed_z: f32,
}

#[derive(Component)]
pub struct PlayerParent;

impl Default for PlayerMove {
    fn default() -> Self {
        PlayerMove {
            max_speed: 3.0,
            accel: 10.0,
            speed_x: 0.0,
            speed_z: 0.0,
        }
    }
}

//move player with WASD keys
pub fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut q_parent: Query<&mut Transform, With<PlayerParent>>,
    mut query: Query<(&mut PlayerMove, &mut Transform, &Parent), (Without<PlayerParent>, With<PlayerMove>)>,
) {
    for (mut player_move, mut transform, parent) in query.iter_mut() {
        let mut parent_transform = q_parent.get_mut(parent.get()).unwrap();
        //set target speeds based on keys pressed
        let mut target_speed_x = 0.0;
        let mut target_speed_z = 0.0;
        if keys.pressed(KeyCode::W) {
            target_speed_x -= player_move.max_speed;
            target_speed_z -= player_move.max_speed;
        }
        if keys.pressed(KeyCode::A) {
            target_speed_x -= player_move.max_speed;
            target_speed_z += player_move.max_speed;
        }
        if keys.pressed(KeyCode::S) {
            target_speed_x += player_move.max_speed;
            target_speed_z += player_move.max_speed;
        }
        if keys.pressed(KeyCode::D) {
            target_speed_x += player_move.max_speed;
            target_speed_z -= player_move.max_speed;
        }
        //adjust diagonal movement speed
        if target_speed_x != 0.0 && target_speed_z != 0.0 {
            target_speed_x /= (2 as f32).sqrt();
            target_speed_z /= (2 as f32).sqrt();
        }
        //accelerate x speed to target
        if (player_move.speed_x - target_speed_x).abs() < player_move.accel * time.delta_seconds() {
            player_move.speed_x = target_speed_x;
        } else if player_move.speed_x < target_speed_x {
            player_move.speed_x += player_move.accel * time.delta_seconds();
        } else {
            player_move.speed_x -= player_move.accel * time.delta_seconds();
        }
        //accelerate z speed to target
        if (player_move.speed_z - target_speed_z).abs() < player_move.accel * time.delta_seconds() {
            player_move.speed_z = target_speed_z;
        } else if player_move.speed_z < target_speed_z {
            player_move.speed_z += player_move.accel * time.delta_seconds();
        } else {
            player_move.speed_z -= player_move.accel * time.delta_seconds();
        }
        //clamp speeds and apply translation
        player_move.speed_x = clamp(player_move.speed_x, -player_move.max_speed, player_move.max_speed);
        player_move.speed_z = clamp(player_move.speed_z, -player_move.max_speed, player_move.max_speed);
        parent_transform.translation.x += player_move.speed_x * time.delta_seconds();
        parent_transform.translation.z += player_move.speed_z * time.delta_seconds();
        transform.rotate(Quat::from_rotation_y(atan2(player_move.speed_x as f64, player_move.speed_z as f64) as f32));
    }
}
