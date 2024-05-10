use bevy::prelude::*;

use crate::Score;

#[derive(Clone, Copy)]
enum BlockType {
    Wall,
    Path,
    Warp(u8,u8),
}

#[derive(Clone, Copy)]
enum BlockReward {
    Nothing,
    PointToken,
    GhostWeaknessToken,
    //Fruit(String),
}

#[derive(Clone, Copy)]
pub struct BlockCell {
    exit_path_count: u8,
    block_type: BlockType,
    block_reward: BlockReward,
}

#[derive(Clone, Copy)]
pub struct Direction {
    pub vertical: f32,
    pub horizontal: f32
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub direction_of_travel: Direction,
}

#[derive(Component)]
pub struct PointTokenEntity;

#[derive(Component)]
pub struct GameLogic {
    pub game_blocks: [[BlockCell; BOARD_WIDTH]; BOARD_HEIGHT],
}

const SCREEN_WIDTH_PX: f32 = 410.0;
const SCREEN_HEIGHT_PX: f32 = 450.0;

const BOARD_WIDTH: usize = 26;
const BOARD_HEIGHT: usize = 29;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_gameboard);
        app.add_systems(Update, (player_movement, check_player_points_collision));
    }
}
fn setup_gameboard(mut commands: Commands) {
    
    let game_logic: GameLogic = GameLogic {
    // initialise all the game blocks to default values (as a wall)
        //game_blocks: [[BlockCell::default(); BOARD_HEIGHT]; BOARD_WIDTH], //[[BlockCell {exit_path_count: 0, block_type: BlockType::Wall, block_reward:BlockReward::Nothing}; 20]; 24];

        game_blocks: {
            const W: BlockCell = BlockCell {exit_path_count: 0, block_type: BlockType::Wall, block_reward: BlockReward::Nothing};
            const P: BlockCell = BlockCell {exit_path_count: 2, block_type: BlockType::Path, block_reward: BlockReward::PointToken};
            const Q: BlockCell = BlockCell {exit_path_count: 2, block_type: BlockType::Path, block_reward: BlockReward::Nothing}; // a path but with no point token
            const X: BlockCell = BlockCell {exit_path_count: 2, block_type: BlockType::Warp(25,13), block_reward: BlockReward::Nothing}; // X warps to Y
            const Y: BlockCell = BlockCell {exit_path_count: 2, block_type: BlockType::Warp(0, 13), block_reward: BlockReward::Nothing}; // Y warps to X

           [[P, P, P, P, P, P, P, P, P, P, P, P, W, W, P, P, P, P, P, P, P, P, P, P, P, P], // r0
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, P], // r1
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, P], // r2
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, P], // r3
            [P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P], // r4
            [P, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, P], // r5
            [P, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, P], // r6
            [P, P, P, P, P, P, W, W, P, P, P, P, W, W, P, P, P, P, W, W, P, P, P, P, P, P], // r7
            [W, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, W], // r8
            [W, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, W], // r9
            [W, W, W, W, W, P, W, W, P, P, P, P, P, P, P, P, P, P, W, W, P, W, W, W, W, W], // r10
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W], // r11
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W], // r12
            [X, Q, P, P, P, P, P, P, P, W, W, W, W, W, W, W, W, P, P, P, P, P, P, P, Q, Y], // r13
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W], // r14
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W], // r15
            [W, W, W, W, W, P, W, W, P, P, P, P, P, P, P, P, P, P, W, W, P, W, W, W, W, W], // r16
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W], // r17
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W], // r18
            [P, P, P, P, P, P, P, P, P, P, P, P, W, W, P, P, P, P, P, P, P, P, P, P, P, P], // r19
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, W, W, P], // r20
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, W, W, P], // r21
            [P, P, P, W, W, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, W, W, P, P, P], // r22
            [W, W, P, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, P, W, W], // r23
            [W, W, P, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, P, W, W], // r24
            [P, P, P, P, P, P, W, W, P, P, P, P, W, W, P, P, P, P, W, W, P, P, P, P, P, P], // r25
            [P, W, W, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, W, W, P], // r26
            [P, W, W, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, W, W, P], // r27
            [P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P]] // r28
        }
    };

    // spawn the point token pattern based on the gameblocks
    let mut row_index: u32 = 0;
    let mut col_index: u32 = 0;

    for block_row in game_logic.game_blocks {
        for block_cell in block_row {
            match block_cell.block_reward {
                BlockReward::PointToken => {
                    // block cell is a point token type
                    // spawn a point token in the bevy commands
                    let screen_coords = get_screen_coords(col_index as f32, row_index as f32);

                    commands.spawn((SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(5.0, 5.0)),
                            
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            screen_coords.x,
                            screen_coords.y,
                            0.0105), // this should be below the warp tunnel z but above the character z level
                        ..default()
                    },
                    PointTokenEntity));
                },
                _ => (),
            }

            col_index += 1;
        }
        row_index += 1;
        col_index = 0;
    }
    
    commands.spawn(
        game_logic
    );

}

