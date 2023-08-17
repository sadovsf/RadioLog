use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{prelude::{Rect, Alignment}, widgets::{Paragraph, Block, Wrap, Borders}, style::{Style, Color}};
use unicode_width::UnicodeWidthStr;
use crate::{common_types::RenderFrame, traits::{UIElement, RenderResult, EventResult}, app_context::AppContext};

use super::unique_ids::define_typed_element;






#[derive(Debug, Default)]
pub struct Input {
    content :String,
    label :Option<String>,
    is_focused :bool,

    cursor_pos :usize,
    cursor_byte_pos :usize,
}
define_typed_element!(Input);

impl Input {

    pub fn set_label(mut self, label :String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn set_focused(&mut self, focused :bool) {
        self.is_focused = focused;
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor_pos = 0;
        self.cursor_byte_pos = 0;
    }

    pub fn set(&mut self, content :String) {
        self.content = content;
        self.cursor_pos = self.content.width();
        self.cursor_byte_pos = self.content.len();
    }

    pub fn get(&self) -> &String {
        &self.content
    }


    fn insert_char(&mut self, c :char) {
        self.content.insert(self.cursor_byte_pos, c);
        self.cursor_pos += 1;
        self.cursor_byte_pos += c.len_utf8();
    }

    fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            let deleted_char = self.content.char_indices().nth(self.cursor_pos - 1).unwrap().1;
            let char_bytes = deleted_char.len_utf8();
            self.content.remove(self.cursor_byte_pos - char_bytes);
            self.cursor_pos -= 1;
            self.cursor_byte_pos -= char_bytes;
        }
    }

    fn delete_char_forward(&mut self) {
        if self.cursor_pos < self.content.len() {
            self.content.remove(self.cursor_byte_pos);
        }
    }

    fn cursor_next(&mut self) {
        if self.cursor_byte_pos < self.content.len() {
            let next_char = self.content.char_indices().nth(self.cursor_pos).unwrap().1;
            let char_bytes = next_char.len_utf8();
            self.cursor_pos += 1;
            self.cursor_byte_pos += char_bytes;
        }
    }

    fn cursor_prev(&mut self) {
        if self.cursor_pos > 0 {
            let prev_char = self.content.char_indices().nth(self.cursor_pos - 1).unwrap().1;
            let char_bytes = prev_char.len_utf8();
            self.cursor_pos -= 1;
            self.cursor_byte_pos -= char_bytes;
        }
    }
}


impl<'a> UIElement for Input {
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, _app_ctx :&mut AppContext) -> RenderResult {
        let mut block = Block::default().borders(Borders::ALL);
        if let Some(label) = &self.label {
            block = block.title(label.clone());
        }

        if self.is_focused {
            block = block.style(Style::default().fg(Color::Yellow));
        }

        let input = Paragraph::new(self.content.clone())
            .block(block)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(input, rect);

        if self.is_focused {
            f.set_cursor(
                rect.x + self.cursor_pos as u16 + 1,
                rect.y + 1
            );
        }
        Ok(())
    }

    fn on_input(&mut self, input :&KeyEvent, _app_ctx :&mut AppContext) -> EventResult {
        if self.is_focused == false {
            return EventResult::NOOP;
        }

        match input.code {
            KeyCode::Char(c) => {
                self.insert_char(c);
                EventResult::Handled
            },
            KeyCode::Backspace => {
                self.delete_char();
                EventResult::Handled
            },
            KeyCode::Delete => {
                self.delete_char_forward();
                EventResult::Handled
            },
            KeyCode::Left => {
                self.cursor_prev();
                EventResult::Handled
            },
            KeyCode::Right => {
                self.cursor_next();
                EventResult::Handled
            },
            KeyCode::Home => {
                self.cursor_pos = 0;
                self.cursor_byte_pos = 0;
                EventResult::Handled
            },

            KeyCode::End => {
                self.cursor_pos = self.content.width();
                self.cursor_byte_pos = self.content.len();
                EventResult::Handled
            },
            _ => EventResult::NotHandled
        }
    }
}