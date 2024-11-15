use bevy::prelude::*;

use rand::prelude::*;

use crate::gamelogic::{at_decision_point, check_collision, get_available_directions, get_new_position_alt, GameLogic, Horizontal, OnGameplayScreen, Player, Vertical};
use crate::gamestates::GameState;
use crate::{AnimationIndicies, AnimationTimer, ConsecutiveKills, Score};
use crate::gamelogic;
use gamelogic::Direction;
use gamelogic::get_screen_coords;
use gamelogic::get_game_board_coords;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LevelSetup), spawn_ghosts)
            .add_systems(Update, (move_ghost, check_ghost_player_collision, update_weakened_timers).run_if(in_state(GameState::Gameplay)));
    }
}

pub enum GhostPositionStatus {
    InPen,
    OutAndAbout,
    ReturningToPen,
}

pub enum GhostActionsStatus {
    Idle, // (in pen)
    LeavingPen,
    SearchingForPlayer,
    Weakened,
    RunningToPen,
    GoingIntoPen,
}

const GHOST_WEAKENED_SPEED: f32 = 3.0;
const GHOST_RUNNING_HOME: f32 = 6.0;

#[derive(Component)]
pub struct Ghost {
    //pub lifetime: Timer,
    pub direction_of_travel: Direction,

    pub body_entity: Entity,
    pub eyes_entity: Entity,

    pub speed: f32,

    pub position_status: GhostPositionStatus,
    pub actions_status: GhostActionsStatus,

    pub time_in_pen: Timer,

    pub time_weakened: Option<Timer>,

    pub name: String,

    pub base_colour: Color,

    pub last_decision_point: Vec2,
}

#[derive(Component)]
pub struct GhostBody {}

#[derive(Component)]
pub struct GhostEyes {}

pub fn spawn_ghosts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // spawn our 4 ghosts

    // ghost details holds the individual data for each of the ghosts
    struct GhostDetails {
        name: String,
        speed: f32,
        transform: Transform,
        colour: Color,
        time_in_pen: f32,
    }

    let ghost_details: [GhostDetails; 4] = [
        // red
        GhostDetails { name: String::from("Blinky"),    speed: 4.00, transform: Transform::from_xyz(-20.0, 5.0, 0.01101), colour: Color::srgb (1.0, 0.0, 0.0), time_in_pen: 1.0 },
        // cyan
        GhostDetails { name: String::from("Inky"),   speed: 4.01, transform: Transform::from_xyz(0.0, 10.0, 0.01102),  colour: Color::srgb (0.0, 1.0, 1.0), time_in_pen: 5.0 },
        // pink
        GhostDetails { name: String::from("Pinky"),   speed: 3.99, transform: Transform::from_xyz(20.0, 0.0, 0.01103),  colour: Color::srgb (1.0, 0.0, 1.0), time_in_pen: 9.0 },
        // yellow
        GhostDetails { name: String::from("Clyde"), speed: 3.98, transform: Transform::from_xyz(40.0, 15.0, 0.01104), colour: Color::srgb (1.0, 1.0, 0.0), time_in_pen: 13.0 }
    ];

    for ghost_detail in ghost_details {

        let mut eyes_transform = ghost_detail.transform.clone();
        eyes_transform.translation.z += 0.00001;

        let ghost_size = UVec2::new(22, 24);
        let ghost_anim_indicies = AnimationIndicies {first: 0, last: 4};

        let eyes_indicies = AnimationIndicies {first: 0, last: 4};

        let ghost = Ghost {
            name: ghost_detail.name,
            direction_of_travel: Direction {vertical: Vertical::Zero, horizontal: Horizontal::Left},
            speed: ghost_detail.speed,
            body_entity: commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: ghost_detail.colour,
                        ..default()
                    },
                    transform: ghost_detail.transform,
                    texture: asset_server.load("GhostBody_SpriteSheet.png"),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlases.add(TextureAtlasLayout::from_grid(
                    ghost_size,
                    1,
                    5,
                    None, None)),
                    index: ghost_anim_indicies.first,
                },
                ghost_anim_indicies,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                GhostBody {},
                OnGameplayScreen,
            )).id(),

            eyes_entity: commands.spawn((
                SpriteBundle {
                    transform: eyes_transform,
                    texture: asset_server.load("GhostEyes_SpriteSheet.png"),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlases.add(TextureAtlasLayout::from_grid(
                        ghost_size,
                        1,
                        5,
                        None, None)),
                    index: eyes_indicies.first,
                },
                eyes_indicies,
                GhostEyes {},
                OnGameplayScreen,

            )).id(),

            position_status: GhostPositionStatus::InPen, // all ghosts start in the pen
            actions_status: GhostActionsStatus::Idle,
            time_in_pen: Timer::from_seconds(ghost_detail.time_in_pen, TimerMode::Once),
            time_weakened: None,
            last_decision_point: Vec2 {x: 0.0, y: 0.0},
            base_colour: ghost_detail.colour,
        };
        
        commands.spawn((ghost, OnGameplayScreen));

    }

    // test suite for ghost functions
    /*let mut direction_test: Vec<Direction> = Vec::new();
    direction_test.push(Direction {vertical: Vertical::Up, horizontal: Horizontal::Zero});
    direction_test.push(Direction {vertical: Vertical::Down, horizontal: Horizontal::Zero});
    direction_test.push(Direction {vertical: Vertical::Zero, horizontal: Horizontal::Left});
    direction_test.push(Direction {vertical: Vertical::Zero, horizontal: Horizontal::Right});
     
    assert_eq!((false, 0), find_direction_match(&direction_test, &Direction {vertical: Vertical::Up, horizontal: Horizontal::Zero}));
    assert_eq!((false, 3), find_direction_match(&direction_test, &Direction {vertical: Vertical::Zero, horizontal: Horizontal::Right}));*/
}