fn player_movement(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    game_logic_query: Query<&GameLogic>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // extract the transform and the player objects from the query
    let player_query_single = player_query.single_mut();
    let mut transform = player_query_single.0;
    let mut player = player_query_single.1;

    let game_logic = game_logic_query.single();

    let movement_amount = player.speed * time.delta_seconds();

    let mut pressed_direction = Direction {vertical : 0.0, horizontal : 0.0};

    // convert keycode into a direction
    if input.pressed(KeyCode::Up) {
        pressed_direction.vertical += -1.0;
    }
    if input.pressed(KeyCode::Down) {
        pressed_direction.vertical += 1.0;
    }
    if input.pressed(KeyCode::Right) {
        pressed_direction.horizontal += 1.0;
    }
    if input.pressed(KeyCode::Left) {
        pressed_direction.horizontal += -1.0;
    }


    let current_pos = get_game_board_coords(Vec2{x: transform.translation.x, y: transform.translation.y});

    let mut potential_pos: (Vec2, bool) = (Vec2{x:0.0, y:0.0}, true);

    const TURNING_THRESHOLD : f32 = 0.2;

    // compare pressed direction to current direction of travel (favor vertical direction changes)
    if pressed_direction.vertical != 0.0 && pressed_direction.vertical != player.direction_of_travel.vertical {
        // we are only looking at the vertical direction here
        // we have pressed the opposite direction to which we were moving before, try to move back the way we have come
        //  or we are changing direction down a hallway 
        //  or we have been sat not moving at all

        let vertical_direction = Direction{vertical: pressed_direction.vertical, horizontal: 0.0};

        let mut skip_get_pos = false;

        // if the player was not already going vertical (they are turning from a horizontal direction of travel to turn a 90 degree corner)
        if player.direction_of_travel.vertical == 0.0 {
            // check if they are close enough to the center coordinate of a cell (only allow turning down a corridor if we are close enough to it)
            let rounded_pos = current_pos.round();
            // don't let the turn happen if we are too far away from the center position -- ASSUMPTION: ALL CORRIDOORS ARE ONLY 1 BLOCK WIDE

            let mut min_x = -TURNING_THRESHOLD;
            let mut max_x = TURNING_THRESHOLD;
            let diff = current_pos.x - rounded_pos.x;

            if player.direction_of_travel.horizontal == -1.0 {
                min_x = 0.0;
            } else if player.direction_of_travel.horizontal == 1.0 {
                max_x = 0.0;
            }

            skip_get_pos = diff < min_x || diff > max_x;
            potential_pos.1 = skip_get_pos;

            //info!("Pressed vertical, was going horizontal  Skip:{:?}  Diff:{:?}  MinX:{:?}  MaxX:{:?}", skip_get_pos, diff, min_x, max_x);
        }

        // if no collision detected yet, get the new position
        if skip_get_pos == false {
            potential_pos = get_new_position_alt(game_logic, current_pos, vertical_direction, movement_amount);
        }
        
        // check if there was any collision detected at all in this direction
        if potential_pos.1 == false {

            // set new player direction of travel
            player.direction_of_travel = vertical_direction;

            // snap horizontal position to the nearest whole number
            potential_pos.0.x = potential_pos.0.x.round();
        }
    }
    
    // now check the horizontal direction if the vertical was not fruitful
    if potential_pos.1 && pressed_direction.horizontal != 0.0 && pressed_direction.horizontal != player.direction_of_travel.horizontal {
    
        let horizontal_direction = Direction{vertical: 0.0, horizontal: pressed_direction.horizontal};
        
        let mut skip_get_pos = false;

        // if the player was not already going horizontal (they are turning from a vertical direction of travel to turn a 90 degree corner)
        if player.direction_of_travel.horizontal == 0.0 {
            // check if they are close enough to the center coordinate of a cell (only allow turning down a corridor if we are close enough to it)
            let rounded_pos = current_pos.round();
            // don't let the turn happen if we are too far away from the center position -- ASSUMPTION: ALL CORRIDOORS ARE ONLY 1 BLOCK WIDE

            let mut min_y = -TURNING_THRESHOLD;
            let mut max_y = TURNING_THRESHOLD;
            let diff = current_pos.y - rounded_pos.y;

            // up is -1 down is +1 (opposite to the game board coords)
            if player.direction_of_travel.vertical == -1.0 {
                min_y = 0.0;
            } else if player.direction_of_travel.vertical == 1.0 {
                max_y = 0.0;
            }

            skip_get_pos = diff < min_y || diff > max_y;
            potential_pos.1 = skip_get_pos;
        }

        // if no collision detected yet, get the new position
        if skip_get_pos == false {
            potential_pos = get_new_position_alt(game_logic, current_pos, horizontal_direction, movement_amount);
        }

        // check if there was any collision detected at all in this direction
        if potential_pos.1 == false {
            // set new player direction of travel
            player.direction_of_travel = horizontal_direction;

            // snap vertical position to the nearest whole number
            potential_pos.0.y = potential_pos.0.y.round();
        }
    }

    if potential_pos.1 {
        // neither of the player inputs directions are valid or are maybe are not present
        // for now then we will try to continue in the same direction as we were moving before

        potential_pos = get_new_position_alt(game_logic, current_pos,
            player.direction_of_travel, movement_amount);
    }

    if potential_pos.1 {
        // we cannot move anymore - stop moving now!
        player.direction_of_travel.horizontal = 0.0;
        player.direction_of_travel.vertical = 0.0;
        
        // snap to the block position so that we are directly on the path
        let snapped_grid_pos = Vec2 {x: current_pos.x.round(), y: current_pos.y.round()};
        let new_screen_pos = get_screen_coords(snapped_grid_pos.x, snapped_grid_pos.y);
        transform.translation.x = new_screen_pos.x;
        transform.translation.y = new_screen_pos.y;

    } else {
        // we have found a valid new position, move to this position
        let screen_pos = get_screen_coords(potential_pos.0.x, potential_pos.0.y);
        transform.translation.x = screen_pos.x;
        transform.translation.y = screen_pos.y;

        // make sure the player sprite is facing the current direction

        // update the rotation of the sprite based on the direction it is moving
        // create an angle from the direction:
        // direction.horizontal = 1 = 0 degrees
        // direction.horizontal = -1 = 180 degrees
        // direction.vertical = -1 = 90 degrees
        // direction.vertical = 1 = 270 degrees

        // 0 - ((direction horizontal x 90 degrees) - 90)
        // 360 - (direction vertical x 90 degrees) + 180
        if player.direction_of_travel.horizontal != 0.0 || player.direction_of_travel.vertical != 0.0 {

            let rotation_h = 
                if player.direction_of_travel.horizontal != 0.0 {
                    0.0 - ((player.direction_of_travel.horizontal * 90.0) - 90.0)
                } else {
                    0.0
                };
            let rotation_v = 
                if player.direction_of_travel.vertical != 0.0 {
                    (player.direction_of_travel.vertical * 90.0) + 180.0
                } else {
                    0.0
                };
            let rotation_degrees = rotation_h + rotation_v;

            transform.rotation = Quat::from_rotation_z(f32::to_radians(rotation_degrees));

            if rotation_degrees == 180.0 {
                transform.rotate_x(std::f32::consts::PI); // flip along the x axis 180 degrees (so we are now seeing the 'back' of the image)
                // - imagine it is a page of paper where the ink has seeped through perfectly
            }
        }
    }

    /*
    // no input - continue in the same direction as before
    //let mut new_direction = player.direction_of_travel;

    // check if is it possible to move in the pressed direction
    let new_pos = get_new_position_alt(game_logic,
        get_game_board_coords(Vec2{x: transform.translation.x, y: transform.translation.y}),
        pressed_direction, movement_amount);

    if (pressed_direction.horizontal != 0.0 || pressed_direction.vertical != 0.0) && new_pos.1 == false {
        // valid new position
        //info!("Valid new position {:?},{:?}", new_pos.0.x, new_pos.0.y);

        player.direction_of_travel = pressed_direction;
        let screen_pos = get_screen_coords(new_pos.0.x, new_pos.0.y);
        transform.translation.x = screen_pos.x;
        transform.translation.y = screen_pos.y;
    
        // update the rotation of the sprite based on the direction it is moving
        // create an angle from the direction:
        // direction.horizontal = 1 = 0 degrees
        // direction.horizontal = -1 = 180 degrees
        // direction.vertical = -1 = 90 degrees
        // direction.vertical = 1 = 270 degrees

        // 0 - ((direction horizontal x 90 degrees) - 90)
        // 360 - (direction vertical x 90 degrees) + 180
        if pressed_direction.horizontal != 0.0 || pressed_direction.vertical != 0.0 {

            let rotation_h = 
                if pressed_direction.horizontal != 0.0 {
                    0.0 - ((pressed_direction.horizontal * 90.0) - 90.0)
                } else {
                    0.0
                };
            let rotation_v = 
                if pressed_direction.vertical != 0.0 {
                    (pressed_direction.vertical * 90.0) + 180.0
                } else {
                    0.0
                };
            let rotation_degrees = rotation_h + rotation_v;

            transform.rotation = Quat::from_rotation_z(f32::to_radians(rotation_degrees));

            if rotation_degrees == 180.0 {
                transform.rotate_x(std::f32::consts::PI); // flip along the x axis 180 degrees (so we are now seeing the 'back' of the image)
                // - imagine it is a page of paper where the ink has seeped through perfectly
            }
        }
    } else {
        // continue moving if possible in the same direction as we were before
        let grid_pos = get_game_board_coords(Vec2 {x: transform.translation.x, y: transform.translation.y});
        let new_pos2 = get_new_position_alt(game_logic, grid_pos, player.direction_of_travel, movement_amount);

        if new_pos2.1 == false {
            // it is possible
            let screen_pos = get_screen_coords(new_pos2.0.x, new_pos2.0.y);
            transform.translation.x = screen_pos.x;
            transform.translation.y = screen_pos.y;
        } else {
            // stop moving
            player.direction_of_travel.horizontal = 0.0;
            player.direction_of_travel.vertical = 0.0;
            
            // snap to the block position so that we are directly on the path
            let snapped_grid_pos = Vec2 {x: grid_pos.x.round(), y: grid_pos.y.round()};
            let new_screen_pos = get_screen_coords(snapped_grid_pos.x, snapped_grid_pos.y);
            transform.translation.x = new_screen_pos.x;
            transform.translation.y = new_screen_pos.y;
        }
    }*/
}

