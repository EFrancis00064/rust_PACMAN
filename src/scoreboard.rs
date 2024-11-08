use std::{fs::{self, File}, io::{prelude::*, BufReader}};

use bevy::{input::{keyboard::{Key, KeyboardInput}, ButtonState}, prelude::*};

use crate::{gamestates::{despawn_screen, GameState}, Score};

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
        app.add_systems(OnExit(GameState::GameOver), (save_scoreboard, despawn_screen::<OnGameOverScreen>).chain());
    }
}

#[derive(Component, Clone)]
struct LeaderboardItem {
    name: String,
    score_num: i32,
    is_current_player: bool,
}

#[derive(Component, Clone)]
struct Rank(u32);

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
            background_color: Color::BLACK.into(),
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
            if player_on_leaderboard {
                gameover_message.spawn(TextBundle {
                    text: Text::from_section(
                        "Enter your initials:",
                        TextStyle {
                            font_size: 20.0,
                            ..default()
                        }),
                    ..default()
                });
            }
        });

        screen.spawn(NodeBundle { // Leaderboard
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(| leaderboard_area | {

            let mut lb_rank = 1;
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
                        leaderboard_item,
                        Rank(lb_rank),
                        ));
                } else {
                    leaderboard_area.spawn((
                        TextBundle {
                            text,
                            ..default()
                        },
                        PassiveLeaderboardEntry,
                        leaderboard_item,
                        Rank(lb_rank),
                    ));
                }

                lb_rank += 1;
            }
        });

        screen.spawn(NodeBundle { // High score message / game over message
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(| continue_message | {
            continue_message.spawn(TextBundle {
                text: Text::from_section(
                    "Press start to continue",
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    }),
                ..default()
            });
        });
    });

}

fn player_initials(
    mut lb_player_entry: Query<&mut Text, With<PlayerLeaderboardEntry>>,
    mut event_reader_keys: EventReader<KeyboardInput>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for ev in event_reader_keys.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        if let Ok(mut lb_player_entry) = lb_player_entry.get_single_mut() {
            if let Some(section) = lb_player_entry.sections.first_mut() {

                match &ev.logical_key {
                    Key::Space => {
                        // move on to continue - go back to splashscreen
                        game_state.set(GameState::SplashScreen);
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
        } else {
            // no player entry to be gotten
            // just check if the start button has been pressed
            match &ev.logical_key {
                Key::Space => {
                    game_state.set(GameState::LevelSetup);
                },
                _ => {},
            }
        }
        
    }
}

fn save_scoreboard(
    leaderboard_items: Query<(&LeaderboardItem, &Rank)>,
    player_lb_entry: Query<&Text, With<PlayerLeaderboardEntry>>,
    mut score: ResMut<Score>,
) {
    if leaderboard_items.is_empty() {
        // early quit if player isnt currently on the scoreboard
        return;
    }

    let mut leaderboard: Vec<(Rank, LeaderboardItem)> = Vec::new();
    for (lb_item, rank) in leaderboard_items.iter() {
        leaderboard.push((rank.clone(), lb_item.clone()));
    }

    // sorts by rank only (ascending)
    leaderboard.sort_by(| a,b | {
        a.0.0.cmp(&b.0.0)
    });

    // limit the leaderboard to 10
    leaderboard = leaderboard.into_iter().take(10).collect();

    let mut file_output = String::default();
    for (_, lb_item) in leaderboard {
        file_output.push_str(&format!("{}:{:?}\n",
        if lb_item.is_current_player {
            let mut player_name_string = "   ".to_string();
            if let Ok(player_name) = player_lb_entry.get_single() {
                if let Some(player_name) = player_name.sections.first() {
                    let player_name: String = player_name.value.chars().into_iter().take(3).collect();
                    player_name_string = player_name.replace("_", " ");
                }
            }
            player_name_string
        } else {
            lb_item.name
        },
        lb_item.score_num));
    }

    match fs::write("leaderboard.txt", file_output) {
        Err(_) => {
            info!("Cannot write to leaderboard!");
        },
        _ => {},
    }

    // reset score to 0
    score.0 = 0;
}