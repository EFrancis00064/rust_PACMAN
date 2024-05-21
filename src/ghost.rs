use bevy::prelude::*;

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
            .add_systems(Update, (move_ghost));
    }
}

#[derive(Component)]
pub struct Ghost {
    //pub lifetime: Timer,
    pub direction_of_travel: Direction,

    pub body_entity: Entity,
    pub eyes_entity: Entity,

    pub speed: f32,
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

    let ghost_colors: [(Transform, Color); 4] = [
        (Transform::from_xyz(-20.0, 20.0, 0.01), Color::Rgba {red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0}),
        (Transform::from_xyz(0.0, 20.0, 0.01), Color::Rgba {red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0}),
        (Transform::from_xyz(20.0, 20.0, 0.01), Color::Rgba {red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0}),
        (Transform::from_xyz(40.0, 20.0, 0.01), Color::Rgba {red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0})
    ];

    for ghost_color in ghost_colors {

        let ghost_size = Vec2::new(21.0, 21.0);
        let ghost_anim_indicies = AnimationIndicies {first: 0, last: 4};

        let mut ghost_sprite = TextureAtlasSprite::new(ghost_anim_indicies.first);
        ghost_sprite.custom_size = Some(Vec2::new(21.0, 20.0));
        ghost_sprite.color = ghost_color.1;

        info!("Ghost 'color': {:?}", ghost_color);

        let eyes_indicies = AnimationIndicies {first: 0, last: 4};
        //let eyes_pos = Transform::from_xyz(0.0, 20.0, 0.0105);


        let ghost = Ghost {
            direction_of_travel: Direction {vertical: 0.0, horizontal: 1.0},
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
                    transform: ghost_color.0,

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
                    transform: ghost_color.0,

                    ..default()
                },
                eyes_indicies,
                GhostEyes {},

            )).id(),
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

            let movement = ghost.speed * time.delta_seconds();

            let mut new_pos = get_game_board_coords(Vec2 {x: eye_transform.translation.x, y: eye_transform.translation.y} );

            // move eyes by the speed, in the current direction
            new_pos.x += movement * ghost.direction_of_travel.horizontal;
            new_pos.y += movement * ghost.direction_of_travel.vertical;

            if new_pos.x > 15.0 {
                ghost.direction_of_travel.horizontal = -1.0;
            } else if new_pos.x < 10.0 {
                ghost.direction_of_travel.horizontal = 1.0;
            }

            let sprite_index =
                if ghost.direction_of_travel.vertical == 1.0 { 2 } // up
                else if ghost.direction_of_travel.vertical == -1.0 { 3 } // down
                else if ghost.direction_of_travel.horizontal == 1.0 { 0 } // right
                else if ghost.direction_of_travel.horizontal == -1.0 { 1 } // left
                else { 4 }; // no direction
            
            // if sprite index is valid, update the sprite index
            if sprite_index >= indices.first && sprite_index <= indices.last {
                sprite.index = sprite_index;
            }

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

fn spawn_ghost(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut score: ResMut<Score>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    /*

    let player_transform = player.single();
    if score.0 >= 10 {
        score.0 -= 10;
        info!("Spent $10 on a ghost, remaining money: ${:?}", score.0);

        let texture = asset_server.load("BasicGhost.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Ghost {
                //lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }*/
}

fn ghost_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut ghosts: Query<(Entity, &mut Ghost)>,
    mut score: ResMut<Score>,
) {
    /*for (ghost_entity, mut ghost) in &mut ghosts {
        ghost.lifetime.tick(time.delta());

        if ghost.lifetime.finished() {
            score.0 += 15;

            commands.entity(ghost_entity).despawn();

            info!("Ghost sold for $15! Current Money: ${:?}", score.0)
        }
    }*/
}