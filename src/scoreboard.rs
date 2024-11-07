use std::{fs::File, io::{prelude::*, BufReader}};

use bevy::{input::{keyboard::{Key, KeyboardInput}, ButtonState}, prelude::*};

use crate::{gamestates::GameState, Score};

#[derive(Component)]
pub struct OnGameOverScreen;

#[derive(Component)]
pub struct PlayerLeaderboardEntry;

#[derive(Component)]
pub struct PassiveLeaderboardEntry;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_scoreboard);
        app.add_systems(Update, player_initials.run_if(in_state(GameState::GameOver)));
    }
}

struct LeaderboardItem {
    name: String,
    score_num: i32,
    is_current_player: bool,
}

fn setup_scoreboard(
    mut commands: Commands,
    score: Res<Score>,
) {
    // structure of leaderboard
    let mut leaderboard: Vec<LeaderboardItem> = Vec::new();
    let mut leaderboard_has_space = true;
    let mut leaderboard_lowest = -1;

    // get the leaderboard from the file
    if let Ok(file) = File::open("leaderboard.txt") {
        // file opened successfully
        let buf_reader = BufReader::new(file);

        let mut leaderboard_count = 0;

        for line in buf_reader.lines() {
            if let Ok(line_string) = line {
                let split_strings: Vec<&str> = line_string.split(':').collect();
                let mut split_strings = split_strings.iter();


                if let Some(name) = split_strings.next() { // get the first item in the split
                    if let Some(score) = split_strings.next() { // get the second item in the split
                        if let Ok(score_number) = score.parse::<i32>() { // get the score as a number from the score string

                            leaderboard.push(
                                LeaderboardItem {
                                    name: name.to_string(), score_num: score_number, is_current_player: false
                                });

                            if score_number < leaderboard_lowest || leaderboard_lowest == -1 {
                                leaderboard_lowest = score_number;
                            }
                            leaderboard_count += 1;
                        }
                    }
                }
            }
        }
        if leaderboard_count >= 10 {
            leaderboard_has_space = false;
        }
    }


    // check if the new score makes it onto the leaderboard
    let player_on_leaderboard = leaderboard_has_space || score.0 > leaderboard_lowest;

    if player_on_leaderboard {
        // insert the current player into the leaderboard

        // find the index on the leaderboard for the player
        let mut insert_index = 0;

        for leaderboard_item in &leaderboard {
            // move down the leaderboard increasing our index each time we come across a value that is larger than the current score
            if leaderboard_item.score_num > score.0 {
                insert_index += 1;
            }
        }

        leaderboard.insert(insert_index as usize, LeaderboardItem {name: "___".to_string(), score_num: score.0, is_current_player: true});
    }

    // display the leaderboard on the screen

    commands.spawn((
        NodeBundle { // overall screen obj
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        OnGameOverScreen,
    )).with_children( | screen| {
        screen.spawn(NodeBundle { // High score message / game over message
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(| gameover_message | {
            let message = if player_on_leaderboard {
                "New highscore!"
            } else {
                "Better luck next time!"
            };

            gameover_message.spawn(TextBundle {
                text: Text::from_section(
                    message,
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    }),
                ..default()
            });
            gameover_message.spawn(TextBundle {
                text: Text::from_section(
                    "Enter your initials:",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    }),
                ..default()
            });
        });

        screen.spawn(NodeBundle { // High score message / game over message
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(70.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(| leaderboard_area | {

            // now spawn the leaderboard items
            for leaderboard_item in leaderboard {

                let leaderboard_string = format!("{} {:>8}", &leaderboard_item.name, leaderboard_item.score_num); // max score will be 99,999,999

                let text = Text::from_section(
                    &leaderboard_string,
                     TextStyle {
                        font_size: 20.0,
                        color: 
                            if leaderboard_item.is_current_player {Color::linear_rgb(0.0, 0.3, 0.0)}
                            else {Color::linear_rgb(1.0, 1.0, 1.0)},
                        ..default()
                    });

                if leaderboard_item.is_current_player {
                    leaderboard_area.spawn((
                        TextBundle {
                            text,
                            background_color: BackgroundColor::from(Color::linear_rgb(1.0, 1.0, 1.0)),
                            ..default()
                        },
                        PlayerLeaderboardEntry,
                        ));
                } else {
                    leaderboard_area.spawn((
                        TextBundle {
                            text,
                            ..default()
                        },
                        PassiveLeaderboardEntry,
                    ));
                }
            }
        });
    });

}

fn player_initials(
    mut lb_player_entry: Query<&mut Text, With<PlayerLeaderboardEntry>>,
    mut event_reader_keys: EventReader<KeyboardInput>,
) {
    for ev in event_reader_keys.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        if let Some(section) = lb_player_entry.single_mut().sections.first_mut() {

            match &ev.logical_key {
                Key::Enter => {

                },
                Key::Backspace => {
                    // get the position of the last user entered character
                    // go through each of the characters from the beginning to find this
                    let mut index = 0;
                    let mut iterator = section.value.chars();
                    while index <= 2 && iterator.next() != Some('_') {
                        index += 1;
                    }

                    // check if there are any characters to backspace
                    if index > 0 {
                        section.value.replace_range(index-1..index, "_");
                    }
                },
                Key::Character(input) => {
                    if input.chars().any(|c| c.is_control() || !c.is_alphabetic() ) {
                        continue;
                    }
                    
                    section.value = section.value.replacen('_', &input.to_uppercase(), 1);
                    
                },
                _ => {}
            }
        }
    }
}