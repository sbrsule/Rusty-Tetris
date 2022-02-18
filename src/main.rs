mod map;
mod helpers;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use crate::map::MapPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: String::from("TETRIS"),
            width: 720.0, 
            height: 1280.0,
            ..Default::default()
        })
        .add_plugin(MapPlugin)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
