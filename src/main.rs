// use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
//use bevy::input::common_conditions::input_toggle_active;/
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ghost::GhostPlugin;
use ui::GameUI;
use gamelogic::{GameLogicPlugin, Player, Direction};

mod ghost;
mod ui;
mod gamelogic;



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
                        resolution: (410.0, 470.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
        )
        /*.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )*/
        .add_plugins((GhostPlugin, GameUI, GameLogicPlugin))
        .insert_resource(Money(100.0))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6))) // this doesnt seem to be working
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
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

    let background_texture = asset_server.load("Background_single.png");

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(410.0, 455.0)),

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
            transform: Transform::from_xyz(0.0, -40.0, 0.01),

            ..default()
        },
        animation_indicies,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player { speed: 5.0, direction_of_travel: Direction {vertical: 0.0, horizontal: 0.0} },
    ));
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