fn move_ghost(
    mut ghosts: Query<&mut Ghost>,
    mut ghost_eyes_transforms: Query<(&mut Transform, &AnimationIndicies, &mut TextureAtlas), (With<GhostEyes>, Without<GhostBody>)>,
    mut ghost_body_transforms: Query<(&mut Transform, &mut Sprite), (With<GhostBody>, Without<GhostEyes>)>,
    player_transform: Query<&Transform, (With<Player>, Without<GhostBody>, Without<GhostEyes>)>,
    game_logic: Query<&GameLogic>,
    time: Res<Time>,
) {
    let player_transform = player_transform.single();
    let player_game_pos = get_game_board_coords(Vec2 {x: player_transform.translation.x, y: player_transform.translation.y});

    for mut ghost in &mut ghosts {
        if let Ok((mut eye_transform, indices, mut atlas)) = ghost_eyes_transforms.get_mut(ghost.eyes_entity) {
            if let Ok((mut body_transform, mut ghost_sprite)) = ghost_body_transforms.get_mut(ghost.body_entity) {
                
                let mut new_pos = get_game_board_coords(Vec2 {x: eye_transform.translation.x, y: eye_transform.translation.y} );

                let pen_movement = 2.0 * time.delta_seconds();
                
                // distance calculated as pythagoras
                let distance_from_player = ((player_game_pos.x - new_pos.x).abs().powi(2) + (player_game_pos.y - new_pos.y).abs().powi(2)).sqrt();

                const PEN_EXIT: Vec2 = Vec2 {x: 12.5, y:10.0};

                match ghost.actions_status {
                    GhostActionsStatus::Idle => {
                        new_pos.x += pen_movement * ghost.direction_of_travel.horizontal as i32 as f32;
                        new_pos.y += pen_movement * ghost.direction_of_travel.vertical as i32 as f32;

                        if new_pos.x > 15.0 {
                            ghost.direction_of_travel.horizontal = Horizontal::Left;
                        } else if new_pos.x < 10.0 {
                            ghost.direction_of_travel.horizontal = Horizontal::Right;
                        }
                        
                        ghost.time_in_pen.tick(time.delta());
                        if ghost.time_in_pen.just_finished() {
                            ghost.actions_status = GhostActionsStatus::LeavingPen;
                        }
                    },
                    GhostActionsStatus::LeavingPen => {
                        // check if we are in line with the pen exit x position
                        if (new_pos.x - PEN_EXIT.x).abs() < 0.1 {

                            new_pos.x = PEN_EXIT.x;
                            
                            // move towards the pen exit y position
                            let y_diff = new_pos.y - PEN_EXIT.y;
                            new_pos.y -= if y_diff > 0.0 {
                                ghost.direction_of_travel.vertical = Vertical::Up;

                                pen_movement.min(y_diff) // we are below the exit pos
                            } else {
                                ghost.direction_of_travel.vertical = Vertical::Down;
                                (-pen_movement).max(y_diff) // we are above the exit pos
                            };
                            
                            ghost.direction_of_travel.horizontal = Horizontal::Zero;

                            // check if we are now in line with the pen exit y position
                            if (new_pos.y - PEN_EXIT.y).abs() < 0.01 {
                                new_pos.y = PEN_EXIT.y;
                                // we are out of the pen
                                ghost.actions_status = GhostActionsStatus::SearchingForPlayer;
                                ghost.position_status = GhostPositionStatus::OutAndAbout;
                                
                                ghost.direction_of_travel.vertical = Vertical::Zero;
                                
                                if random() { 
                                    ghost.direction_of_travel.horizontal = Horizontal::Left;
                                } else {
                                    ghost.direction_of_travel.horizontal = Horizontal::Right;
                                }
                            }
                        } else {
                            // move towards pen exit x position
                            let x_diff = new_pos.x - PEN_EXIT.x;

                            new_pos.x -= if x_diff > 0.0 {
                                ghost.direction_of_travel.horizontal = Horizontal::Left;
                                pen_movement.min(x_diff)
                            } else {
                                ghost.direction_of_travel.horizontal = Horizontal::Right;
                                (-pen_movement).max(x_diff)
                            };

                            ghost.direction_of_travel.vertical = Vertical::Zero;
                        }

                    },
                    GhostActionsStatus::SearchingForPlayer => {
                        let movement = ghost.speed * time.delta_seconds();
                        
                        // as distance from player gets bigger, chance to do random movements decreases until some threshold
                        let chance_of_random_action = (distance_from_player / 30.0).min(0.9); // based on distance from player (or 0.9 if distance is too far)

                        new_pos = ghost_decisions(
                            movement, 
                            game_logic.single(),
                            new_pos, 
                            chance_of_random_action, 
                            &mut ghost,
                            player_game_pos
                        );

                    },
                    GhostActionsStatus::Weakened => {
                        // running away from player - ghost will aim for a position that is directly opposite from the position of the player
                        let movement = GHOST_WEAKENED_SPEED * time.delta_seconds();

                        // this is a position directly opposite the player position
                        let ghost_pos_aim = Vec2::new((2.0 * new_pos.x) - player_game_pos.x, (2.0 * new_pos.y) - player_game_pos.y);

                        new_pos = ghost_decisions(
                            movement,
                            game_logic.single(),
                            new_pos,
                            0.2,
                            &mut ghost,
                            ghost_pos_aim
                        );

                    },
                    GhostActionsStatus::RunningToPen => {
                        let movement = GHOST_RUNNING_HOME * time.delta_seconds();

                        new_pos = ghost_decisions(
                            movement,
                            game_logic.single(),
                            new_pos,
                            0.0,
                            &mut ghost,
                            PEN_EXIT
                        );

                        // return the ghost to the pen if the ghost has reached the PEN_EXIT
                        let diff = (new_pos - PEN_EXIT).abs();
                        if diff.x < 0.3 && diff.y < 0.3 {
                            ghost.actions_status = GhostActionsStatus::GoingIntoPen;
                            new_pos = PEN_EXIT;
                        }
                    },
                    GhostActionsStatus::GoingIntoPen => {
                        // move back into the pen - then go back to normal and idle

                        const PEN_CENTER: Vec2 = Vec2 { x: 12.5, y: 13.0 };

                        new_pos.x = PEN_CENTER.x;
                        ghost.direction_of_travel.vertical = Vertical::Down;
                        ghost.direction_of_travel.horizontal = Horizontal::Zero;

                        let y_diff = PEN_CENTER.y - new_pos.y;

                        new_pos.y += pen_movement.min(y_diff); // move either the pen movement or the y diff whichever is smaller

                        
                        // check if we are now in line with the pen exit y position
                        if (new_pos.y - PEN_CENTER.y).abs() < 0.01 {
                            new_pos.y = PEN_CENTER.y;
                            // we are now in the center of the pen
                            ghost.actions_status = GhostActionsStatus::Idle;
                            ghost.position_status = GhostPositionStatus::InPen;
                            
                            ghost.direction_of_travel.vertical = Vertical::Zero;

                            // regenerate the ghost body
                            ghost_sprite.color = ghost.base_colour;
                            ghost.time_in_pen = Timer::from_seconds(5.0, TimerMode::Once);
                            
                            if random() { 
                                ghost.direction_of_travel.horizontal = Horizontal::Left;
                            } else {
                                ghost.direction_of_travel.horizontal = Horizontal::Right;
                            }
                        }
                    }
                }



                // update eyes direction
                let sprite_index =
                    
                    if ghost.direction_of_travel.horizontal ==      Horizontal::Right { 0 } // right
                    else if ghost.direction_of_travel.horizontal == Horizontal::Left  { 1 } // left
                    else if ghost.direction_of_travel.vertical ==   Vertical::Up      { 2 } // up
                    else if ghost.direction_of_travel.vertical ==   Vertical::Down    { 3 } // down
                    else { 4 }; // no direction
                
                // if sprite index is valid, update the sprite index
                if sprite_index >= indices.first && sprite_index <= indices.last {
                    atlas.index = sprite_index;
                }

                // update transforms for both eyes and body
                let screen_pos = get_screen_coords(new_pos.x, new_pos.y);
                eye_transform.translation.x = screen_pos.x;
                eye_transform.translation.y = screen_pos.y;

                body_transform.translation.x = eye_transform.translation.x;
                body_transform.translation.y = eye_transform.translation.y;
            }
        }
    }
}

