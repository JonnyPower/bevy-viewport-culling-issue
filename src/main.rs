use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use rand::seq::SliceRandom;

#[derive(Component)]
struct PrimaryCamera;
#[derive(Component)]
struct SecondaryCamera;

const SECONDARY_CAMERA_SIZE: u32 = 250;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_primary_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.single();
    for x in -50_i8..50_i8 {
        for y in -50_i8..50_i8 {
            let pos = Vec2::new(x as f32 * 32.0, y as f32 * 32.0);
            let textures = vec!["green.png", "pink.png", "teal.png"];
            let choice = textures[(x.abs() % 3) as usize];
            let tex_handle = asset_server.load(choice);
            commands.spawn(
                SpriteBundle {
                    texture: tex_handle,
                    transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                    ..default()
                }
            );
        }
    }
    commands.spawn((
        PrimaryCamera,
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        }
    ));
    commands.spawn((
        SecondaryCamera,
        Camera2dBundle {
            camera: Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::new(window.physical_width() - SECONDARY_CAMERA_SIZE - 20, window.physical_height() - SECONDARY_CAMERA_SIZE - 20),
                    physical_size: UVec2::splat(SECONDARY_CAMERA_SIZE),
                    ..default()
                }),
                order: 2,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 99.0).with_scale(Vec3::splat(window.height() / SECONDARY_CAMERA_SIZE as f32)),
            ..default()
        }
    ));
}

fn move_primary_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_q: Query<&mut Transform, With<PrimaryCamera>>
) {
    let mut camera = camera_q.single_mut();
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let horizontal = right as i8 - left as i8;
    camera.translation.x += horizontal as f32 * 5.0;
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let vertical = up as i8 - down as i8;
    camera.translation.y += vertical as f32 * 5.0;
}