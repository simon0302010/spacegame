use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{RES_HEIGHT, RES_WIDTH};

pub fn spawn_asteroid(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut rng = rand::rng();

    if !rng.random_bool(1.0 / 100.0) {
        return;
    }

    let pos_x = rng.random_range(-(RES_WIDTH as i32 / 2)..(RES_WIDTH as i32 / 2));
    let pos_y = rng.random_range(-(RES_HEIGHT as i32 / 2)..(RES_HEIGHT as i32 / 2));
    let scale = rng.random_range(60..100);
    let linvel_x = rng.random_range(-15..15);
    let linvel_y = rng.random_range(-15..15);

    commands.spawn((
        Sprite::from_image(asset_server.load("asteroids/1.png")),
        Transform::from_xyz(pos_x as f32, pos_y as f32, 0.0).with_scale(Vec3::splat(1.0 / scale as f32)),
        Velocity {
            linvel: Vec2 { x: linvel_x as f32, y: linvel_y as f32 },
            ..default()
        },
        GravityScale(0.0),
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        },
        Sleeping::disabled(),
        RigidBody::Dynamic,
        Collider::ball(20.0),
    ));
}