fn ghost_decisions(
    movement_amount: f32,
    game_logic: &GameLogic,
    mut current_pos: Vec2,
    chance_of_random: f32,
    ghost: &mut Ghost,
    ghost_pos_aim: Vec2,
) -> Vec2 {
    // check if we are at an intersection to make a decision
    // otherwise just continue in the direction we were going before (no need to change anything)

    let rounded_new_pos = current_pos.round();
    
    // for each "decision point" only handle them once (each ghost will record which one it has handled most recently)
    if (rounded_new_pos.x as i32 != ghost.last_decision_point.x as i32 || 
        rounded_new_pos.y as i32 != ghost.last_decision_point.y as i32) &&
        at_decision_point(current_pos, ghost.direction_of_travel) {

        let available_directions = get_available_directions(current_pos, ghost.direction_of_travel, game_logic);

        // make sure there are directions in the list
        if available_directions.is_empty() == false {
            
            let mut decision = 0;

            // if there is more than 1 available direction
            if available_directions.len() != 1 {

                //let chance_of_random_action = (distance_from_player / 30.0).min(0.9); // based on distance from player (or 0.9 if distance is too far)
                
                if thread_rng().gen_bool(chance_of_random as f64) {
                    
                    // choose a random direction
                    decision = thread_rng().gen_range(0..available_directions.len());

                } else {

                    decision = ghost_to_position(ghost_pos_aim, current_pos, &available_directions);
                }
            }

            // change direction to the one in the decision
            let has_changed = available_directions[decision].vertical != ghost.direction_of_travel.vertical ||
                                    available_directions[decision].horizontal != ghost.direction_of_travel.horizontal;

            ghost.direction_of_travel = available_directions[decision];

            if has_changed {
                if let Horizontal::Zero = ghost.direction_of_travel.horizontal {
                    // we are now moving vertically
                    // snap our position to the rounded x value
                    current_pos.x = current_pos.x.round();
                } else if let Vertical::Zero = ghost.direction_of_travel.vertical {
                    // we are now moving horizontally
                    // snap our position to the rounded y value
                    current_pos.y = current_pos.y.round();
                }
            }
            
            ghost.last_decision_point = rounded_new_pos;
        }
    }
    
    // try to move in the current direction of travel
    let new_pos = get_new_position_alt(game_logic, current_pos, ghost.direction_of_travel, movement_amount).0;

    return new_pos;
}

