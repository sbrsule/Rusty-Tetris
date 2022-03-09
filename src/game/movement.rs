use bevy::prelude::*;

use crate::misc::{states::AppState, directions::Direction};

use super::{player::{VerticalDirection, HorizontalDirection, Player}, collision::VerticalFlag};

const VERTICAL_TICK: f64 = 0.5;
const HORIZONTAL_TICK: f64 = 0.15;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LastVerticalUpdate(0.0))
            .insert_resource(LastHorizontalUpdate(0.0))
            .add_system_set(
                SystemSet::on_update(AppState::Moving)
                    .before("collision")
                    .with_system(input_direction_system)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Moving)
                    .after("collision")
                    .with_system(movement_system)
            );
    }
}

fn input_direction_system(
    mut keys: ResMut<Input<KeyCode>>,
    mut vertical_direction: ResMut<VerticalDirection>,
    vertical_flag: Res<VerticalFlag>,
    mut horizontal_direction: ResMut<HorizontalDirection>,
) {
    if keys.just_pressed(KeyCode::A) || keys.pressed(KeyCode::A) {
        horizontal_direction.0 = Some(Direction::Left);
        if keys.just_pressed(KeyCode::A) {
            keys.reset(KeyCode::A);
        }
    } else if keys.just_pressed(KeyCode::D) || keys.pressed(KeyCode::D) {
        horizontal_direction.0 = Some(Direction::Right);
        if keys.just_pressed(KeyCode::D) {
            keys.reset(KeyCode::D);
        }
    } else if keys.just_pressed(KeyCode::S) || keys.pressed(KeyCode::S) {
        vertical_direction.0 = Some(Direction::SuperDown);
        if keys.just_pressed(KeyCode::S) {
            keys.reset(KeyCode::S);
        }
    } 
}

#[derive(Default)]
pub struct LastVerticalUpdate(pub f64);

#[derive(Component)]
pub struct LastHorizontalUpdate(pub f64);

fn movement_system(
    mut player_query: Query<(&Player, &mut Transform)>,
    mut vertical_direction: ResMut<VerticalDirection>,
    mut horizontal_direction: ResMut<HorizontalDirection>,
    mut vertical_update: ResMut<LastVerticalUpdate>,
    mut horizontal_update: ResMut<LastHorizontalUpdate>,
    time: Res<Time>,
) {
    if time.seconds_since_startup() - vertical_update.0 >= VERTICAL_TICK 
        && vertical_direction.0 == Some(Direction::Down) {
            for (_player, mut transform) in player_query.iter_mut() {
                transform.translation.y -= 24.0;
            }

            vertical_update.0 = time.seconds_since_startup();
    } else if time.seconds_since_startup() - vertical_update.0 >= HORIZONTAL_TICK
        && vertical_direction.0 == Some(Direction::SuperDown) {
            for (_player, mut transform) in player_query.iter_mut() {
                transform.translation.y -= 24.0;
            }

            vertical_update.0 = time.seconds_since_startup();
            vertical_direction.0 = Some(Direction::Down);
        }

    if time.seconds_since_startup() - horizontal_update.0 >= HORIZONTAL_TICK {
        match horizontal_direction.0 {
            Some(Direction::Left) => {
                for (_player, mut transform) in player_query.iter_mut() {
                    transform.translation.x -= 24.0;
                }

                horizontal_direction.0 = None;
            }
            Some(Direction::Right) => {
                for (_player, mut transform) in player_query.iter_mut() {
                    transform.translation.x += 24.0;
                }

                horizontal_direction.0 = None;
            }
            _ => (),
        }

        horizontal_update.0 = time.seconds_since_startup();
    } 
}