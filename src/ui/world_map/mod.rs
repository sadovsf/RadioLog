mod world_map_widget;
mod map_shape;
mod world;

use crossterm::event::KeyCode;
use tui::prelude::{Layout, Direction, Constraint};

use crate::actions::Actions;
use crate::data::Data;
use crate::traits::{UIElement, RenderResult, EventResult};

use self::world_map_widget::WorldMapWidget;
use self::world_map_widget::WorldMapWidgetState;


#[derive(Default)]
pub struct WorldMap {
    state :WorldMapWidgetState,
}


impl WorldMap {
    fn zoom_map(&mut self, zoom :f64) {
        let old_center = WorldMapWidget::map_center(&self.state);
        self.state.zoom += zoom;
        self.state.zoom = self.state.zoom.clamp(0.05, 5.0);
        let new_center = WorldMapWidget::map_center(&self.state);

        self.state.top_left.longitude += old_center.longitude - new_center.longitude;
        self.state.top_left.latitude += old_center.latitude - new_center.latitude;
    }
}


impl UIElement for WorldMap {
    fn render(&mut self, f :&mut crate::common_types::RenderFrame, _actions :&mut crate::actions::ActionProcessor) -> RenderResult {
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
        WorldMapWidget::render(f, rects[1], &self.state);
        RenderResult::Rendered
    }

    fn on_action(&mut self, action :&crate::actions::Actions, _actions :&mut crate::actions::ActionProcessor) -> crate::traits::EventResult {
        match action {
            Actions::FocusLog(log_id) => {
                if let Some(log_id) = log_id {
                    if let Some(log) = Data::get_log(*log_id) {
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

    fn on_input(&mut self, key :&crossterm::event::KeyEvent, _actions :&mut crate::actions::ActionProcessor) -> EventResult {
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