mod map_shape;
mod world;
use map_shape::MapShape;

use tui::{widgets::{Block, Borders, canvas::{Canvas, Context}}, layout::Rect, style::{Color, Style}, Frame, backend::Backend, text::Span};
use crate::data::{position::Position, Data};


pub struct WorldMapState {
    pub top_left :Position,
    pub zoom :f64,

    pub own_position :Position,
    pub selected_position :Option<Position>,
}

impl Default for WorldMapState {
    fn default() -> WorldMapState {
        WorldMapState {
            top_left: Position::new(-90.0, -180.0),
            zoom: 1.0,

            own_position: Data::get_config().own_position,
            selected_position: None
        }
    }
}


#[derive(Default)]
pub struct WorldMap {

}


impl WorldMap {

    pub fn draw_points(ctx :&mut Context, state :&WorldMapState) {
        ctx.print(
            state.own_position.longitude,
            state.own_position.latitude,
            Span::styled("x", Style::default().fg(Color::Green))
        );

        if state.selected_position.is_some() {
            let selected_position = state.selected_position.as_ref().unwrap();
            ctx.print(
                selected_position.longitude,
                selected_position.latitude,
                Span::styled("x", Style::default().fg(Color::Red))
            );
        }
    }

    pub fn map_center(state :&WorldMapState) -> Position {
        let width = state.top_left.longitude + (360.0 * state.zoom);
        let height = state.top_left.latitude + (180.0 * state.zoom);

        Position::new(
            state.top_left.latitude + (height / 2.0),
            state.top_left.longitude + (width / 2.0),
        )
    }

    pub fn render<B: Backend>(f :&mut Frame<B>, area :Rect, state :&WorldMapState) {
        let canvas = Canvas::default()
            .block(Block::default().title("World").borders(Borders::ALL))
            .paint(|ctx| {
                ctx.draw(&MapShape {
                    color: Color::White,
                });
                ctx.layer();
                WorldMap::draw_points(ctx, &state);
            })
            .marker(tui::symbols::Marker::Braille)
            .x_bounds([state.top_left.longitude, state.top_left.longitude + (360.0 * state.zoom)])
            .y_bounds([state.top_left.latitude, state.top_left.latitude + (180.0 * state.zoom)]);

        f.render_widget(canvas, area);
    }
}