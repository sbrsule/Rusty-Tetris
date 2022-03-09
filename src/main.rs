use bevy::prelude::*;
use game::{collision::CollisionPlugin, movement::MovementPlugin, player::PlayerPlugin, setup_game::SetupGamePlugin};
use menu::MenuPlugin;
use misc::states::AppState;

mod menu;
mod game;
mod misc;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: String::from("Tetris"),
            width: 800.0, 
            height: 600.0,
            scale_factor_override: Some(1.0),
            ..Default::default()
        }) 
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(SetupGamePlugin)
        .add_state(AppState::Menu)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
