use bevy::prelude::*;
use crate::{
    game::{
        player::{Player, Block},
    },
    game::player::{VerticalDirection, HorizontalDirection},
    misc::{directions::Direction, states::AppState},
};

pub struct CollisionPlugin;

const HORIZONTAL_TICK: f64 = 0.15;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<VerticalFlag>()
            .init_resource::<CollisionUpdate>()
            .add_system_set(
                SystemSet::on_update(AppState::Moving) 
                    .label("collision")
                    .with_system(vertical_collision_system)
                    .with_system(horizontal_collision_system)
            );
    }
}

#[derive(Default)]
pub struct VerticalFlag(pub bool);

#[derive(Default)]
pub struct CollisionUpdate(pub f64);

fn vertical_collision_system(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut vertical_flag: ResMut<VerticalFlag>, 
    mut collision_update: ResMut<CollisionUpdate>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut entity_player_query: Query<Entity, With<Player>>,
    block_query: Query<(&Block, &Transform)>,
    mut vertical_direction: ResMut<VerticalDirection>,
) {
        for (_player, player_transform) in player_query.iter_mut() {
            if  player_transform.translation.y <= -216.0 {
                vertical_flag.0 = true;
                break;
            }
            for (_block, block_transform) in block_query.iter() {
                if player_transform.translation.x == block_transform.translation.x && player_transform.translation.y == block_transform.translation.y + 24.0 {
                    vertical_flag.0 = true;
                    break;
                }
            }
        }
        
        if !vertical_flag.0 {
            collision_update.0 = time.seconds_since_startup();
        } else {
            for (_player, _transform) in player_query.iter_mut() {
                vertical_direction.0 = None;
            }
        }
    if time.seconds_since_startup() - collision_update.0 > HORIZONTAL_TICK && vertical_flag.0 {
        vertical_direction.0 = None;
        for entity in entity_player_query.iter_mut() {
            commands.entity(entity)
                .remove::<Player>()
                .insert(Block);
        }
        app_state.set(AppState::Still).unwrap_or_default();
        println!("{:?}", app_state);
    }
}

fn horizontal_collision_system(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    mut horizontal_direction: ResMut<HorizontalDirection>,
) {
    for (_player, transform) in player_query.iter_mut() {
        if transform.translation.x <= -96.0  && horizontal_direction.0 == Some(Direction::Left) {
            horizontal_direction.0 = None;
        } else if transform.translation.x >= 86.0 && horizontal_direction.0 == Some(Direction::Right) {
            horizontal_direction.0 = None;
        }
    }
}