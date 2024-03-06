use bevy::prelude::*;

use crate::Score;

#[derive(Component)]
pub struct ScoreText;

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_money_ui);
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Money",
                        TextStyle {
                            font_size: 20.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                ScoreText,
            ));
        });
}

fn update_money_ui(mut texts: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Score: {:?}", score.0);
    }
}