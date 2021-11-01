use crate::field::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub fn draw(field: &Field, lines: ResMut<DebugLines>) {
    draw_grid(field, lines);
}

fn draw_grid(field: &Field, mut lines: ResMut<DebugLines>) {
    let width = 400.;
    let height = 400.;

    let block_width = width / field.width as f32;
    let block_height = height / field.height as f32;

    for i in 0..=field.width {
        let x = (i as f32 * block_width) - (width / 2.);
        let start = Vec3::new(x, -height / 2., 100.);
        let end = Vec3::new(x, height / 2., 100.);
        let duration = 0.0; // Duration of 0 will show the line for 1 frame.
        lines.line(start, end, duration);
    }

    for i in 0..=field.height {
        let y = (i as f32 * block_height) - (height / 2.);
        let start = Vec3::new(-width / 2., y, 100.);
        let end = Vec3::new(width / 2., y, 100.);
        let duration = 0.0; // Duration of 0 will show the line for 1 frame.
        lines.line(start, end, duration);
    }
}

