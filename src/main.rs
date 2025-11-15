// ...existing code...

use std::time::Duration;

use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::ecs::event::EventReader;
use bevy::window::WindowResized;
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::*;

mod player;
use player::*;

mod camera;
use camera::*;

mod asteroids;
use asteroids::*;

mod collisions;
use collisions::*;

/// In-game resolution width.
const RES_WIDTH: u32 = 320;

/// In-game resolution height.
const RES_HEIGHT: u32 = 180;

/// Render layers for high-resolution rendering.
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space Game".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(AudioPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 20.0,
                    ..default()
                },
                refresh_interval: Duration::from_millis(50),
                ..default()
            },
        })
        .add_plugins(RapierDebugRenderPlugin {
            default_collider_debug: ColliderDebug::AlwaysRender,
            enabled: true,
            mode: DebugRenderMode::all(),
            ..default()
        })
        .add_systems(
            Startup,
            (setup_background, setup_camera, setup_player, init_timer),
        )
        .add_systems(
            Update,
            (
                control_player,
                fit_canvas,
                keep_player,
                manage_projectiles,
                manage_asteroids,
                shoot,
                collision_system,
            ),
        )
        .run();
}

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct Canvas;

fn setup_background(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Sprite::from_image(asset_server.load("bg.png")),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_canvas(
    mut resize_messages: EventReader<WindowResized>,
    mut projection: Single<&mut Projection, With<OuterCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    for window_resized in resize_messages.read() {
        let h_scale = window_resized.width / RES_WIDTH as f32;
        let v_scale = window_resized.height / RES_HEIGHT as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}

fn get_high_res_size(window: &Window) -> f32 {
    let h_scale = window.width() / RES_WIDTH as f32;
    let v_scale = window.height() / RES_HEIGHT  as f32;
    h_scale.min(v_scale).round()
}