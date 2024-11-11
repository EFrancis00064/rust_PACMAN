use bevy::prelude::*;
//use bevy_inspector_egui::egui::Ui;

use crate::{gamelogic::OnGameplayScreen, gamestates::despawn_screen, GameState, LivesLeft, Score};

#[derive(Component)]
pub struct OnLevelCompleteScreen;

#[derive(Component, Deref, DerefMut)]
pub struct HeartLife(pub i32);

#[derive(Component)]
pub struct HeartContainer;

#[derive(Component)]
pub struct ScoreText;

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(OnEnter(GameState::LevelSetup), spawn_hearts_ui)
            .add_systems(Update, update_score_ui)
            .add_systems(OnExit(GameState::LoseLife), update_lives_ui)
            .add_systems(OnEnter(GameState::LevelComplete), (despawn_screen::<OnGameplayScreen>, setup_level_complete, add_life_ui).chain())
            .add_systems(OnExit(GameState::LevelComplete), despawn_screen::<OnLevelCompleteScreen>);
    }
}

fn spawn_game_ui(
    mut commands: Commands,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    //width: Val::Px(410.0), //Val::Percent(100.0),
                    width: Val::Percent(100.0),
                
                    height: Val::Px(20.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                z_index: ZIndex::Local(1),
                ..default()
            },
        ))
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
            commands.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::End,
                        //width: Val::Percent(20.0),
                        height: Val::Px(20.0),
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                },
                HeartContainer,
            ));
        });
}

fn spawn_hearts_ui(
    heart_container: Query<Entity, With<HeartContainer>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.entity(heart_container.single()).with_children( | container | {
        for i in 0..3 {
            let icon = asset_server.load("Heart.png");
            container.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(20.0),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },
                HeartLife(i),
            ));
        }
    });
}


fn setup_level_complete(
    mut commands: Commands,
    mut lives_left: ResMut<LivesLeft>,
) {
    // increase lives left by 1
    lives_left.0 += 1;
    if lives_left.0 > 5 {
        lives_left.0 = 5;
    }

    // show the screen
    commands.spawn(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OnLevelCompleteScreen,
        )
    ).with_children(|commands| {
        commands.spawn(
            TextBundle {
                text: Text::from_section(
                    "Level Complete!\n\nPress start",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    }
                )
                .with_justify(JustifyText::Center),
                ..default()
            },
        );
    });
}

fn update_score_ui(mut texts: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
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
        info!("Heart life entity: {:?} {:?} {:?}", heart_life.0, lives_left.0, heart_entity);
        if heart_life.0 >= lives_left.0 {
            commands.entity(heart_entity).despawn();
        }
    }
}

fn add_life_ui(
    heart_container: Query<Entity, With<HeartContainer>>,
    hearts: Query<&HeartLife>,
    mut commands: Commands,
    lives_left: Res<LivesLeft>,
    asset_server: Res<AssetServer>,
) {
    if hearts.iter().count() < 5 {
        let icon = asset_server.load("Heart.png");
        commands.entity(heart_container.single()).with_children( |container | {
            container.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(20.0),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },
                HeartLife(lives_left.0 - 1),
            ));
        });
    }
}