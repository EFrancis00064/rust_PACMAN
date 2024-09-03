use bevy::{prelude::*, text::TextLayoutInfo};
use bevy_inspector_egui::egui::Ui;

use crate::{gamelogic::OnGameplayScreen, Score, LivesLeft, GameState};

#[derive(Component, Deref, DerefMut)]
pub struct HeartLife(pub i32);

#[derive(Component)]
pub struct ScoreText;

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, (update_money_ui, update_lives_ui.run_if(in_state(GameState::GameStart))));
    }
}

fn spawn_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(
            NodeBundle {
                style: Style {
                    //width: Val::Px(410.0), //Val::Percent(100.0),
                    width: Val::Percent(100.0),
                
                    height: Val::Px(20.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
        )
        .with_children(|commands| {
            commands.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        justify_items: JustifyItems::Start,
                        width: Val::Percent(80.0), // I don't get this? surely if base node is 100% that should be full width
                        ..default()
                    },
                    ..default()
                },
            ).with_children(|commands| {
                commands.spawn((
                    TextBundle {
                        text: Text::from_section(
                            "Score: ",
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
            commands.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexEnd,
                        justify_items: JustifyItems::End,
                        width: Val::Percent(20.0),
                        height: Val::Px(20.0),
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                },
            )
            .with_children(|heart_node| {
                for i in 0..3 {
                    let icon = asset_server.load("Heart.png");
                    heart_node.spawn((
                        ImageBundle {
                            style: Style {
                                padding: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
                                ..default()
                            },
                            //style: Style {width: Val::Px(30.0),
                                // This takes the icons out of the flexbox flow, to be positioned exactly
                                //position_type: PositionType::Absolute,
                                // The icon will be close to the left border of the button
                                //left: Val::Px(10.0),
                            //    ..default()
                            //},
                            image: UiImage::new(icon),
                            ..default()
                        },
                        OnGameplayScreen,
                        HeartLife(i),
                    ));
                }
            });
        });
}

fn update_money_ui(mut texts: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Score: {:?}", score.0);
    }
}

fn update_lives_ui(
    hearts: Query<(Entity, &HeartLife)>,
    mut commands: Commands,
    lives_left: Res<LivesLeft>
) {
    for (heart_entity, heart_life) in hearts.iter() {
        if heart_life.0 >= lives_left.0 {
            commands.entity(heart_entity).despawn();
        }
    }
}