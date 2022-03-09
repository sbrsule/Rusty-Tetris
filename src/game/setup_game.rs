use bevy::prelude::*;
use crate::misc::{shapes::Shape, states::AppState};
use super::player::spawn_block;
use rand;

pub struct SetupGamePlugin;

pub struct NextShape(pub Shape);

impl Plugin for SetupGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NextShape(rand::random::<Shape>()))
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(setup_game)
                    .with_system(spawn_block)
            );
    }
}

fn setup_game(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(240.0, 480.0)),
                ..Default::default()
            },
            ..Default::default()
        });
    
}
