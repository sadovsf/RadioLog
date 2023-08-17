use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};
use ratatui::{prelude::Rect, widgets::{Block, Borders}, style::{Style, Color}};
use tui_textarea::{TextArea, Input, Key};

use crate::{common_types::RenderFrame, app_context::AppContext, traits::{RenderResult, EventResult, UIElement}};

use super::define_typed_element;





pub struct LogsInput<'a> {
    textarea :TextArea<'a>,
}
define_typed_element!(LogsInput<'_>);


impl Default for LogsInput<'_> {
    fn default() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_block(Block::default().borders(Borders::ALL).title("Input"));

        LogsInput {
            textarea
        }
    }
}



impl<'a> UIElement for LogsInput<'a> {
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, _app_ctx :&mut AppContext) -> RenderResult {
        let widget = self.textarea.widget();
        f.render_widget(widget, rect);
        Ok(())
    }

    fn on_input(&mut self, event :&KeyEvent, _app_ctx :&mut AppContext) -> EventResult {
        let handled = self.textarea.input(Input {
            key: match event.code {
                KeyCode::Char(c) => Key::Char(c),
                KeyCode::Backspace => Key::Backspace,
                KeyCode::Enter => Key::Enter,
                KeyCode::Left => Key::Left,
                KeyCode::Right => Key::Right,
                KeyCode::Up => Key::Up,
                KeyCode::Down => Key::Down,
                KeyCode::Tab => Key::Tab,
                KeyCode::Delete => Key::Delete,
                KeyCode::Home => Key::Home,
                KeyCode::End => Key::End,
                KeyCode::PageUp => Key::PageUp,
                KeyCode::PageDown => Key::PageDown,
                KeyCode::Esc => Key::Esc,
                KeyCode::F(x) => Key::F(x),
                _ => Key::Null,
            },
            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            alt: event.modifiers.contains(KeyModifiers::ALT),
        });

        match handled {
            true => EventResult::Handled,
            false => EventResult::NotHandled
        }
    }

    fn set_focused(&mut self, focused :bool) {
        self.textarea.set_block(
            match focused {
                true  => Block::default().borders(Borders::ALL).title("Input").border_style(Style::default().fg(Color::Yellow)),
                false => Block::default().borders(Borders::ALL).title("Input").border_style(Style::default()),
            }
        );
    }
}