use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    RES_HEIGHT, RES_WIDTH,
    collisions::{GROUP_ASTEROID, GROUP_PROJECTILE},
    get_high_res_size,
    player::Player,
};

#[derive(Component)]
pub struct Projectile {
    initial_velocity: Vec2,
}

#[derive(Resource)]
pub struct ProjectilesData {
    pub last_shoot: f32,
}

const SHOOT_STRENGTH: f32 = 200.0;
const MAX_SHOOT_DELTA_S: f32 = 0.2;

// TODO: sound
pub fn shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kb_input: Res<ButtonInput<KeyCode>>,
    player_transform: Query<(&Transform, &Velocity), With<Player>>,
    window: Single<&Window>,
    mut proj_data: ResMut<ProjectilesData>,
    time: Res<Time>,
) {
    if kb_input.just_pressed(KeyCode::Space)
        && let Ok((trans, vel)) = player_transform.single()
        && time.elapsed_secs() - proj_data.last_shoot > MAX_SHOOT_DELTA_S
    {
        let rotated: Vec3 =
            (trans.rotation * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)) * Vec3::X;
        let velocity = Vec2::new(rotated.x, rotated.y) * SHOOT_STRENGTH + vel.linvel;
        commands.spawn((
            Sprite::from_image(asset_server.load("proj.png")),
            Transform::from_xyz(trans.translation.x, trans.translation.y, 0.0)
                .with_rotation(trans.rotation)
                .with_scale(Vec3::splat(1.0 / 5.0)),
            RigidBody::Dynamic,
            Velocity::linear(velocity),
            Sleeping::disabled(),
            Projectile {
                initial_velocity: velocity,
            },
            GravityScale(0.0),
            Collider::ball(2.0 * get_high_res_size(&window)),
            ActiveEvents::COLLISION_EVENTS,
            Ccd::enabled(),
            CollisionGroups::new(
                Group::from_bits_truncate(GROUP_PROJECTILE),
                Group::from_bits_truncate(GROUP_ASTEROID),
            ),
        ));

        proj_data.last_shoot = time.elapsed_secs();
    }
}

pub fn manage_projectiles(
    mut commands: Commands,
    transform: Query<&Transform, With<Projectile>>,
    entity: Query<Entity, With<Projectile>>,
    velocity_query: Query<(&Velocity, &Projectile), With<Projectile>>,
) {
    for (trans, ent) in transform.iter().zip(entity.iter()) {
        if trans.translation.x > (RES_WIDTH / 2) as f32 {
            commands.entity(ent).despawn();
        }
        if trans.translation.x < -((RES_WIDTH as f32) / 2.0) {
            commands.entity(ent).despawn();
        }
        if trans.translation.y > (RES_HEIGHT / 2) as f32 {
            commands.entity(ent).despawn();
        }
        if trans.translation.y < -((RES_HEIGHT as f32) / 2.0) {
            commands.entity(ent).despawn();
        }
    }

    // TODO: animation for despawn
    for ((vel, proj), ent) in velocity_query.iter().zip(entity.iter()) {
        if proj.initial_velocity.x.abs() * 0.8 > vel.linvel.x.abs()
            || proj.initial_velocity.y.abs() * 0.8 > vel.linvel.y.abs()
        {
            commands.entity(ent).despawn();
            info!(
                "despawned projecticle with speed {:?} (spawned with {:?})",
                vel.linvel, proj.initial_velocity
            );
        }
    }
}
