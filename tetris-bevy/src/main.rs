mod instance;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};
use instance::Instance;

const WIDTH: f32 = 10.0;
const HEIGHT: f32 = 20.0;
const BG: Color = Color::GRAY;
const INACTIVE_BG: Color = Color::BLACK;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
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
