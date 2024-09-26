use bevy::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ghost::GhostPlugin;
use ui::GameUI;
use gamelogic::GameLogicPlugin;
use splashscreen::SplashPlugin;
use gamestates::GameState;

mod ghost;
mod ui;
mod gamelogic;
mod splashscreen;
mod gamestates;

#[derive(Resource)]
pub struct Score(pub i32);

#[derive(Resource)]
pub struct ConsecutiveKills(pub i32);

#[derive(Resource)]
pub struct CurrentColour(f32);

#[derive(Resource)]
pub struct LivesLeft(i32);

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
        .add_plugins((SplashPlugin, GhostPlugin, GameUI, GameLogicPlugin))
        .insert_resource(Score(0))
        .insert_resource(CurrentColour(0.0))
        .insert_resource(LivesLeft(0))
        .insert_resource(ConsecutiveKills(0))
        .init_state::<GameState>() // in later versions of bevy this is init_state
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, update_multi_colours)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    let camera = Camera2dBundle::default();

    commands.spawn(camera);

}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndicies,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (indicies, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indicies.last {
                indicies.first
            } else {
                atlas.index + 1
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

    let sprite_colour = Color::srgb(r, g, b);

    //info!("Updating current colour: {:?} {:?}", current_colour_index.0, sprite_colour);

    for mut sprite in &mut query {
        sprite.color = sprite_colour;
    }
}