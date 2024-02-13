// use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ghost::GhostPlugin;
use ui::GameUI;

mod ghost;
mod ui;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "PACMAN in Bevy and Rust".into(),
                        resolution: (400.0, 800.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins((GhostPlugin, GameUI))
        .insert_resource(Money(100.0))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6))) // this doesnt seem to be working
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 600.0,
        min_height: 600.0,
    };

    commands.spawn(camera);
    /*commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GREEN),
        },
        ..default()
    });*/

    let texture = asset_server.load("BasicPAC.png");

    commands.spawn((
        SpriteBundle {
        /*sprite: Sprite {
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },*/
        texture,
        ..default()
        },
        Player { speed: 200.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let mut movement_amount = player.speed * time.delta_seconds();

        struct Direction {
            vertical: f32,
            horizontal: f32,
        }

        let mut direction = Direction {vertical : 0.0, horizontal : 0.0};

        // convert keycode into a direction
        if input.pressed(KeyCode::Up) {
            direction.vertical = 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.vertical = -1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.horizontal = 1.0;
        }
        if input.pressed(KeyCode::Left) {
            direction.horizontal = -1.0;
        }

        if direction.horizontal != 0.0 && direction.vertical != 0.0 {
            // if both directions are present, we are going to diagon alley - move both x and y at a reduced rate (pythagoras)
            movement_amount = ((movement_amount.powi(2)) / 2.0).sqrt();
        }
        
        transform.translation.x += direction.horizontal * movement_amount;
        transform.translation.y += direction.vertical   * movement_amount;
    }
}
