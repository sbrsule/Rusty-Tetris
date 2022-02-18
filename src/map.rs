use bevy::{prelude::*, core::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(rotate_shape)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(update)
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum ShapeType {
    OBlock,
    IBlock,
    LBlock,
    JBlock,
    TBlock,
    ZBlock,
    SBlock
}

#[derive(Component)]
struct Block {
    position: [[u32; 2]; 4],
    shape: ShapeType
}

impl Block {
    fn new (&self) {

    }
}
fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut map_query: MapQuery
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle: Handle<Image> = asset_server.load("textures/mytiles.png");
    let layer_settings = LayerSettings::new(
        MapSize(2, 2),
        ChunkSize(2, 2),
        TileSize(16.0, 16.0),
        TextureSize(64.0, 16.0)
    );
    let (mut layer_builder_grid, layer_0_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings.clone(), 0u16, 0u16);

    layer_builder_grid.set_all(Tile {
        texture_index: 1,
        ..Default::default()
    }
    .into());

    let mut block = Block {position: [[0,0], [1,0], [2,0], [3,0]], shape: ShapeType::IBlock};

    map_query.build_layer(&mut commands, layer_builder_grid, texture_handle.clone());
    commands.entity(layer_0_entity).insert(block);

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    map.add_layer(&mut commands, 0u16, layer_0_entity);


    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-32.0, -32.0, 0.0))
        .insert(GlobalTransform::default());
}

fn rotate_shape(
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    mut block: Query<&mut Block>
) {
    let mut block = block.single_mut();
    // Rotates Clockwise
    if keys.just_pressed(KeyCode::E) {
        match block.shape {
            ShapeType::OBlock => (),
            ShapeType::IBlock => {
                block.position = [[0,0], [0,1], [0,2], [0,3]];
            }
            ShapeType::LBlock => todo!(),
            ShapeType::JBlock => todo!(),
            ShapeType::TBlock => todo!(),
            ShapeType::ZBlock => todo!(),
            ShapeType::SBlock => todo!(),
        }

        keys.reset(KeyCode::E);
    }
}

fn update(
    mut commands: Commands,
    mut tile_query: Query<&mut Tile>,
    mut map_query: MapQuery,
    time: Res<Time>,
    mut block: Query<&mut Block>
) {
    let mut block = block.single_mut();
    for pos in block.position {
        let tile_pos = TilePos(pos[0], pos[1]);
        let tile_entity = map_query.get_tile_entity(tile_pos, 0u16, 0u16).unwrap();
        let mut tile = tile_query.get_mut(tile_entity).unwrap();
        tile.texture_index = 0;
        map_query.notify_chunk_for_tile(tile_pos, 0u16, 0u16);
    }
}
