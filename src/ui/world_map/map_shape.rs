
use tui::{widgets::canvas::{Shape, Painter}, style::Color};
use super::world::WORLD_HIGH_RESOLUTION;


pub struct MapShape {
    pub color: Color,
}


impl Shape for MapShape {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in WORLD_HIGH_RESOLUTION {
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}