use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode,
    window::WindowResolution,
};

const WIDTH: f32 = 720.0;
const HEIGHT: f32 = 1280.0;
const BG: Color = Color::GRAY;
const INACTIVE_BG: Color = Color::BLACK;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: WIDTH,
                min_height: HEIGHT,
            },
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(INACTIVE_BG),
        },
        ..default()
    });

    cmds.spawn(SpriteBundle {
        sprite: Sprite {
            color: BG,
            custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
            ..default()
        },
        ..default()
    });
}
