use bevy::prelude::*;

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
/*impl BlockCell {
    fn default() -> BlockCell {
        BlockCell {
            exit_path_count: 0,
            block_type: BlockType::Wall,
            block_reward: BlockReward::Nothing,
        }
    }
}*/

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
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(5.0, 5.0)),
                            
                            ..default()
                        },
                        transform: Transform::from_xyz((((col_index as f32) * 15.0) - (SCREEN_WIDTH_PX / 2.0)) + 17.5,// - (SCREEN_WIDTH_PX / 2.0), 
                        //(SCREEN_HEIGHT_PX - (row_index - (SCREEN_HEIGHT_PX / 2.0))) - 10.0,
                        (((((BOARD_HEIGHT as u32 - 1) - row_index) as f32) * 15.0) - (SCREEN_HEIGHT_PX / 2.0)) + 5.0,
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


/*fn get_block_cell(pos: Vec2) {

}

    
    fn count_point_tokens_left() {
        // go through each of the items in the gameboard array
        let mut count: u32;
        for row in game_blocks {
            for cell in row {
                if cell.block_type == PointToken {
                    count += 1;
                }
            }
        }
        count
    }*/
