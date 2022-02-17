use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: String::from("TETRIS"),
            width: 720.0, 
            height: 1280.0,
            ..Default::default()
        })
        .run();
}
