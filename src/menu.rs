use bevy::prelude::*;

use crate::misc::states::AppState;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const START_FONT: &str = "fonts/PressStart2P-vaV7.ttf";
const BLINK_TICK: f64 = 0.75;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_menu)
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(blink_text)
                    .with_system(user_input)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Menu)
                    .with_system(despawn_menu)
            );
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct StartText;

#[derive(Component)]
pub struct LastUpdate(f64);

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(LastUpdate(0.0))
        .insert(MainMenu);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "PRESS ENTER",
                        TextStyle {
                            font: asset_server.load(START_FONT),
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    }
                    ),
                    ..Default::default()
                })
                .insert(StartText);
            parent 
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "TETRIS",
                        TextStyle {
                            font: asset_server.load(START_FONT),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
        });
}

fn blink_text(
    mut start_text_query: Query<(&StartText, &mut Text)>,
    time: Res<Time>,
    mut last_update_query: Query<&mut LastUpdate>,
) {
    let mut last_update = last_update_query.single_mut();
    let (mut _start_text, mut text) = start_text_query.single_mut();
    if time.seconds_since_startup() - last_update.0 > BLINK_TICK {
        if text.sections[0].style.color == Color::WHITE {
            text.sections[0].style.color = Color::NONE;
        } else {
            text.sections[0].style.color = Color::WHITE;
        }   

        last_update.0 = time.seconds_since_startup();
    }
}

fn user_input(
    mut app_state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Return) {
        app_state.set(AppState::Game).unwrap_or_default();
    }
}

fn despawn_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenu>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}