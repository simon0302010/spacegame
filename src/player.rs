use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{RES_HEIGHT, RES_WIDTH, collisions::{GROUP_ASTEROID, GROUP_PLAYER, GROUP_PROJECTILE}, get_high_res_size};

#[derive(Component)]
pub struct Player;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<&Window>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("ship.png")),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0 / 40.0)),
        RigidBody::Dynamic,
        GravityScale(0.0),
        Velocity::default(),
        Damping {
            linear_damping: 2.0,
            angular_damping: 3.0,
        },
        Collider::ball(9.0 * 10.0 * get_high_res_size(&window)),
        Player,
        ActiveEvents::COLLISION_EVENTS,
        Ccd::enabled(),
        CollisionGroups::new(
            Group::from_bits_truncate(GROUP_PLAYER),
            Group::from_bits_truncate(GROUP_ASTEROID | GROUP_PROJECTILE)
        )
    ));
}

const TURN_SPEED: f32 = 0.5;
const MAX_TURN_SPEED: f32 = 5.0;
const THRUST: f32 = 600.0;

/// Controls player
pub fn control_player(
    time: Res<Time>,
    mut velocity: Query<&mut Velocity, With<Player>>,
    mut transforms: Query<&mut Transform, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut vel) = velocity.single_mut()
        && let Ok(transform) = transforms.single_mut()
    {
        if kb_input.pressed(KeyCode::ArrowRight) || kb_input.pressed(KeyCode::KeyD) {
            vel.angvel = (vel.angvel - TURN_SPEED).max(-MAX_TURN_SPEED);
        } else if kb_input.pressed(KeyCode::ArrowLeft) || kb_input.pressed(KeyCode::KeyA) {
            vel.angvel = (vel.angvel + TURN_SPEED).min(MAX_TURN_SPEED);
        }

        // Forward thrust
        if kb_input.pressed(KeyCode::ArrowUp) || kb_input.pressed(KeyCode::KeyW) {
            let direction = transform.up();
            vel.linvel += direction.xy() * THRUST * time.delta_secs();
        }
    }
}

pub fn keep_player(mut transform: Query<&mut Transform, With<Player>>) {
    if let Ok(mut trans) = transform.single_mut() {
        if trans.translation.x > (RES_WIDTH / 2) as f32 {
            trans.translation.x = -((RES_WIDTH as f32) / 2.0);
        }
        if trans.translation.x < -((RES_WIDTH as f32) / 2.0) {
            trans.translation.x = (RES_WIDTH / 2) as f32;
        }
        if trans.translation.y > (RES_HEIGHT / 2) as f32 {
            trans.translation.y = -((RES_HEIGHT as f32) / 2.0);
        }
        if trans.translation.y < -((RES_HEIGHT as f32) / 2.0) {
            trans.translation.y = (RES_HEIGHT / 2) as f32;
        }
    }
}
