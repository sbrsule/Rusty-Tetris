use bevy::{
    prelude::*, 
    input::{ElementState, keyboard::KeyboardInput},
};
use rand::{Rng, 
    distributions::{Distribution, Standard}
};
use crate::AppState;

const PLAYER_SPRITE: &str = "textures/TetrisTiles.png";
const VERTICAL_TICK: f64 = 0.5;
pub const HORIZONTAL_TICK: f64 = 0.15;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(setup_game)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Moving)
                    .with_system(eliminate_system)
            )
            .add_system_set(
                SystemSet::on_enter(AppState::Still)
                    .with_system(spawn_block)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Moving)
                    .with_system(rotation_system)
                    .with_system(input_direction_system.label("input"))
                    .with_system(hard_collision_system.label("collision").after("input"))
                    .with_system(movement_system.after("collision"))
            );
    }
}
#[derive(Component, Clone)]
struct Materials {
    player: Handle<Image>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Shape {
    IBlock,
    LBlock,
    JBlock,
    SBlock,
    ZBlock,
    TBlock,
    OBlock,
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..=6) {
            0 => Shape::IBlock,
            1 => Shape::LBlock,
            2 => Shape::JBlock,
            3 => Shape::SBlock,
            4 => Shape::ZBlock,
            5 => Shape::TBlock,
            _ => Shape::OBlock,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Direction {
    Down,
    Left,
    Right,
    Still,
}

#[allow(dead_code)]
#[derive(Component, Clone)]
pub struct Player {
    pub center: [f32; 2],
    pub shape: Shape,
    pub direction: Option<Direction>,
}

impl Player {
    fn new(center: [f32; 2], shape: Shape, direction: Option<Direction>) -> Self {
        Player {
            center,
            shape,
            direction,
        }
    }
}

#[derive(Component)]
pub struct InactiveBlock;

#[derive(Component)]
pub struct LastVerticalUpdate(f64);

#[derive(Component)]
pub struct LastHorizontalUpdate(f64);

#[derive(Component)]
pub struct SingleTick(f64);

#[derive(Component)]
pub struct Eliminate {
    time: f64,
    step: usize,
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut app_state: ResMut<State<AppState>>,
) {
    let materials = Materials{player: asset_server.load(PLAYER_SPRITE)};
    let eliminate = Eliminate{time:0.0, step: 0};

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(LastVerticalUpdate(0.0))
        .insert(LastHorizontalUpdate(0.0))
        .insert(materials.clone())
        .insert(SingleTick(0.0))
        .insert(eliminate);


    let texture_atlas = TextureAtlas::from_grid(materials.player.clone(), Vec2::new(24.0, 24.0), 5, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let (pos, center, shape, index) = random_block();
    let player = Player::new(center, shape, None);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(240.0, 480.0)),
                ..Default::default()
            },
            ..Default::default()
        });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(pos[0], pos[1], 0.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(player.clone());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[2], pos[3], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(player.clone());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[4], pos[5], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(player.clone());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[6], pos[7], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(player.clone());

    app_state.set(AppState::Moving).unwrap_or_default();
}

