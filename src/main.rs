// use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::{prelude::*, render::camera::ScalingMode};

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
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_ghost))
        
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

fn spawn_ghost(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a ghost, remaining money: ${:?}", money.0);

        let texture = asset_server.load("BasicGhost.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
        ));
    }
}