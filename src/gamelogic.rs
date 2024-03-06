use bevy::prelude::*;

use crate::Score;

#[derive(Clone, Copy)]
enum BlockType {
    Wall,
    Path,
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
            [P, P, P, P, P, P, P, P, P, W, W, W, W, W, W, W, W, P, P, P, P, P, P, P, P, P], // r13
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
                            0.1),
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

    let mut movement_amount = player.speed * time.delta_seconds();

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

    // no input - continue in the same direction as before
    //let mut new_direction = player.direction_of_travel;

    // check if is it possible to move in the pressed direction
    let new_pos = get_new_position(game_logic,
        get_game_board_coords(Vec2{x: transform.translation.x, y: transform.translation.y}),
        pressed_direction, movement_amount);

    if (pressed_direction.horizontal != 0.0 || pressed_direction.vertical != 0.0) && new_pos.1 {
        // valid new position
        info!("Valid new position {:?},{:?}", new_pos.0.x, new_pos.0.y);

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
        let new_pos2 = get_new_position(game_logic,
            get_game_board_coords(Vec2{x: transform.translation.x, y: transform.translation.y}),
            player.direction_of_travel, movement_amount);

        if new_pos2.1 {
            // it is possible
            let screen_pos = get_screen_coords(new_pos2.0.x, new_pos2.0.y);
            transform.translation.x = screen_pos.x;
            transform.translation.y = screen_pos.y;
        } else {
            // stop moving
            player.direction_of_travel.horizontal = 0.0;
            player.direction_of_travel.vertical = 0.0;
        }
    }
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
            score.0 += 1.0;
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
    let mut new_pos = current_pos;
    new_pos.x += direction.horizontal * distance;
    new_pos.y += direction.vertical * distance;

    let block_size = Vec2 {x: 0.9, y: 0.9};

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