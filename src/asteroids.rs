use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    RES_HEIGHT, RES_WIDTH,
    collisions::{GROUP_ASTEROID, GROUP_PLAYER, GROUP_PROJECTILE},
    get_high_res_size,
};

#[derive(Component)]
pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Asteroid {
    pub score: u32,
    pub scale: f32,
}

pub fn init_timer(mut commands: Commands) {
    commands.spawn(SpawnTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}

pub fn manage_asteroids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer_query: Query<&mut SpawnTimer>,
    time: Res<Time>,
    transform: Query<&Transform, With<Asteroid>>,
    entity: Query<Entity, With<Asteroid>>,
    window: Single<&Window>,
) {
    // despawn logic
    for (trans, ent) in transform.iter().zip(entity.iter()) {
        if trans.translation.x > (RES_WIDTH) as f32 {
            commands.entity(ent).despawn();
        }
        if trans.translation.x < -(RES_WIDTH as f32) {
            commands.entity(ent).despawn();
        }
        if trans.translation.y > (RES_HEIGHT) as f32 {
            commands.entity(ent).despawn();
        }
        if trans.translation.y < -(RES_HEIGHT as f32) {
            commands.entity(ent).despawn();
        }
    }

    // get spawn timer
    let mut spawn_timer = match timer_query.single_mut() {
        Ok(tim) => tim,
        Err(e) => {
            warn!("failed to get asteroid spawn timer: {}.", e);
            return;
        }
    };
    spawn_timer.timer.tick(time.delta());

    let mut rng = rand::rng();

    if !spawn_timer.timer.just_finished() || rng.random_bool(1.0 / 4.0) {
        return;
    }

    let pos_x = rng.random_range(-(RES_WIDTH as i32 / 2)..(RES_WIDTH as i32 / 2));
    let pos_y = rng.random_range(-(RES_HEIGHT as i32 / 2)..(RES_HEIGHT as i32 / 2));
    let scale = rng.random_range(0.3..0.6);
    let linvel_x = rng.random_range(-15..15);
    let linvel_y = rng.random_range(-15..15);
    let angvel = rng.random_range(-2..2);

    let score: u32 = {
        let m = (2.0_f32 - 4.0_f32) / (0.6_f32 - 0.3_f32);
        let b = 4.0_f32 - m * 0.3_f32;
        (m * scale + b).floor() as u32
    };

    // TODO: don't spawn near the player, limit spawning
    // spawningggg
    commands.spawn((
        Sprite::from_image(asset_server.load("asteroids/1.png")),
        Transform::from_xyz(pos_x as f32, pos_y as f32, 0.0).with_scale(Vec3::splat(scale / 40.0)),
        Velocity {
            linvel: Vec2 {
                x: linvel_x as f32,
                y: linvel_y as f32,
            },
            angvel: angvel as f32,
        },
        GravityScale(0.0),
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        },
        Sleeping::disabled(),
        RigidBody::Dynamic,
        Collider::ball(500.0 * scale * get_high_res_size(&window)),
        ActiveEvents::COLLISION_EVENTS,
        Ccd::enabled(),
        Asteroid { scale, score },
        CollisionGroups::new(
            Group::from_bits_truncate(GROUP_ASTEROID),
            Group::from_bits_truncate(GROUP_PLAYER | GROUP_PROJECTILE | GROUP_ASTEROID),
        ),
    ));
}
