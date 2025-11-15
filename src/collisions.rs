use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{asteroids::Asteroid, player::Player, player::Projectile};

pub fn collision_system(
    mut collision_events: EventReader<CollisionEvent>,
    q_player: Query<Entity, With<Player>>,
    q_projectile: Query<Entity, With<Projectile>>,
    q_asteroid: Query<Entity, With<Asteroid>>,
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
                info!("Projectile hit asteroid!");
            }

            if (is_player1 && is_asteroid2) || (is_player2 && is_asteroid1) {
                info!("Player hit asteroid!");
            }
        }
    }
}
