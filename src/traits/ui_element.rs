use crossterm::event::KeyEvent;
use crate::{actions::{Actions, ActionProcessor}, common_types::RenderFrame};

#[derive(PartialEq, Eq)]
pub enum EventResult {
    Handled,
    NotHandled,
    NOOP
}

pub enum RenderResult {
    Rendered,
    Failed,
    NOOP
}

#[derive(PartialEq)]
pub enum UIEvents<'a> {
    Input(KeyEvent),
    Action(&'a Actions),
}

pub trait UIElement {
    fn render(&mut self, _f :&mut RenderFrame, _actions :&mut ActionProcessor) -> RenderResult;


    fn on_draw(&mut self, f :&mut RenderFrame, actions :&mut ActionProcessor) -> RenderResult {
        self.render(f, actions)
    }

    fn on_event(&mut self, event :&UIEvents, actions :&mut ActionProcessor) -> EventResult {
        self._route_event(event, actions)
    }

    fn _route_event(&mut self, event :&UIEvents, actions :&mut ActionProcessor) -> EventResult {
        match event {
            UIEvents::Input(key) => self.on_input(key, actions),
            UIEvents::Action(action) => self.on_action(action, actions),
        }
    }

    fn on_input(&mut self, _key :&KeyEvent, _actions :&mut ActionProcessor) -> EventResult {
        EventResult::NotHandled
    }

    fn on_action(&mut self, _action :&Actions, _actions :&mut ActionProcessor) -> EventResult {
        EventResult::NotHandled
    }
}