fn check_ghost_player_collision(
    mut ghost_bodys: Query<(&Transform, &mut Sprite), (With<GhostBody>, Without<Player>)>,
    player_transform: Query<&Transform, (With<Player>, Without<GhostBody>)>,
    mut ghosts: Query<&mut Ghost>,
    mut game_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
    mut consecutive_kills: ResMut<ConsecutiveKills>,
) {
    let player_transform = player_transform.single();
    let player_rect = Rect::from_center_size(Vec2 {x: player_transform.translation.x, y: player_transform.translation.y}, Vec2 {x: 21.0, y: 21.0});

    //let player_pos = Vec2 {x: player_tranform.translation.x, y: player_tranform.translation.y};

    for mut ghost in &mut ghosts {
        // get the transform associated with this ghost
        if let Ok((ghost_transform, mut ghost_sprite)) = ghost_bodys.get_mut(ghost.body_entity) {
            if check_collision(
                player_rect,
                Rect::from_center_size(
                    Vec2 {x: ghost_transform.translation.x, y: ghost_transform.translation.y},
                    Vec2 {x: 21.0, y: 21.0}
                )
            ) {
                // collision detected
                match ghost.actions_status {
                    GhostActionsStatus::Weakened => {
                        ghost.actions_status = GhostActionsStatus::RunningToPen;
                        ghost.position_status = GhostPositionStatus::ReturningToPen;
                        // set ghost body colour to be transparent
                        ghost_sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.0);

                        ghost.time_weakened = None;

                        // considered putting a cap on this but normal operation should really only let 
                        //  the player get to consecutive kills equal to number of ghosts - it could cause a funny, non-game-breaking glitch otherwise
                        // TLDR; no cap intentionally
                        consecutive_kills.0 += 1;
                        score.0 += 100 * consecutive_kills.0;
                    },
                    GhostActionsStatus::RunningToPen => {
                        // ghost is running to pen - do nothing
                        // if we wanted we could do something here - stall the ghost while it runs home? etc.
                    },
                    GhostActionsStatus::SearchingForPlayer => {
                        // all other times - touching a ghost means lose a life
                        game_state.set(GameState::LoseLife);
                    },
                    _ => {

                    }
                }
            }
        }
    }
}