fn input_direction_system(
    mut keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    if keys.just_pressed(KeyCode::A) {
        for mut player in player_query.iter_mut() {
            player.direction = Some(Direction::Left);
        }
        keys.reset(KeyCode::A);
    } else if keys.pressed(KeyCode::A) {
        for mut player in player_query.iter_mut() {
            player.direction = Some(Direction::Left);
        }
    } else if keys.just_pressed(KeyCode::D) {
        for mut player in player_query.iter_mut() {
            player.direction = Some(Direction::Right);
        }
        keys.reset(KeyCode::D);
    } else if keys.pressed(KeyCode::A) {
        for mut player in player_query.iter_mut() {
            player.direction = Some(Direction::Right);
        }
    } else if keys.pressed(KeyCode::S) {
        for mut player in player_query.iter_mut() {
            player.direction = Some(Direction::Down);
        }
    } else {
        for mut player in player_query.iter_mut() {
            player.direction = None;
        }
    }
}
fn movement_system(
    mut keys: ResMut<Input<KeyCode>>,
    time: Res<Time>,
    mut last_vertical_update_query: Query<&mut LastVerticalUpdate>,
    mut last_horizontal_update_query: Query<&mut LastHorizontalUpdate>,
    mut entity_query: QuerySet<(
        QueryState<(&mut Player, &mut Transform)>,
        QueryState<(Entity, &mut Transform), With<Player>>,
    )>,
) {
    let current_time = time.seconds_since_startup();
    let mut last_vertical_update = last_vertical_update_query.single_mut();
    let mut last_horizontal_update = last_horizontal_update_query.single_mut();
    
    let horizontal_update = if current_time - last_horizontal_update.0 >= HORIZONTAL_TICK {
        true
    } else {
        false
    };
    
    let mut collision_flag = false;
    match entity_query.q0().iter_mut().next().unwrap().0.direction {
        Some(Direction::Left) => {
            for (mut player, mut transform) in entity_query.q0().iter_mut() {
                transform.translation.x -= 24.0;
                player.center = [player.center[0] - 24.0, player.center[1]];
            }
        }
        Some(Direction::Right) => {
            for (mut player, mut transform) in entity_query.q0().iter_mut() {
                transform.translation.x += 24.0;
                player.center = [player.center[0] + 24.0, player.center[1]];
            }
        }
        Some(Direction::Down) => {
            if current_time - last_horizontal_update.0 >= HORIZONTAL_TICK {
                for (mut player, mut transform) in entity_query.q0().iter_mut() {
                    transform.translation.y -= 24.0;
                    player.center = [player.center[0], player.center[1] - 24.0];
                }
                last_horizontal_update.0 = current_time;
            }
        }
        Some(Direction::Still) => (),
        None => {
            if current_time - last_vertical_update.0 >= VERTICAL_TICK {
                for (mut player, mut transform) in entity_query.q0().iter_mut() {
                    transform.translation.y -= 24.0;
                    player.center = [player.center[0], player.center[1] - 24.0];
                }
                last_vertical_update.0 = current_time;
            }
        } 
    }
}

fn spawn_block(
    mut app_state: ResMut<State<AppState>>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    materials: Query<&mut Materials>,
) {
    let materials = materials.single();
    let texture_atlas = TextureAtlas::from_grid(materials.player.clone(), Vec2::new(24.0, 24.0), 5, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let (pos, center, shape, index) = random_block();
    let player = Player::new(center, shape, None);


    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[0], pos[1], 0.0),
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(player.clone());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[2], pos[3], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(player.clone());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[4], pos[5], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
    }) .insert(player.clone());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(pos[6], pos[7], 0.0), 
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
    })
    .insert(player.clone());

    app_state.pop().unwrap_or_default();
    app_state.push(AppState::Moving).unwrap_or_default();
}


fn hard_collision_system(
    mut app_state: ResMut<State<AppState>>,
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    block_query: Query<(&InactiveBlock, &Transform)>,
    entity_query: Query<Entity, With<Player>>,
    time: ResMut<Time>,
    mut tick: Query<&mut SingleTick>,
) {
    let mut collision_flag = false;
    let time = time.seconds_since_startup();
    for (_player, player_transform) in player_query.iter() {
        if player_transform.translation.y <= -216.0 {
            collision_flag = true;
            break;
        }
    }
    let mut wall_flag = false;
    for (mut player, player_transform) in player_query.iter_mut() {
        if player_transform.translation.x  >= 86.0 && player.direction == Some(Direction::Right) {
            wall_flag = true;
        } else if player_transform.translation.x <= -96.0 && player.direction == Some(Direction::Left) {
            wall_flag = true;
        }
        for (_block, block_transform) in block_query.iter() {
            if player_transform.translation.y <= block_transform.translation.y + 24.0 && player_transform.translation.x == block_transform.translation.x {
                collision_flag = true;
                break;
            }
        }
    }

    if wall_flag {
        for (mut player, _transform) in player_query.iter_mut() {
            player.direction = None;
        }
    }
    if collision_flag {
        for (mut player, _transform) in player_query.iter_mut() {
            player.direction = Some(Direction::Still);
        }
        if time - tick.single_mut().0 >= HORIZONTAL_TICK{
            for entity in entity_query.iter() {
                commands.entity(entity).remove::<Player>();
                commands.entity(entity).insert(InactiveBlock);
            }
            app_state.pop().unwrap_or_default();
            app_state.push(AppState::Still).unwrap_or_default();
        }
    } else {
        tick.single_mut().0 = time;
    }
}

