use std::collections::HashSet;

use tui::prelude::Rect;

use crate::{traits::{UIElement, UIEvents, EventResult, RenderResult, RenderError}, common_types::RenderFrame, app_context::AppContext};



#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct UIElementID {
    index :usize
}




#[derive(Default)]
pub struct UIHandler {
    elements: Vec<Box<dyn UIElement>>,
    rendered: HashSet<UIElementID>,
    last_frame: u8
}

impl UIHandler {
    pub fn add(&mut self, element: Box<dyn UIElement>) -> UIElementID {
        self.elements.push(element);
        UIElementID { index: self.elements.len() - 1 }
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

    fn check_frame(&mut self, frame_index :u8) {
        if self.last_frame != frame_index {
            self.rendered.clear();
            self.last_frame = frame_index;
        }
    }

    pub fn draw_all(&mut self, frame_index :u8, f :&mut RenderFrame, app_ctx :&mut AppContext) -> RenderResult {
        self.check_frame(frame_index);

        if self.elements.len() == self.rendered.len() {
            return Ok(());
        }

        for (idx, element) in &mut self.elements.iter_mut().enumerate() {
            let id = UIElementID { index: idx };
            if self.rendered.contains(&id) {
                continue;
            }
            element.on_draw(f, f.size(), app_ctx)?;
            self.rendered.insert(id);
        };
        Ok(())
    }

    pub fn draw_single(&mut self, id :&UIElementID, frame_index :u8, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        self.check_frame(frame_index);

        if self.rendered.contains(&id) {
            return Err(RenderError::AlreadyRendered);
        }

        let result = self.elements[id.index].on_draw(f, rect, app_ctx);
        self.rendered.insert(id.clone());
        result
    }
}