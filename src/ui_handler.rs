use crate::{traits::{UIElement, UIEvents, EventResult}, common_types::RenderFrame, app_context::AppContext};





#[derive(Default)]
pub struct UIHandler {
    elements: Vec<Box<dyn UIElement>>,
}

impl UIHandler {
    pub fn add(&mut self, element: Box<dyn UIElement>) -> usize {
        self.elements.push(element);
        self.elements.len() - 1
    }

    pub fn send_event(&mut self, event :&UIEvents, app_ctx :&mut AppContext) -> EventResult {
        let mut result = EventResult::NotHandled;
        for element in &mut self.elements {
            if element.on_event(event, app_ctx) == EventResult::Handled {
                result = EventResult::Handled;
                break;
            }
        }
        result
    }

    pub fn draw(&mut self, f :&mut RenderFrame, app_ctx :&mut AppContext) {
        for element in &mut self.elements {
            element.on_draw(f, app_ctx);
        }
    }
}