fn update_weakened_timers(
    mut ghosts: Query<&mut Ghost>,
    mut ghost_bodys: Query<&mut Sprite, (With<GhostBody>, Without<Player>)>,
    time: Res<Time>,
) {
    for mut ghost in ghosts.iter_mut() {

        match &mut ghost.time_weakened {
            Some(timer) => {
                timer.tick(time.delta());
            
                if timer.finished() {
                    // change this ghost back to normal
                    ghost.actions_status = GhostActionsStatus::SearchingForPlayer;

                    if let Ok(mut body_sprite) = ghost_bodys.get_mut(ghost.body_entity) {
                        body_sprite.color = ghost.base_colour;
                    }

                    ghost.time_weakened = None;
                }
            },
            None => {},
        }
    }
}

fn find_direction_match(available_directions: &Vec<Direction>, search: &Direction) -> (bool, usize) {
    let mut found_index = 0;

    let mut decision_index = 0;

    // now try to match the preferred direction with the available directions
    for available_dir in available_directions {
        // check for a match
        if search.horizontal == available_dir.horizontal && search.vertical == available_dir.vertical {
            // its a match - choose this direction
            found_index = decision_index;
            break;
        }

        decision_index += 1;
    }
    
    (decision_index == available_directions.len(), found_index)
}

fn ghost_to_position(position_aim: Vec2, current_position: Vec2, available_directions: &Vec<Direction>) -> usize {
    let mut decision = 0;
    
    // go towards position_aim
    // find max difference between vertical and horizontal and decide to go that direction
    //let mut preferred_direction = ghost.direction_of_travel;
    let mut preferred_horizontal =  Direction {horizontal: Horizontal::Zero, vertical: Vertical::Zero};
    let mut preferred_vertical = Direction {horizontal: Horizontal::Zero, vertical: Vertical::Zero};

    let hor_diff = position_aim.x - current_position.x;
    let ver_diff = position_aim.y - current_position.y;
    
    
    // setup the horizontal direction
    preferred_horizontal.horizontal = if hor_diff >= 0.0 {
        // player is to the right, go right
        Horizontal::Right
    } else {
        // player is to the left, go left
        Horizontal::Left
    };

    // setup the vertical direction
    preferred_vertical.vertical = if ver_diff >= 0.0 {
        // player is below, go down
        Vertical::Down
    } else {
        // player is above, go up
        Vertical::Up
    };

    let (preferred_direction, second_preferred_direction) = if hor_diff.abs() > ver_diff.abs() {
        // go in a horizontal direction
        (preferred_horizontal, preferred_vertical)
    } else {
        // go in a vertical direction
        (preferred_vertical, preferred_horizontal)
    };

    let (mut direction_not_found, mut decision_index) = find_direction_match(available_directions, &preferred_direction);
    if !direction_not_found {
        decision = decision_index;
    } else {
        (direction_not_found, decision_index) = find_direction_match(available_directions, &second_preferred_direction);
        if !direction_not_found {
            decision = decision_index;
        } else {
            // neither choice is any good, choose at random
            if available_directions.len() > 1 {
                decision = thread_rng().gen_range(0..available_directions.len());
            }
        }
    }

    return decision;
}