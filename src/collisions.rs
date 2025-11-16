use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    asteroids::Asteroid, get_high_res_size, player::Player, shooting::Projectile, ui::GameState,
};

pub const GROUP_PLAYER: u32 = 0b0001;
pub const GROUP_PROJECTILE: u32 = 0b0010;
pub const GROUP_ASTEROID: u32 = 0b0100;

#[derive(Resource)]
pub struct Stats {
    pub score: u32,
    pub health: f32,
}

pub fn collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    q_player: Query<Entity, With<Player>>,
    q_projectile: Query<Entity, With<Projectile>>,
    q_asteroid: Query<Entity, With<Asteroid>>,
    q2_asteroid: Query<(&Asteroid, &Transform, &Velocity)>,
    q_vel: Query<&Velocity>,
    mut stats: ResMut<Stats>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            let is_player1 = q_player.get(*entity1).is_ok();
            let is_player2 = q_player.get(*entity2).is_ok();

            let is_projectile1 = q_projectile.get(*entity1).is_ok();
            let is_projectile2 = q_projectile.get(*entity2).is_ok();

            let is_asteroid1 = q_asteroid.get(*entity1).is_ok();
            let is_asteroid2 = q_asteroid.get(*entity2).is_ok();

            if (is_projectile1 && is_asteroid2) || (is_projectile2 && is_asteroid1) {
                // animation
                let mut ast_size = 0.0;
                let mut location: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                let mut _velocity: Vec2 = Vec2::new(0.0, 0.0);
                if let Ok((ast, trans, vel)) = q2_asteroid.get(*entity1) {
                    stats.score += ast.score;
                    ast_size = ast.scale;
                    location = trans.translation;
                    _velocity = vel.linvel;
                } else if let Ok((ast, trans, vel)) = q2_asteroid.get(*entity2) {
                    stats.score += ast.score;
                    ast_size = ast.scale;
                    location = trans.translation;
                    _velocity = vel.linvel;
                }
                if is_asteroid1 {
                    commands.entity(*entity1).despawn();
                } else if is_asteroid2 {
                    commands.entity(*entity2).despawn();
                }
                if ast_size == 0.0 {
                    continue;
                }
                {
                    let mut rng = rand::rng();
                    let ratio: f32 = 0.3 + rng.random::<f32>() * 0.4;

                    let ast_size1 = ast_size * ratio;
                    let ast_size2 = ast_size * (1.0 - ratio);

                    if ast_size1 < 0.15 || ast_size2 < 0.15 {
                        continue;
                    }

                    let projectile_vel = if let Ok(proj_vel) = q_vel.get(*entity1) {
                        proj_vel.linvel * 0.3
                    } else if let Ok(proj_vel) = q_vel.get(*entity2) {
                        proj_vel.linvel * 0.3
                    } else {
                        Vec2::ZERO
                    };

                    let mut rng = rand::rng();
                    let angle1 = rng.random_range(0.0..std::f32::consts::TAU);
                    let angle2 = rng.random_range(0.0..std::f32::consts::TAU);
                    
                    let separation = 50.0;
                    let perp_vel1 = Vec2::new(angle1.cos(), angle1.sin()) * separation * 0.3;
                    let perp_vel2 = Vec2::new(angle2.cos(), angle2.sin()) * separation * 0.3;

                    spawn_asteroid(
                        &mut commands,
                        &asset_server,
                        location,
                        projectile_vel + perp_vel1,
                        0.0,
                        ast_size1,
                        &window,
                    );
                    spawn_asteroid(
                        &mut commands,
                        &asset_server,
                        location,
                        projectile_vel + perp_vel2,
                        0.0,
                        ast_size2,
                        &window,
                    );
                }
                if is_projectile1 && q_projectile.get(*entity1).is_ok() {
                    commands.entity(*entity1).despawn();
                } else if is_projectile2 && q_projectile.get(*entity2).is_ok() {
                    commands.entity(*entity2).despawn();
                }
                info!("Projectile hit asteroid!");
            }

            if (is_player1 && is_asteroid2) || (is_player2 && is_asteroid1) {
                stats.health -= 1.0;
                info!("Player hit asteroid!");
                if stats.health <= 0.0 {
                    next_state.set(GameState::GameOver);
                }
            }
        }
    }
}

fn spawn_asteroid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
    linvel: Vec2,
    angvel: f32,
    scale: f32,
    window: &Single<&Window>,
) {
    let score: u32 = {
        let m = (2.0_f32 - 4.0_f32) / (0.6_f32 - 0.3_f32);
        let b = 4.0_f32 - m * 0.3_f32;
        (m * scale + b).floor() as u32
    };

    commands.spawn((
        Sprite::from_image(asset_server.load("asteroids/1.png")),
        Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::splat(scale / 40.0)),
        Velocity {
            linvel: Vec2 {
                x: linvel.x,
                y: linvel.y,
            },
            angvel: angvel,
        },
        GravityScale(0.0),
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        },
        Sleeping::disabled(),
        RigidBody::Dynamic,
        Collider::ball(500.0 * scale * get_high_res_size(window)),
        // Collider::ball(0.5 * scale),
        ActiveEvents::COLLISION_EVENTS,
        Ccd::enabled(),
        Asteroid { scale, score },
        CollisionGroups::new(
            Group::from_bits_truncate(GROUP_ASTEROID),
            Group::from_bits_truncate(GROUP_PLAYER | GROUP_PROJECTILE | GROUP_ASTEROID),
        ),
    ));
}
