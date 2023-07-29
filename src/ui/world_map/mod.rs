mod map_shape;
mod world;

use crossterm::event::KeyCode;
use tui::prelude::{Layout, Direction, Constraint};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders};
use tui::widgets::canvas::{Canvas, Context};

use crate::actions::Actions;
use crate::app_context::AppContext;
use crate::common_types::RenderFrame;
use crate::data::position::Position;
use crate::traits::{UIElement, RenderResult, EventResult};

use self::map_shape::MapShape;


pub struct WorldMapWidgetState {
    pub top_left :Position,
    pub zoom :f64,

    pub selected_position :Option<Position>,
}

impl Default for WorldMapWidgetState {
    fn default() -> Self {
        Self {
            top_left: Position::new(-90.0, -180.0),
            zoom: 1.0,

            selected_position: None
        }
    }
}



#[derive(Default)]
pub struct WorldMap {
    state :WorldMapWidgetState,
}


impl WorldMap {
    fn zoom_map(&mut self, zoom :f64) {
        let old_center = self.map_center();
        self.state.zoom += zoom;
        self.state.zoom = self.state.zoom.clamp(0.05, 5.0);
        let new_center = self.map_center();

        self.state.top_left.longitude += old_center.longitude - new_center.longitude;
        self.state.top_left.latitude += old_center.latitude - new_center.latitude;
    }

    pub fn draw_points(&self, ctx :&mut Context, app_ctx :&AppContext) {
        ctx.print(
            app_ctx.data.config.own_position.longitude,
            app_ctx.data.config.own_position.latitude,
            Span::styled("x", Style::default().fg(Color::Green))
        );

        if self.state.selected_position.is_some() {
            let selected_position = self.state.selected_position.as_ref().unwrap();
            ctx.print(
                selected_position.longitude,
                selected_position.latitude,
                Span::styled("x", Style::default().fg(Color::Red))
            );
        }
    }

    pub fn map_center(&self) -> Position {
        let width = self.state.top_left.longitude + (360.0 * self.state.zoom);
        let height = self.state.top_left.latitude + (180.0 * self.state.zoom);

        Position::new(
            self.state.top_left.latitude + (height / 2.0),
            self.state.top_left.longitude + (width / 2.0),
        )
    }
}


impl UIElement for WorldMap {
    fn render(&mut self, f :&mut RenderFrame, app_ctx :&mut AppContext) -> RenderResult {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ].as_ref()
            )
            .split(f.size());


        let canvas = Canvas::default()
            .block(Block::default().title("World").borders(Borders::ALL))
            .paint(|ctx| {
                ctx.draw(&MapShape {
                    color: Color::White,
                });
                ctx.layer();
                self.draw_points(ctx, app_ctx);
            })
            .marker(tui::symbols::Marker::Braille)
            .x_bounds([self.state.top_left.longitude, self.state.top_left.longitude + (360.0 * self.state.zoom)])
            .y_bounds([self.state.top_left.latitude, self.state.top_left.latitude + (180.0 * self.state.zoom)]);

        f.render_widget(canvas, rects[1]);

        Ok(())
    }

    fn on_action(&mut self, action :&crate::actions::Actions, app_ctx :&mut AppContext) -> crate::traits::EventResult {
        match action {
            Actions::FocusLog(log_id) => {
                if let Some(log_id) = log_id {
                    if let Some(log) = app_ctx.data.logs.get(*log_id) {
                        self.state.selected_position = log.position();
                    } else {
                        self.state.selected_position = None;
                    }
                } else {
                    self.state.selected_position = None;
                }
                EventResult::NotHandled
            }
            _ => EventResult::NOOP
        }
    }

    fn on_input(&mut self, key :&crossterm::event::KeyEvent, _app_ctx :&mut AppContext) -> EventResult {
        match key.code {
            // Map controls:
            KeyCode::Char('+') => {
                self.zoom_map(-0.05);
                EventResult::Handled
            },
            KeyCode::Char('-') => {
                self.zoom_map(0.05);
                EventResult::Handled
            },

            KeyCode::Char('8') => {
                self.state.top_left.latitude += 5.0;
                EventResult::Handled
            },
            KeyCode::Char('5') => {
                self.state.top_left.latitude -= 5.0;
                EventResult::Handled
            },
            KeyCode::Char('4') => {
                self.state.top_left.longitude -= 5.0;
                EventResult::Handled
            },
            KeyCode::Char('6') => {
                self.state.top_left.longitude += 5.0;
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }
}