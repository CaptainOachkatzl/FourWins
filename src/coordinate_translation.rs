#![allow(dead_code)]

#[derive(Clone, Copy)]
pub struct CoordinateTranslation {
    width_coordinates: usize,
    height_coordinates: usize,
    width_pixel: f32,
    height_pixel: f32,
    x_offset: f32,
    y_offset: f32,
    block_width: f32,
    block_height: f32,
}

impl CoordinateTranslation {
    pub fn new(
        width_coordinates: usize,
        height_coordinates: usize,
        width_pixel: f32,
        height_pixel: f32,
        x_offset: f32,
        y_offset: f32,
    ) -> CoordinateTranslation {
        return CoordinateTranslation {
            width_coordinates: width_coordinates,
            height_coordinates: height_coordinates,
            width_pixel: width_pixel,
            height_pixel: height_pixel,
            x_offset: x_offset,
            y_offset: y_offset,
            block_width: width_pixel / width_coordinates as f32,
            block_height: height_pixel / height_coordinates as f32,
        };
    }

    pub fn block_center_to_pixel_position(
        &self,
        x: usize,
        y: usize,
    ) -> (f32, f32) {
        return (
            self.horizontal_center_to_pixel(x),
            self.vertical_center_to_pixel(y),
        );
    }

    pub fn horizontal_center_to_pixel(&self, x: usize) -> f32 {
        return self.x_offset + (x as f32 * self.block_width) + (self.block_width / 2.);
    }

    pub fn vertical_center_to_pixel(&self, y: usize) -> f32 {
        return self.y_offset + (y as f32 * self.block_height) + (self.block_height / 2.);
    }
}
