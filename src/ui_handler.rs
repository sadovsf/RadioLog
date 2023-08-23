use ratatui::prelude::Rect;
use crate::{traits::{UIElement, UIEvents, EventResult, RenderResult, UIElementType, RenderError}, common_types::RenderFrame, app_context::AppContext};



#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct UIElementID {
    index :usize,
    element_type :&'static UIElementType,
}


struct ElementStorage {
    element :Box<dyn UIElement>,
    last_rendered_frame :u8,
}


#[derive(Default)]
pub struct UIHandler {
    elements: Vec<ElementStorage>,
    focused_index :Option<usize>,
}

impl UIHandler {
    pub fn add(&mut self, element: Box<dyn UIElement>) -> UIElementID {
        let id = UIElementID {
            index: self.elements.len(),
            element_type: element.get_type()
        };

        self.elements.push(ElementStorage {
            element,
            last_rendered_frame: 0,
        });

        if self.focused_index.is_none() {
            self.focus_next();
        }

        id
    }

    // Currently not needed but may be usefull in the future.
    // unsafe fn downcast_element<T>(item :&mut dyn UIElement) -> &mut T {
    //     let ptr = item as *mut dyn UIElement as *mut T;
    //     &mut *ptr
    // }

    // pub fn get<T :TypedUIElement>(&mut self, id :&UIElementID) -> Result<&mut T, String> {
    //     let element = self.elements[id.index].element.as_mut();
    //     if element.get_type() != id.element_type {
    //         return Err("Element type mismatch".to_string());
    //     }
    //     if element.get_type() != T::get_type_type() {
    //         return Err("Element type mismatch".to_string());
    //     }
    //     Ok(unsafe { UIHandler::downcast_element::<T>(element) })
    // }

    pub fn send_event(&mut self, event :&UIEvents, app_ctx :&mut AppContext) -> EventResult {
        if let Some(index) = self.focused_index {
            let element = &mut self.elements[index];
            return element.element.on_event(event, app_ctx);
        }

        let mut result = EventResult::NotHandled;
        for element in &mut self.elements {
            if element.element.on_event(event, app_ctx) == EventResult::Handled {
                result = EventResult::Handled;
                break;
            }
        }
        result
    }

    pub fn draw_all(&mut self, frame_index :u8, f :&mut RenderFrame, app_ctx :&mut AppContext) -> RenderResult {
        for entry in &mut self.elements {
            if entry.last_rendered_frame != frame_index {
                entry.last_rendered_frame = frame_index;
                entry.element.on_draw(f, f.size(), app_ctx)?;
            }
        };
        Ok(())
    }

    pub fn draw_single(&mut self, id :&UIElementID, frame_index :u8, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        let entry = &mut self.elements[id.index];
        if entry.last_rendered_frame == frame_index {
            return Err(RenderError::AlreadyRendered);
        }

        entry.last_rendered_frame = frame_index;
        entry.element.on_draw(f, rect, app_ctx)?;
        Ok(())
    }

    pub fn focus_next(&mut self) {
        if self.elements.len() == 0 && self.focused_index.is_some() {
            panic!("Invalid ui handler state. Empty elements list while having focused index.");
        }
        if self.elements.len() == 0 {
            return;
        }

        if self.focused_index.is_none() {
            self.focused_index = Some(0);
            self.elements[0].element.set_focused(true);
            return;
        }

        if let Some(index) = self.focused_index {
            let mut index = index + 1;
            if index >= self.elements.len() {
                index = 0;
            }

            if index != self.focused_index.unwrap() {
                self.elements[self.focused_index.unwrap()].element.set_focused(false);
                self.elements[index].element.set_focused(true);
            }
            self.focused_index = Some(index);
        }
    }
}