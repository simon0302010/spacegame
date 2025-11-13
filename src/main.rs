use bevy::color::palettes::css::GRAY;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::ecs::event::EventReader;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::window::WindowResized;
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::*;

/// In-game resolution width.
const RES_WIDTH: u32 = 320;

/// In-game resolution height.
const RES_HEIGHT: u32 = 180;

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

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
                .set(ImagePlugin::default_nearest())
        )
        .add_plugins(AudioPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(FpsOverlayPlugin::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_camera, setup_player))
        .add_systems(Update, (control_player, fit_canvas))
        .run();
}

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct OuterCamera;

#[derive(Component)]
struct Player;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        Collider::ball(20.0),
        Player,
        PIXEL_PERFECT_LAYERS,
    ));
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // This Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // Fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // This camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2d,
        Camera {
            // Render before the "main pass" camera
            order: -1,
            target: RenderTarget::Image(image_handle.clone().into()),
            clear_color: ClearColorConfig::Custom(GRAY.into()),
            ..default()
        },
        Msaa::Off,
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    // Spawn the canvas
    commands.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));

    // The "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2d, Msaa::Off, OuterCamera, HIGH_RES_LAYERS));
}

const TURN_SPEED: f32 = 0.5;
const MAX_TURN_SPEED: f32 = 5.0;
const THRUST: f32 = 600.0;

/// Controls player
fn control_player(
    time: Res<Time>,
    mut velocity: Query<&mut Velocity, With<Player>>,
    mut transforms: Query<&mut Transform, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut vel) = velocity.single_mut() {
        if let Ok(transform) = transforms.single_mut() {
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