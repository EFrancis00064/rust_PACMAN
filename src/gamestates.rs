use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    SplashScreen,
    LevelSetup,
    GameStart,
    Gameplay,
    LoseLife,
    LevelComplete,
    GameOver,
}

/*
SplashScreen
Game starts in SplashScreen state: waiting for the user to press "start" or space bar - when pressed go into LevelSetup state

LevelSetup
Setup all the gameboard objects on screen then move to GameStart state

GameStart
A few seconds pass before Gameplay can begin

Gameplay
All normal game processes occur in Gameplay state

LoseLife
When a player and ghost collision is detected, the game goes into LoseLife state - the ghosts and player reset but the rest stays the same (gameboard, points tokens, score etc),
if there are lives left, a few seconds pass before going back into the Gameplay state, otherwise go to GameOver state

LevelComplete
When all points tokens are eaten by the player - the level is complete, the score stays the same and the level is increased,
the complete gameboard is cleared and we go back into LevelSetup state (the level value should affect speed of both player and ghosts)

GameOver
Show game over on screen, check if the player has a highscore for the scoreboard, if so get them to enter their initials, clear score and level values and go back to splash screen
*/

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}