use bevy::prelude::*;

use crate::gamelogic::{GameLogic, Horizontal, Vertical};
use crate::{AnimationIndicies, AnimationTimer};
use crate::{gamelogic, Player, Score};
use gamelogic::Direction;
use gamelogic::get_screen_coords;
use gamelogic::get_game_board_coords;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ghosts)
            .add_systems(Update, move_ghost);
    }
}

enum GhostStatus {
    InPen,
    LeavingPen,
    SearchingForPlayer,
    RunningToPen,
}

#[derive(Component)]
pub struct Ghost {
    //pub lifetime: Timer,
    pub direction_of_travel: Direction,

    pub body_entity: Entity,
    pub eyes_entity: Entity,

    pub speed: f32,

    pub status: GhostStatus,
    pub time_in_pen: Timer,
}

#[derive(Component)]
pub struct GhostBody {}

#[derive(Component)]
pub struct GhostEyes {}

fn spawn_ghosts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // spawn our 4 ghosts

    // ghost details holds the individual data for each of the ghosts
    struct GhostDetails {
        transform: Transform,
        colour: Color,
        time_in_pen: f32,
    }

    let ghost_details: [GhostDetails; 4] = [
        GhostDetails { transform: Transform::from_xyz(-20.0, 20.0, 0.01), colour: Color::Rgba {red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0}, time_in_pen: 5.0 },
        GhostDetails { transform: Transform::from_xyz(0.0, 20.0, 0.01),   colour: Color::Rgba {red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0}, time_in_pen: 10.0 },
        GhostDetails { transform: Transform::from_xyz(20.0, 20.0, 0.01),  colour: Color::Rgba {red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0}, time_in_pen: 15.0 },
        GhostDetails { transform: Transform::from_xyz(40.0, 20.0, 0.01),  colour: Color::Rgba {red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0}, time_in_pen: 20.0 }
    ];

    for ghost_detail in ghost_details {

        let ghost_size = Vec2::new(21.0, 21.0);
        let ghost_anim_indicies = AnimationIndicies {first: 0, last: 4};

        let mut ghost_sprite = TextureAtlasSprite::new(ghost_anim_indicies.first);
        ghost_sprite.custom_size = Some(Vec2::new(21.0, 20.0));
        ghost_sprite.color = ghost_detail.colour;

        let eyes_indicies = AnimationIndicies {first: 0, last: 4};
        //let eyes_pos = Transform::from_xyz(0.0, 20.0, 0.0105);


        let ghost = Ghost {
            direction_of_travel: Direction {vertical: Vertical::Zero, horizontal: Horizontal::Left},
            speed: 1.0,
            body_entity: commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                        asset_server.load("GhostBody_SpriteSheet.png"),
                        ghost_size,
                        1,
                        5,
                        None, None)),
                    sprite: ghost_sprite,
                    transform: ghost_detail.transform,

                    ..default()
                },
                ghost_anim_indicies,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                GhostBody {},
            )).id(),

            eyes_entity: commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                        asset_server.load("GhostEyes_SpriteSheet.png"),
                        ghost_size,
                        1,
                        5,
                        None, None)),
                    sprite: TextureAtlasSprite::new(eyes_indicies.first),
                    transform: ghost_detail.transform,

                    ..default()
                },
                eyes_indicies,
                GhostEyes {},

            )).id(),

            status: GhostStatus::InPen, // all ghosts start in the pen
            time_in_pen: Timer::from_seconds(ghost_detail.time_in_pen, TimerMode::Once),
        };
        
        commands.spawn(ghost);

    }
}


fn move_ghost(
    mut ghosts: Query<&mut Ghost>,
    mut ghost_eyes_transforms: Query<(&mut Transform, &AnimationIndicies, &mut TextureAtlasSprite), (With<GhostEyes>, Without<GhostBody>)>,
    mut ghost_body_transforms: Query<&mut Transform, (With<GhostBody>, Without<GhostEyes>)>,
    time: Res<Time>,
) {
    for mut ghost in &mut ghosts {
        if let Ok((mut eye_transform, indices, mut sprite)) = ghost_eyes_transforms.get_mut(ghost.eyes_entity) {
            
            let mut new_pos = get_game_board_coords(Vec2 {x: eye_transform.translation.x, y: eye_transform.translation.y} );

            let pen_movement = 1.0 * time.delta_seconds();

            const PEN_EXIT: Vec2 = Vec2 {x: 12.5, y:10.0};

            match ghost.status {
                GhostStatus::InPen => {
                    new_pos.x += pen_movement * ghost.direction_of_travel.horizontal as i32 as f32;
                    new_pos.y += pen_movement * ghost.direction_of_travel.vertical as i32 as f32;

                    if new_pos.x > 15.0 {
                        ghost.direction_of_travel.horizontal = Horizontal::Left;
                    } else if new_pos.x < 10.0 {
                        ghost.direction_of_travel.horizontal = Horizontal::Right;
                    }
                    
                    ghost.time_in_pen.tick(time.delta());
                    if ghost.time_in_pen.just_finished() {
                        ghost.status = GhostStatus::LeavingPen;
                    }
                },
                GhostStatus::LeavingPen => {
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
                            ghost.status = GhostStatus::SearchingForPlayer;
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
                GhostStatus::SearchingForPlayer => {
                    let movement = ghost.speed * time.delta_seconds();

                    // check if we are at an intersection to make a decision
                    // otherwise just continue in the direction we were going before (no need to change anything)
                    
                },
                GhostStatus::RunningToPen => {}
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
                sprite.index = sprite_index;
            }

            // update transforms for both eyes and body
            let screen_pos = get_screen_coords(new_pos.x, new_pos.y);
            eye_transform.translation.x = screen_pos.x;
            eye_transform.translation.y = screen_pos.y;

            if let Ok(mut body_transform) = ghost_body_transforms.get_mut(ghost.body_entity) {
                body_transform.translation.x = eye_transform.translation.x;
                body_transform.translation.y = eye_transform.translation.y;
            }
        }
    }
}

