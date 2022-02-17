use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);
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
struct Shape {
    ShapeType: ShapeType,
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

    map_query.build_layer(&mut commands, layer_builder_grid, texture_handle.clone());
    commands.entity(layer_0_entity);

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    map.add_layer(&mut commands, 0u16, layer_0_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-32.0, -32.0, 0.0))
        .insert(GlobalTransform::default());
}