use bevy::prelude::*;
use crate::Player;

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
        app.add_systems(Update, player_movement);
    }
}
fn setup_gameboard(mut commands: Commands) {
    
    let game_logic: GameLogic = GameLogic {
    // initialise all the game blocks to default values (as a wall)
        //game_blocks: [[BlockCell::default(); BOARD_HEIGHT]; BOARD_WIDTH], //[[BlockCell {exit_path_count: 0, block_type: BlockType::Wall, block_reward:BlockReward::Nothing}; 20]; 24];

        game_blocks: {
            const W: BlockCell = BlockCell {exit_path_count: 0, block_type: BlockType::Wall, block_reward: BlockReward::Nothing};
            const P: BlockCell = BlockCell {exit_path_count: 2, block_type: BlockType::Path, block_reward: BlockReward::PointToken};

            [[P, P, P, P, P, P, P, P, P, P, P, P, W, W, P, P, P, P, P, P, P, P, P, P, P, P],
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, P],
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, P],
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, P],
            [P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P],
            [P, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, P],
            [P, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, P],
            [P, P, P, P, P, P, W, W, P, P, P, P, W, W, P, P, P, P, W, W, P, P, P, P, P, P],
            [W, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, W],
            [W, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, P, W, W, W, W, W],
            [W, W, W, W, W, P, W, W, P, P, P, P, P, P, P, P, P, P, W, W, P, W, W, W, W, W],
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W],
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W],
            [P, P, P, P, P, P, P, P, P, W, W, W, W, W, W, W, W, P, P, P, P, P, P, P, P, P],
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W],
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W],
            [W, W, W, W, W, P, W, W, P, P, P, P, P, P, P, P, P, P, W, W, P, W, W, W, W, W],
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W],
            [W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W],
            [P, P, P, P, P, P, P, P, P, P, P, P, W, W, P, P, P, P, P, P, P, P, P, P, P, P],
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, W, W, P],
            [P, W, W, W, W, P, W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, W, W, P],
            [P, P, P, W, W, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, W, W, P, P, P],
            [W, W, P, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, P, W, W],
            [W, W, P, W, W, P, W, W, P, W, W, W, W, W, W, W, W, P, W, W, P, W, W, P, W, W],
            [P, P, P, P, P, P, W, W, P, P, P, P, W, W, P, P, P, P, W, W, P, P, P, P, P, P],
            [P, W, W, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, W, W, P],
            [P, W, W, W, W, W, W, W, W, W, W, P, W, W, P, W, W, W, W, W, W, W, W, W, W, P],
            [P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P]]
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

                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(5.0, 5.0)),
                            
                            ..default()
                        },
                        transform: Transform::from_xyz(/*
                            (((col_index as f32) * 15.0) - (SCREEN_WIDTH_PX / 2.0)) + 17.5,
                            (((((BOARD_HEIGHT as u32 - 1) - row_index) as f32) * 15.0) - (SCREEN_HEIGHT_PX / 2.0)) + 5.0,*/
                            screen_coords.x,
                            screen_coords.y,
                            0.1),
                        ..default()
                    });
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
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // extract the transform and the player objects from the query
    let player_query_single = player_query.single_mut();
    let mut transform = player_query_single.0;
    let player = player_query_single.1;

    let mut movement_amount = player.speed * time.delta_seconds();

    struct Direction {
        vertical: f32,
        horizontal: f32,
    }

    let mut direction = Direction {vertical : 0.0, horizontal : 0.0};

    // convert keycode into a direction
    if input.pressed(KeyCode::Up) {
        direction.vertical = 1.0;
    }
    if input.pressed(KeyCode::Down) {
        direction.vertical = -1.0;
    }
    if input.pressed(KeyCode::Right) {
        direction.horizontal = 1.0;
    }
    if input.pressed(KeyCode::Left) {
        direction.horizontal = -1.0;
    }

    if direction.horizontal != 0.0 && direction.vertical != 0.0 {
        // if both directions are present, we are going to diagon alley - move both x and y at a reduced rate (pythagoras)
        movement_amount = ((movement_amount.powi(2)) / 2.0).sqrt();
    }
    
    transform.translation.x += direction.horizontal * movement_amount;
    transform.translation.y += direction.vertical   * movement_amount;

    // update the rotation of the sprite based on the direction it is moving
    // create an angle from the direction:
    // direction.horizontal = 1 = 0 degrees
    // direction.horizontal = -1 = 180 degrees
    // direction.vertical = 1 = 90 degrees
    // direction.vertical = -1 = 270 degrees

    // 0 - ((direction horizontal x 90 degrees) - 90)
    // 360 - (direction vertical x 90 degrees) + 180
    if direction.horizontal != 0.0 || direction.vertical != 0.0 {

        let rotation_h = 
            if direction.horizontal != 0.0 {
                0.0 - ((direction.horizontal * 90.0) - 90.0)
            } else {
                0.0
            };
        let rotation_v = 
            if direction.vertical != 0.0 {
                360.0 - ((direction.vertical * 90.0) + 180.0)
            } else {
                0.0
            };
        let rotation_degrees = rotation_h + rotation_v;

        
        //transform.look_to(Vec3::new(direction.horizontal, direction.vertical, 0.0), Vec3::Y);

        info!("Directions: {:?} {:?} rotation degrees: {:?} + {:?} = {:?}", direction.horizontal, direction.vertical, rotation_h, rotation_v, rotation_degrees);

        transform.rotation = Quat::from_rotation_z(f32::to_radians(rotation_degrees));

        if rotation_degrees == 180.0 {
            transform.rotate_x(std::f32::consts::PI); // flip along the x axis 180 degrees (so we are now seeing the 'back' of the image)
            // - imagine it is a page of paper where the ink has seeped through perfectly
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

