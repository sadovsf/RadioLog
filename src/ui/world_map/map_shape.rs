use tui::{widgets::canvas::{Shape, Painter}, style::Color};
use super::world::WORLD_HIGH_RESOLUTION;


pub struct MapShape {
    pub color: Color,
}

// Improve by using country borders from
//      https://public.opendatasoft.com/explore/dataset/world-administrative-boundaries/export/
//      Possibility to use https://docs.rs/shapefile/latest/shapefile/ and rendering straight from shape file
impl Shape for MapShape {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in WORLD_HIGH_RESOLUTION {
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}