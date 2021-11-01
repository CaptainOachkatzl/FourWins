mod four_wins;
mod field;
mod fill;
mod field_renderer;
mod coordinate_translation;
mod player_input;

use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use four_wins::FourWinsPlugin;

pub trait Copy: Clone { }

const WINDOW_HEIGHT: f32 = 600.;
const WINDOW_WIDTH: f32 = 600.;

fn main() {
    App::build()
    .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
    .insert_resource(WindowDescriptor {
        title: "Four wins".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(DebugLinesPlugin)
    .add_plugin(FourWinsPlugin)
    .run();
}