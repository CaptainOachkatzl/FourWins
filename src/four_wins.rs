use crate::coordinate_translation::*;
use crate::field::*;
use crate::field_renderer;
use crate::fill::*;
use crate::player_input::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub struct FourWinsPlugin;

const PLAYER1_CHIP: &str = "player1.png";
const PLAYER2_CHIP: &str = "player2.png";

pub struct PlayerData {
    pub position: usize,
    pub index: i32,
}

pub struct PlayerControlled(bool);

const FPS: f64 = 60.;
const FRAME_TIME: f64 = 1. / FPS;

const FIELD_HEIGHT: f32 = 400.;
const FIELD_WIDTH: f32 = 400.;
const FIELD_BLOCKS_HORIZONTAL: usize = 8;
const FIELD_BLOCKS_VERTICAL: usize = 8;

const CHIP_START_Y: f32 = FIELD_HEIGHT / 2. + 50.;

impl Plugin for FourWinsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // add things to your app here
        app.insert_resource(Field::new(FIELD_BLOCKS_HORIZONTAL, FIELD_BLOCKS_VERTICAL))
            .insert_resource(CoordinateTranslation::new(
                FIELD_BLOCKS_HORIZONTAL,
                FIELD_BLOCKS_VERTICAL,
                FIELD_WIDTH,
                FIELD_HEIGHT,
                -FIELD_WIDTH / 2.,
                -FIELD_HEIGHT / 2.,
            ))
            .insert_resource(PlayerData {
                position: 0,
                index: 0,
            })
            .insert_resource(PlayerInput::new(Box::new([
                KeyCode::Space,
                KeyCode::Left,
                KeyCode::Right,
            ])))
            .add_startup_system(initialize.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(FRAME_TIME))
                    .with_system(update_player_actions.system()),
            )
            .add_system(render.system());
    }
}

pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    player_data: Res<PlayerData>,
    coordinate_translation: Res<CoordinateTranslation>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    create_player_chip(
        commands,
        asset_server,
        materials,
        &player_data,
        &coordinate_translation,
    );
}

pub fn update_player_actions(
    commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
    mut field: ResMut<Field>,
    mut player_data: ResMut<PlayerData>,
    coordinate_translation: Res<CoordinateTranslation>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut PlayerControlled,
        With<PlayerControlled>,
    )>,
) {
    let winner: i32 = field.get_winner();
    if winner >= 0 {
        return;
    }

    player_input.update(keyboard_input);

    if player_input.just_pressed(KeyCode::Left) {
        if player_data.position > 0 {
            player_data.position -= 1;
        }
    }
    if player_input.just_pressed(KeyCode::Right) {
        if player_data.position < FIELD_BLOCKS_HORIZONTAL - 1 {
            player_data.position += 1;
        }
    }

    let mut drop = false;
    if player_input.just_pressed(KeyCode::Space) {
        drop = true;
    }

    for (_, mut transform, mut player_controlled, _) in query.iter_mut() {
        if player_controlled.0 {
            transform.translation.x =
                coordinate_translation.horizontal_center_to_pixel(player_data.position);

            if drop {
                let y = field.insert(
                    player_data.position,
                    match player_data.index {
                        0 => Fill::Player1,
                        _ => Fill::Player2,
                    },
                );

                if y >= 0 {
                    player_controlled.0 = false;

                    transform.translation.y =
                        coordinate_translation.vertical_center_to_pixel(y as usize);
                    let winner: i32 = field.get_winner();
                    if winner >= 0 {
                        println!("player {} won!", winner);
                        return;
                    }

                    player_data.index = (player_data.index + 1) % 2;
                    create_player_chip(
                        commands,
                        asset_server,
                        materials,
                        &player_data,
                        &coordinate_translation,
                    );
                }
            }
            return;
        }
    }
}

pub fn render(field: Res<Field>, lines: ResMut<DebugLines>) {
    field_renderer::draw(&field, lines);
}

pub fn create_player_chip(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_data: &PlayerData,
    coordinate_translation: &CoordinateTranslation,
) {
    let asset_string = match player_data.index {
        0 => PLAYER1_CHIP,
        _ => PLAYER2_CHIP,
    };

    let player_x = coordinate_translation.horizontal_center_to_pixel(player_data.position);
    let player_pixel_pos = Vec3::new(player_x, CHIP_START_Y, 10.);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load(asset_string).into()),
            sprite: Sprite::new(Vec2::new(40., 40.)),
            transform: Transform {
                translation: player_pixel_pos,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerControlled(true));
}