fn check_player_points_collision(
    player_query: Query<&Transform, With<Player>>,
    point_tokens_query: Query<(&Sprite, &Transform, Entity), With<PointTokenEntity>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    let player = player_query.single();
    let player_rect = Rect::from_center_size(Vec2 {x: player.translation.x, y: player.translation.y}, Vec2 {x: 21.0, y: 21.0});

    //let player_bounding_rect = Rect::from_center_size(Vec2 {x: player.translation.x, y: player.translation.y}, Vec2 {x: 15.0, y: 15.0});
    for (point_token_sprite, point_token_transform, point_token_entity) in point_tokens_query.iter() {
        // check each object for a collision on the transforms

        let size = 
        match point_token_sprite.custom_size {
            Some(size) => size,
            None => Vec2 {x: 1.0, y: 1.0}
        };

        if check_collision(
            Rect::from_center_size(
                Vec2 {x: point_token_transform.translation.x, y: point_token_transform.translation.y},
                size), 
            player_rect)
        {
            score.0 += 10;
            // collision occured - remove the entity and add the associated points to the score
            commands.entity(point_token_entity).despawn();
        }
    }
}

/*
 * Get screen coords of the given col and row (gameboard position)
 */
fn get_screen_coords(col_index: f32, row_index: f32) -> Vec2 {
    Vec2 {
        x: ((col_index * 15.0) - (SCREEN_WIDTH_PX / 2.0)) + 17.5,
        y: ((((BOARD_HEIGHT as f32 - 1.0) - row_index) * 15.0) - (SCREEN_HEIGHT_PX / 2.0)) + 5.0
    }
}

