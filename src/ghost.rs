use bevy::prelude::*;

use crate::{Score, Player};


pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_ghost, ghost_lifetime));
    }
}

#[derive(Component)]
pub struct Ghost {
    pub lifetime: Timer,
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

    let player_transform = player.single();
    if score.0 >= 10.0 {
        score.0 -= 10.0;
        info!("Spent $10 on a ghost, remaining money: ${:?}", score.0);

        let texture = asset_server.load("BasicGhost.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Ghost {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

fn ghost_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut ghosts: Query<(Entity, &mut Ghost)>,
    mut score: ResMut<Score>,
) {
    for (ghost_entity, mut ghost) in &mut ghosts {
        ghost.lifetime.tick(time.delta());

        if ghost.lifetime.finished() {
            score.0 += 15.0;

            commands.entity(ghost_entity).despawn();

            info!("Ghost sold for $15! Current Money: ${:?}", score.0)
        }
    }
}