fn rotation_system(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    block_query: Query<((With<InactiveBlock>, Without<Player>), &Transform)>,
) {
    let mut new_pos: [f32; 8] = [0.0; 8];
    let mut new_center: [f32; 2] = [0.0; 2];
    let mut iterate: usize = 0;
    for (player, transform) in player_query.iter() {
        new_center[0] = player.center[1];
        new_center[1] = -player.center[0];

        new_pos[iterate] = transform.translation.y + (player.center[0] - new_center[0]);
        iterate += 1;
        new_pos[iterate] = -transform.translation.x + (player.center[1] - new_center[1]);
        iterate += 1;
    }
    if keys.just_pressed(KeyCode::E) {
        let mut collision_flag = false;
        for (_block, transform) in block_query.iter() {
            for index in 0..4 {
                if new_pos[index*2] == transform.translation.x && new_pos[index*2 + 1] == transform.translation.y {
                    collision_flag = true;
                    break;
                } else if new_pos[index*2] >= 86.0 || new_pos[index*2] <= -96.0 {
                    collision_flag = true;
                    break;
                }
            }
            if collision_flag {
                break;
            }
        }
        
        if !collision_flag {
            let mut iter: usize = 0;
            for (_player, mut transform) in player_query.iter_mut() {
                transform.translation.x = new_pos[iter*2];
                transform.translation.y = new_pos[iter*2 + 1];
                iter += 1;
            }
        }
    }    
}

fn random_block() -> ([f32; 8], [f32; 2], Shape, usize) {
    let mut rng = rand::thread_rng();
    let shape: Shape = rand::random();
    let pos: [f32; 8]; 
    let center: [f32; 2];
    match shape {
        Shape::IBlock => {
            pos = [12.0, 156.0, 12.0, 180.0, 12.0, 204.0, 12.0, 228.0,];
            center = [12.0, 204.0];
        }
        Shape::JBlock => {
            pos = [12.0, 180.0, 36.0, 180.0, 36.0, 204.0, 36.0, 228.0,];
            center = [36.0, 204.0];
        }
        Shape::SBlock => {
            pos = [-36.0, 204.0, -12.0, 204.0, -12.0, 228.0, 12.0, 228.0,];
            center = [-12.0, 228.0];
        }
        Shape::ZBlock => {
            pos = [-36.0, 228.0, -12.0, 228.0, -12.0, 204.0, 12.0, 204.0,];
            center = [-12.0, 228.0];
        }
        Shape::TBlock => {
            pos = [-36.0, 204.0, -12.0, 204.0, -12.0, 228.0, 12.0, 204.0,];
            center = [-12.0, 228.0];
        }
        Shape::OBlock => {
            pos = [-12.0, 228.0, -12.0, 204.0, 12.0, 228.0, 12.0, 204.0,];
            center = [12.0, 228.0];
        }
        Shape::LBlock => {
            pos = [36.0, 180.0, 12.0, 180.0, 12.0, 204.0, 12.0, 228.0,];
            center = [36.0, 204.0];
        }
    }
    let index: usize = rng.gen_range(0..6) * 5;
    (pos, center, shape, index)

}

fn eliminate_system(
    mut q: QuerySet<(
        QueryState<(&InactiveBlock, &Transform)>,
        QueryState<(Entity, &Transform), With<InactiveBlock>>,
    )>,
    mut eliminate: Query<&mut Eliminate>,
    time: Res<Time>,
) {
    let full_rows: Vec<f32> = Vec::new();
    let (mut eliminate_tick, step) = (eliminate.single_mut().time, eliminate.single_mut().step);
    let time = time.seconds_since_startup();
    if time - eliminate_tick > HORIZONTAL_TICK {
        for (_block, transform) in q.q0().iter() {
            
        }
    }
}