/*
 * Get the gameboard coordinates of the given screen position
 */
fn get_game_board_coords(pos: Vec2) -> Vec2 {
    Vec2 {
        x: ((pos.x - 17.5) + (SCREEN_WIDTH_PX / 2.0)) / 15.0,
        y: (BOARD_HEIGHT as f32 - 1.0) - (((pos.y - 5.0) + (SCREEN_HEIGHT_PX / 2.0)) / 15.0)
    }
}

/*
 * Count the number of remaining point tokens
 */
fn count_point_tokens_left(game_logic: GameLogic) -> u32 {
    // go through each of the items in the gameboard array
    let mut count: u32 = 0;

    for row in game_logic.game_blocks {
        for cell in row {
            match cell.block_reward {
                BlockReward::PointToken => {
                    count += 1;
                },
                _ => (),
            }
        }
    }
    count
}

/*
 * Get a new position going the given distance in the given direction starting from the current_pos
 * All coordinates are in gamelogic coordinates (not screen coords)
 */
fn get_new_position(game_logic: &GameLogic, current_pos: Vec2, direction: Direction, distance: f32) -> (Vec2, bool) {
    let next_snapped_pos = 
        Vec2 {x: current_pos.x.round(), y: current_pos.y.round()};

    let mut valid_pos = current_pos;
    
    // if the snapped position is close enough to the current position
    if (next_snapped_pos.x - current_pos.x).abs() < distance {valid_pos.x = next_snapped_pos.x;}
    if (next_snapped_pos.y - current_pos.y).abs() < distance {valid_pos.y = next_snapped_pos.y;}


    let mut new_pos = valid_pos;
    new_pos.x += direction.horizontal * distance;
    new_pos.y += direction.vertical * distance;

    let block_size = Vec2 {x: 0.99, y: 0.99};

    let collision_rect = Rect::from_center_size(new_pos, block_size);

    // check if the position is valid
    // go through all 9 of the surrounding tiles, for any that are walls, check if the position is valid
    let index_x = new_pos.x.round() as i32;
    let index_y = new_pos.y.round() as i32;

    let mut col = index_x - 1;
    let mut row = index_y - 1;

    let mut valid = true;

    'outer: while row <= index_y + 1 {
        while col <= index_x + 1 {

            let mut is_wall = false;

            // check if the current row and col are valid
            if row < 0 || col < 0 || 
                row >= BOARD_HEIGHT as i32 ||
                col >= BOARD_WIDTH as i32 {
                // it is a wall
                is_wall = true;
            } else {
                match game_logic.game_blocks[row as usize][col as usize].block_type {
                    BlockType::Wall => { is_wall = true; },
                    _ => ()
                }
            }

            if is_wall {
                // check collision with the new_pos
                if check_collision(collision_rect, Rect::from_center_size(Vec2{x: col as f32, y: row as f32}, collision_rect.size())) {
                    //info!("Hit wall {:?},{:?}", col, row);
                    new_pos = current_pos;
                    valid = false;
                    break 'outer;
                }
            }

            col += 1;
        }
        col = index_x - 1;
        row += 1;
    }


    (new_pos, valid)
}

