use crossterm::event::KeyEvent;
use tui::prelude::Rect;
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

    #[error("Element was already rendered this frame")]
    AlreadyRendered,
}

pub type RenderResult = Result<(), RenderError>;

#[derive(PartialEq)]
pub enum UIEvents<'a> {
    Input(KeyEvent),
    Action(&'a Actions),
}


#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct UIElementType {
    uid  :u64,
    name :&'static str,
}

impl UIElementType {
    pub const fn new(name :&'static str, uid :u64) -> Self {
        Self {
            uid: uid,
            name: name,
        }
    }
}


pub trait TypedUIElement :UIElement {
    fn get_type_type() -> &'static UIElementType;
}


pub trait UIElement {
    fn render(&mut self, _f :&mut RenderFrame, _rect :Rect, _app_ctx :&mut AppContext) -> RenderResult;
    fn get_type(&self) -> &'static UIElementType;

    fn on_draw(&mut self, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        self.render(f, rect, app_ctx)
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
