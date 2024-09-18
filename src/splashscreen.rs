use bevy::prelude::*;

use crate::gamestates::{despawn_screen, GameState};

//mod gamestates;

#[derive(Component)]
struct OnSplashScreen;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SplashScreen), splash_setup)
            .add_systems(Update, check_start_pressed.run_if(in_state(GameState::SplashScreen)))
            .add_systems(OnExit(GameState::SplashScreen), despawn_screen::<OnSplashScreen>);
    }
}

fn splash_setup(mut commands: Commands) {
    commands.spawn(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OnSplashScreen,
        )
    ).with_children(|commands| {
        commands.spawn(
            TextBundle {
                text: Text::from_section(
                    "Press start",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    }
                ),
                ..default()
            }.with_text_justify(JustifyText::Center),
        );
    });
}

fn check_start_pressed(
    mut game_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::LevelSetup);
    }
}

