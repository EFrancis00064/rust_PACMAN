use bevy::{prelude::*, ui::update};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ghost::GhostPlugin;
use ui::GameUI;
use gamelogic::{Direction, GameLogicPlugin, Horizontal, Player, Vertical};

mod ghost;
mod ui;
mod gamelogic;



#[derive(Resource)]
pub struct Score(pub i32);

#[derive(Resource)]
pub struct CurrentColour(f32);

#[derive(Component)]
struct AnimationIndicies {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct MultiColoured;

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
        //.add_plugins(
        //    WorldInspectorPlugin::default(),
        //)
        .add_plugins((GhostPlugin, GameUI, GameLogicPlugin))
        .insert_resource(Score(0))
        .insert_resource(CurrentColour(0.0))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, update_multi_colours)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let camera = Camera2dBundle::default();

    commands.spawn(camera);

    let background_texture = asset_server.load("Background_single.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(410.0, 455.0)),
                color: Color::Rgba{red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0},

                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -10.0, 0.0),
                ..default()
            },
            texture: background_texture,
            ..default()
        },
        MultiColoured,
    ));

    

    let animation_indicies = AnimationIndicies {first: 0, last: 4};

    let mut pac_sprite = TextureAtlasSprite ::new(animation_indicies.first);
    pac_sprite.custom_size = Some(Vec2::new(21.0, 20.0)); // had to do this because the sprite was showing one pixel row too many (first row of next frame)

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(
                TextureAtlas::from_grid(
                    asset_server.load("Pacman_SpriteSheet.png"),
                    Vec2::new(21.0, 21.0),
                    1, 5, None, None
                )),
            //sprite: TextureAtlasSprite ::new(animation_indicies.first),
            sprite: pac_sprite,
            transform: Transform::from_xyz(0.0, -40.0, 0.01),

            ..default()
        },
        animation_indicies,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player { speed: 6.0, direction_of_travel: Direction {vertical: Vertical::Zero, horizontal: Horizontal::Zero} },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(410.0, 455.0)), // same size and position as the background
                color: Color::Rgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 },

                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -10.0, 0.05),
                ..default()
            },
            texture: asset_server.load("warp_tunnels.png"),
            ..default()
        },
        MultiColoured,
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

fn update_multi_colours (
    time: Res<Time>,
    mut query: Query<&mut Sprite, With<MultiColoured>>,
    mut current_colour_index: ResMut<CurrentColour>,
) {

    let update_amount = time.delta_seconds() / 2.0;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    // current_colour_index goes from 0 to 3:
    // 0    -    1    -    2    -    3 
    //  r--, g++ |g--, b++ | b--, r++  

    // change the tuple into the actual value we are looking for
    //let mut current_colour_index = current_colour_index.0;

    current_colour_index.0 += update_amount;
    if current_colour_index.0 >= 3.0 {
        current_colour_index.0 = 0.0;
    }

    let decimal = current_colour_index.0 - current_colour_index.0.floor();

    if current_colour_index.0 >= 0.0 && current_colour_index.0 < 1.0 {
        // r--, g++
        r = 1.0 - decimal;
        g = decimal;
    } else if current_colour_index.0 >= 1.0 && current_colour_index.0 < 2.0 {
        // g--, b++
        g = 1.0 - decimal;
        b = decimal;
    } else if current_colour_index.0 >= 2.0 && current_colour_index.0 < 3.0 {
        // b--, r++
        b = 1.0 - decimal;
        r = decimal;
    }

    let sprite_colour = Color::rgb(r, g, b);

    //info!("Updating current colour: {:?} {:?}", current_colour_index.0, sprite_colour);

    for mut sprite in &mut query {
        sprite.color = sprite_colour;
    }
}