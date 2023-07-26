use crossterm::event::KeyEvent;
use tui::{layout::{Rect, Layout, Direction, Constraint}};

use crate::{actions::{ActionProcessor, Actions}, common_types::RenderFrame};

use super::{UIElement, RenderResult, EventResult, UIEvents};


pub struct DialogHelpers {}

impl DialogHelpers {

    pub fn center_rect_perc(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

        Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
    }

    pub fn center_rect_size(size_x: u16, size_y: u16, r: Rect) -> Rect {
        Rect {
            x: r.x + (r.width - size_x) / 2,
            y: r.y + (r.height - size_y) / 2,
            width: size_x,
            height: size_y,
        }
    }
}


pub trait DialogInterface {
    fn set_opened(&mut self, opened :bool);
    fn is_opened(&self) -> bool;

    fn render(&mut self, f :&mut RenderFrame, actions :&mut ActionProcessor) -> RenderResult;

    fn on_input(&mut self, _key :&KeyEvent, _actions :&mut ActionProcessor) -> EventResult {
        EventResult::NotHandled
    }

    fn on_action(&mut self, _action :&Actions, _actions :&mut ActionProcessor) -> EventResult {
        EventResult::NotHandled
    }

    fn open(&mut self) {
        self.set_opened(true);
    }
    fn close(&mut self) {
        self.set_opened(false);
    }
}

impl<T> UIElement for T where T: DialogInterface {

    fn on_draw(&mut self, f :&mut RenderFrame, actions :&mut ActionProcessor) -> RenderResult {
        if self.is_opened() == false {
            return RenderResult::NOOP;
        }
        self.render(f, actions)
    }

    fn on_event(&mut self, event :&UIEvents, actions :&mut ActionProcessor) -> EventResult {
        if self.is_opened() == false {
            return EventResult::NotHandled;
        }

        self._route_event(event, actions)
    }

    fn render(&mut self, f :&mut RenderFrame, actions :&mut ActionProcessor) -> RenderResult {
        T::render(self, f, actions)
    }

    fn on_input(&mut self, key :&KeyEvent, actions :&mut ActionProcessor) -> EventResult {
        T::on_input(self, key, actions)
    }

    fn on_action(&mut self, action :&Actions, actions :&mut ActionProcessor) -> EventResult {
        T::on_action(self, action, actions)
    }
}