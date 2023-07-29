use crossterm::event::KeyEvent;
use crate::{actions::Actions, common_types::RenderFrame, app_context::AppContext};
use thiserror::Error;

#[derive(PartialEq, Eq)]
pub enum EventResult {
    Handled,
    NotHandled,
    NOOP
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Failed to layout UI element")]
    LayoutError,
}

pub type RenderResult = Result<(), RenderError>;

#[derive(PartialEq)]
pub enum UIEvents<'a> {
    Input(KeyEvent),
    Action(&'a Actions),
}

pub trait UIElement {
    fn render(&mut self, _f :&mut RenderFrame, _app_ctx :&mut AppContext) -> RenderResult;


    fn on_draw(&mut self, f :&mut RenderFrame, _app_ctx :&mut AppContext) -> RenderResult {
        self.render(f, _app_ctx)
    }

    fn on_event(&mut self, event :&UIEvents, app_ctx :&mut AppContext) -> EventResult {
        self._route_event(event, app_ctx)
    }

    fn _route_event(&mut self, event :&UIEvents, app_ctx :&mut AppContext) -> EventResult {
        match event {
            UIEvents::Input(key) => self.on_input(key, app_ctx),
            UIEvents::Action(action) => self.on_action(action, app_ctx),
        }
    }

    fn on_input(&mut self, _key :&KeyEvent, _app_ctx :&mut AppContext) -> EventResult {
        EventResult::NotHandled
    }

    fn on_action(&mut self, _action :&Actions, _app_ctx :&mut AppContext) -> EventResult {
        EventResult::NotHandled
    }
}
