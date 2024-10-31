use std::{fs::File, io::{prelude::*, BufReader}};

use bevy::prelude::*;

use crate::{gamestates::GameState, Score};

#[derive(Component)]
pub struct OnGameOverScreen;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_scoreboard);
    }
}

fn setup_scoreboard(
    mut commands: Commands,
    score: Res<Score>,
) {
    info!("Running setup_scoreboard");

    let mut leaderboard: Vec<(String, String)> = Vec::new();
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

                            leaderboard.push((name.to_string(), score.to_string()));

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

    //if let Ok((last_score, last)leaderboard.iter().last()

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
            for (mut leaderboard_name, leaderboard_score) in leaderboard {

                // concatenate the leaderboard name and string with some spaces as separators
                leaderboard_name.push_str("  ");
                leaderboard_name.push_str(&leaderboard_score);

                leaderboard_area.spawn(TextBundle {
                    text: Text::from_section(
                        &leaderboard_name,
                        TextStyle {
                            font_size: 20.0,
                            ..default()
                        }),
                    ..default()
                });
            }
        });
    });

}