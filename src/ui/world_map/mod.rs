mod map_shape;
mod world;

use crossterm::event::KeyCode;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Clear};
use ratatui::widgets::canvas::{Canvas, Context};

use crate::actions::Actions;
use crate::app_context::AppContext;
use crate::common_types::RenderFrame;
use crate::data::position::Position;
use crate::traits::{DialogHelpers, DialogInterface, EventResult, RenderResult, UIElement};

use self::map_shape::MapShape;

use super::unique_ids::define_typed_element;


pub struct WorldMapWidgetState {
    pub top_left :Position,
    pub zoom :f64,

    pub selected_position :Option<Position>,
    pub is_opened :bool,
}

impl Default for WorldMapWidgetState {
    fn default() -> Self {
        Self {
            top_left: Position::new(-90.0, -180.0),
            zoom: 1.0,

            selected_position: None,
            is_opened: false
        }
    }
}



#[derive(Default)]
pub struct WorldMap {
    state :WorldMapWidgetState,
}
define_typed_element!(WorldMap);


impl WorldMap {

    fn zoom_map(&mut self, zoom :f64) {
        let old_center = self.map_center();
        self.state.zoom += zoom;
        self.state.zoom = self.state.zoom.clamp(0.05, 5.0);
        let new_center = self.map_center();

        self.state.top_left.longitude += old_center.longitude - new_center.longitude;
        self.state.top_left.latitude += old_center.latitude - new_center.latitude;
    }

    fn draw_points(&self, ctx :&mut Context, app_ctx :&AppContext) {
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
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        if !self.is_opened() {
            return Ok(());
        }

        let area = DialogHelpers::center_rect_size(rect.width, rect.height, rect);
        f.render_widget(Clear, area); //this clears out the background

        let canvas = Canvas::default()
            .block(
                Block::default().title("World (+- to zoom, arrows to move)")
                .borders(Borders::ALL)
            )
            .paint(|ctx| {
                ctx.draw(&MapShape {
                    color: Color::White,
                });
                ctx.layer();
                self.draw_points(ctx, app_ctx);
            })
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([self.state.top_left.longitude, self.state.top_left.longitude + (360.0 * self.state.zoom)])
            .y_bounds([self.state.top_left.latitude, self.state.top_left.latitude + (180.0 * self.state.zoom)]);

        f.render_widget(canvas, rect);

        Ok(())
    }

    fn on_action(&mut self, action :&crate::actions::Actions, app_ctx :&mut AppContext) -> crate::traits::EventResult {
        match action {
            Actions::ToggleMap => {
                if self.is_opened() {
                    self.close();
                } else {
                    self.open();
                }
                EventResult::Handled
            },
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
        if ! self.is_opened() {
            return EventResult::NOOP;
        }

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

            KeyCode::Up => {
                self.state.top_left.latitude += 5.0;
                EventResult::Handled
            },
            KeyCode::Down => {
                self.state.top_left.latitude -= 5.0;
                EventResult::Handled
            },
            KeyCode::Left => {
                self.state.top_left.longitude -= 5.0;
                EventResult::Handled
            },
            KeyCode::Right => {
                self.state.top_left.longitude += 5.0;
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }
}


impl DialogInterface for WorldMap {
    fn set_opened(&mut self, opened :bool) {
        self.state.is_opened = opened;
    }

    fn is_opened(&self) -> bool {
        self.state.is_opened
    }
}