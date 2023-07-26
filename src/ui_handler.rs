use std::slice::{IterMut};

use crate::{traits::{UIElement, UIEvents, EventResult}, actions::ActionProcessor, common_types::RenderFrame};





#[derive(Default)]
pub struct UIHandler {
    elements: Vec<Box<dyn UIElement>>,
}

impl UIHandler {
    pub fn add(&mut self, element: Box<dyn UIElement>) -> usize {
        self.elements.push(element);
        self.elements.len() - 1
    }

    pub fn send_event(&mut self, event :&UIEvents, actions :&mut ActionProcessor) -> EventResult {
        let mut result = EventResult::NotHandled;
        for element in &mut self.elements {
            if element.on_event(event, actions) == EventResult::Handled {
                result = EventResult::Handled;
            }
        }
        result
    }

    pub fn draw(&mut self, f :&mut RenderFrame, actions :&mut ActionProcessor) {
        for element in &self.elements {
            element.on_draw(f, actions);
        }
    }
}