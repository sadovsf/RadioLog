use crossterm::event::KeyEvent;
use tui::layout::{Rect, Layout, Direction, Constraint};

use crate::{actions::Actions, common_types::RenderFrame, app_context::AppContext};

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

    fn render(&self, f :&mut RenderFrame, _app_ctx :&mut AppContext) -> RenderResult;

    fn on_input(&mut self, _key :&KeyEvent, _app_ctx :&mut AppContext) -> EventResult {
        EventResult::NotHandled
    }

    fn on_action(&mut self, _action :&Actions, _app_ctx :&mut AppContext) -> EventResult {
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

    fn on_draw(&mut self, f :&mut RenderFrame, app_ctx :&mut AppContext) -> RenderResult {
        if self.is_opened() == false {
            return Ok(());
        }
        self.render(f, app_ctx)
    }

    fn on_event(&mut self, event :&UIEvents, app_ctx :&mut AppContext) -> EventResult {
        if self.is_opened() == false {
            return match event {
                UIEvents::Action(_) => self._route_event(event, app_ctx)
                , _ => EventResult::NotHandled
            }
        }

        self._route_event(event, app_ctx)
    }

    fn render(&mut self, f :&mut RenderFrame, app_ctx :&mut AppContext) -> RenderResult {
        T::render(self, f, app_ctx)
    }

    fn on_input(&mut self, key :&KeyEvent, app_ctx :&mut AppContext) -> EventResult {
        T::on_input(self, key, app_ctx)
    }

    fn on_action(&mut self, action :&Actions, app_ctx :&mut AppContext) -> EventResult {
        T::on_action(self, action, app_ctx)
    }
}