// use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
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

#[derive(Component)]
struct AnimationIndicies {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "PACMAN in Bevy and Rust".into(),
                        resolution: (450.0, 519.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
        )
        /*.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )*/
        .add_plugins((GhostPlugin, GameUI))
        .insert_resource(Money(100.0))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6))) // this doesnt seem to be working
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, animate_sprite))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let camera = Camera2dBundle::default();
    /*camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 600.0,
        min_height: 600.0,
    };*/

    commands.spawn(camera);
    /*commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GREEN),
        },
        ..default()
    });*/

    let background_texture = asset_server.load("Background.png");

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(450.0, 499.0)),

                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -10.0, 0.0),
                ..default()
            },
            texture: background_texture,
            ..default()
        }
    );

    let texture_handle = asset_server.load("Pacman_SpriteSheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(21.0, 21.0), 1, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indicies = AnimationIndicies {first: 0, last: 4};

    commands.spawn((
        SpriteSheetBundle {
        
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite ::new(animation_indicies.first),
        
            ..default()
        },
        animation_indicies,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player { speed: 50.0 },
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

        // update the rotation of the sprite based on the direction it is moving
        // create an angle from the direction:
        // direction.horizontal = 1 = 0 degrees
        // direction.horizontal = -1 = 180 degrees
        // direction.vertical = 1 = 90 degrees
        // direction.vertical = -1 = 270 degrees

        // 0 - ((direction horizontal x 90 degrees) - 90)
        // 360 - (direction vertical x 90 degrees) + 180
        if direction.horizontal != 0.0 || direction.vertical != 0.0 {

            let rotation_h = 
                if direction.horizontal != 0.0 {
                    0.0 - ((direction.horizontal * 90.0) - 90.0)
                } else {
                    0.0
                };
            let rotation_v = 
                if direction.vertical != 0.0 {
                    360.0 - ((direction.vertical * 90.0) + 180.0)
                } else {
                    0.0
                };
            let rotation_degrees = rotation_h + rotation_v;

            
            //transform.look_to(Vec3::new(direction.horizontal, direction.vertical, 0.0), Vec3::Y);

            info!("Directions: {:?} {:?} rotation degrees: {:?} + {:?} = {:?}", direction.horizontal, direction.vertical, rotation_h, rotation_v, rotation_degrees);

            transform.rotation = Quat::from_rotation_z(f32::to_radians(rotation_degrees));
            //transform.rotate_z(f32::to_radians(rotation_degrees));
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndicies,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indicies, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indicies.last {
                indicies.first
            } else {
                sprite.index + 1
            };
        }
    }
}