/*
 * Does a basic check of the collision of 2 rectangles
 * - Can fail to detect a collision if object2 is smaller than object1
 * Returns true if collision
 */
fn check_collision(object1: Rect, object2: Rect) -> bool {

    // if left of obj1 is inside left and right of obj2
    (((object1.min.x >= object2.min.x) && (object1.min.x <= object2.max.x)) ||

    // if right of obj1 is inside left and right of obj2
     ((object1.max.x >= object2.min.x) && (object1.max.x <= object2.max.x))) && 

     // if top of obj1 is inside top and bottom of obj2
    (((object1.min.y >= object2.min.y) && (object1.min.y <= object2.max.y)) ||

    // if bottom of obj1 is inside top and bottom of obj2
     ((object1.max.y >= object2.min.y) && (object1.max.y <= object2.max.y)))
}

/**
 * Attempt to get a new position given a current position, a direction and a distance
 * Returns the new postion and if there was a collision
 */
fn get_new_position_alt(game_logic: &GameLogic, current_pos: Vec2, direction: Direction, distance: f32) -> (Vec2, bool) {

    let mut return_val = (current_pos, false);
    
    let mut new_pos = current_pos;
    new_pos.x += direction.horizontal * distance;
    new_pos.y += direction.vertical * distance;
    let collision_rect = Rect::from_center_size(new_pos, Vec2 {x: 1.0, y: 1.0});

    let mut check_for_collision = false;

    // get the cell coords of the cell that we are aiming for
    let mut cell_to_check = new_pos.round();

    cell_to_check.x += direction.horizontal;
    cell_to_check.y += direction.vertical;

    // verify if cell to check is out of bounds
    if cell_to_check.x >= 0.0 && cell_to_check.x < (BOARD_WIDTH as f32) &&
       cell_to_check.y >= 0.0 && cell_to_check.y < (BOARD_HEIGHT as f32) {
        // cell coords are valid
        let cell = game_logic.game_blocks[cell_to_check.y as usize][cell_to_check.x as usize];

        match cell.block_type {
            BlockType::Wall => {
                // check collision of entity with this cell
                check_for_collision = true;
            },
            BlockType::Warp(x, y) => {
                info!("Warp block intercepted {:?},{:?}", x, y);

                // warp the character to the x and y values
                new_pos = Vec2 {x: x as f32, y: y as f32};
            },
            _ => ()
        }
    } else {
        check_for_collision = true;
    }

    if check_for_collision {
        return_val.1 = check_collision(collision_rect,
                                               Rect::from_center_size(Vec2{x: cell_to_check.x, y: cell_to_check.y}, collision_rect.size()));
        // a collision occurred
        if return_val.1 {
            // set position to the nearest whole numbers in that direction
            if direction.horizontal != 0.0 {
                return_val.0.x = return_val.0.x.round();
            } else if direction.vertical != 0.0 {
                return_val.0.y = return_val.0.y.round();
            }
        }
    }

    if return_val.1 == false {
        // no collision has occurred, update the new position
        return_val.0 = new_pos;
    }

    return_val
}