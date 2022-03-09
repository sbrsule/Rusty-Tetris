use bevy::prelude::*;
use rand::Rng;
use crate::misc::states::AppState;
use crate::misc::directions::Direction;
use crate::misc::shapes::Shape;

use super::collision::{CollisionUpdate, VerticalFlag};
use super::movement::{LastVerticalUpdate, LastHorizontalUpdate};
use super::setup_game::NextShape;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(VerticalDirection(Some(Direction::Down)))
            .insert_resource(HorizontalDirection(None))
            .init_resource::<Center>()
            .add_system_set(
                SystemSet::on_exit(AppState::Moving)
                    .with_system(spawn_block)
                    .with_system(reset_player)
            );
    }
}

pub struct VerticalDirection(pub Option<Direction>);
pub struct HorizontalDirection(pub Option<Direction>);

#[derive(Default)]
pub struct Center([f32; 2]);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Block;

const PLAYER_SPRITE: &str = "textures/TetrisTiles.png";

pub fn spawn_block(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut center: ResMut<Center>,
    mut next_shape: ResMut<NextShape>,
) {
    let pos: [f32; 8];
    let new_center: [f32; 2];

    let texture_atlas = TextureAtlas::from_grid(asset_server.load(PLAYER_SPRITE), Vec2::new(24.0, 24.0), 5, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    match next_shape.0 {
        Shape::IBlock => {
            pos = [12.0, 156.0, 12.0, 180.0, 12.0, 204.0, 12.0, 228.0,];
            new_center = [12.0, 204.0];
        }
        Shape::JBlock => {
            pos = [12.0, 180.0, 36.0, 180.0, 36.0, 204.0, 36.0, 228.0,];
            new_center = [36.0, 204.0];
        }
        Shape::SBlock => {
            pos = [-36.0, 204.0, -12.0, 204.0, -12.0, 228.0, 12.0, 228.0,];
            new_center = [-12.0, 228.0];
        }
        Shape::ZBlock => {
            pos = [-36.0, 228.0, -12.0, 228.0, -12.0, 204.0, 12.0, 204.0,];
            new_center = [-12.0, 228.0];
        }
        Shape::TBlock => {
            pos = [-36.0, 204.0, -12.0, 204.0, -12.0, 228.0, 12.0, 204.0,];
            new_center = [-12.0, 228.0];
        }
        Shape::OBlock => {
            pos = [-12.0, 228.0, -12.0, 204.0, 12.0, 228.0, 12.0, 204.0,];
            new_center = [12.0, 228.0];
        }
        Shape::LBlock => {
            pos = [36.0, 180.0, 12.0, 180.0, 12.0, 204.0, 12.0, 228.0,];
            new_center = [36.0, 204.0];
        }
    }

    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0..6) * 5;

    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_xyz(pos[0], pos[1], 0.0),
        sprite: TextureAtlasSprite::new(index),
        ..Default::default()
        })
        .insert(Player);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[2], pos[3], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(Player);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[4], pos[5], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        }) 
        .insert(Player);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[6], pos[7], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(Player);
    center.0 = new_center;  
    next_shape.0 = rand::random::<Shape>(); 
    println!("{:?}" , app_state);
    app_state.set(AppState::Moving).unwrap_or_default();
}

fn reset_player(
    mut vertical_direction: ResMut<VerticalDirection>,
    mut horizontal_direction: ResMut<HorizontalDirection>,
    mut vertical_flag: ResMut<VerticalFlag>,
    mut collision_update: ResMut<CollisionUpdate>,
    mut vertical_update: ResMut<LastVerticalUpdate>,
    mut horizontal_update: ResMut<LastHorizontalUpdate>,
    time: Res<Time>,
) {
    vertical_direction.0 = Some(Direction::Down);
    horizontal_direction.0 = None;
    vertical_flag.0 = false;
    collision_update.0 = time.seconds_since_startup();
    vertical_update.0 = time.seconds_since_startup();
    horizontal_update.0 = time.seconds_since_startup();
}