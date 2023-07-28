use crossterm::event::{KeyCode, KeyEvent};
use tui::{layout::{Constraint, Layout, Direction, Rect}, widgets::{Clear, Block, Borders, Paragraph}, style::{Style, Color}};
use unicode_width::UnicodeWidthStr;

use crate::{traits::{DialogInterface, DialogHelpers, EventResult, RenderResult}, actions::Actions, common_types::RenderFrame, app_context::AppContext};


bitflags::bitflags! {

    #[derive(Clone, Eq, PartialEq)]
    pub struct AlertDialogButton : u8 {
        const OK     = 0b00000001;
        const CANCEL = 0b00000010;
        const YES    = 0b00000100;
        const NO     = 0b00001000;
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum AlertDialogStyle {
    Default,
    Warning,
    Error,
}

impl AlertDialogButton {
    fn count(&self) -> u8 {
        let mut count = 0;
        if self.contains(AlertDialogButton::OK) {
            count += 1;
        }
        if self.contains(AlertDialogButton::CANCEL) {
            count += 1;
        }
        if self.contains(AlertDialogButton::YES) {
            count += 1;
        }
        if self.contains(AlertDialogButton::NO) {
            count += 1;
        }
        count
    }
}

struct AlertDialogState {
    opened :bool,
}


pub struct AlertDialog {
    state :AlertDialogState,

    message :String,
    buttons :AlertDialogButton,
    style :AlertDialogStyle,
    action_on_close: Option<Actions>
}

impl AlertDialog {
    pub fn new(message :String, buttons :AlertDialogButton, style :AlertDialogStyle, on_confirm: Option<Actions>) -> Self {
        Self {
            state :AlertDialogState {
                opened: true,
            },
            message: message,
            buttons: buttons,
            style: style,
            action_on_close: on_confirm,
        }
    }

    fn decide_button_style(&self, button :AlertDialogButton) -> Style {
        match self.style {
            AlertDialogStyle::Warning => {
                match button {
                    AlertDialogButton::YES => {
                        return Style::default()
                            .fg(Color::Black)
                            .bg(Color::LightRed)
                    },
                    AlertDialogButton::NO => {
                        return Style::default()
                            .fg(Color::Black)
                            .bg(Color::LightGreen)
                    },
                    _ => {}
                }
            },
            AlertDialogStyle::Error => {

            },
            _ => {}
        }

        Style::default()
            .fg(Color::Black)
            .bg(Color::LightCyan)
    }

    fn render_button(&self, f :&mut RenderFrame, button :AlertDialogButton, layout :Rect) {
        let button_type = button.iter_names().next().unwrap();
        let button_style = self.decide_button_style(button);


        let mut button_box = layout;
        button_box.x += 2;
        button_box.width -= 2;

        f.render_widget(
            Block::default()
                .style(button_style),
                button_box
        );

        let button_center = Rect {
            x: button_box.x + (button_box.width / 2) - (button_type.0.len() as u16 / 2),
            y: button_box.y + (button_box.height / 2),
            width: button_type.0.len() as u16 + 10,
            height: 1,
        };

        f.render_widget(
            Paragraph::new(button_type.0)
                .style(button_style),
            button_center
        );
    }
}

impl DialogInterface for AlertDialog {
    fn set_opened(&mut self, opened :bool) {
        self.state.opened = opened;
    }

    fn is_opened(&self) -> bool {
        self.state.opened
    }

    fn render(&self, f :&mut RenderFrame, _app_ctx :&mut AppContext) -> RenderResult {
        let area = DialogHelpers::center_rect_size((10 + self.message.width()).max(80) as u16, 10, f.size());
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::default()
                        .fg(match self.style {
                            AlertDialogStyle::Warning => Color::LightYellow,
                            AlertDialogStyle::Error => Color::LightRed,
                            _ => Color::White,
                        })
                ),
            area
        );

        let [_, msg_rect, btns_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(3),
            ].as_ref())
            .split(area)
        else {
            return RenderResult::Failed;
        };

        let [_, msg_rect] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(5),
                Constraint::Percentage(100)
            ].as_ref())
            .split(msg_rect)
        else {
            return RenderResult::Failed;
        };

        f.render_widget(
            Paragraph::new(self.message.clone())
                .style(Style::default().fg(Color::White)),
                msg_rect
        );


        let button_count = self.buttons.count();
        let button_perc = 100 / button_count as u16;
        let constr_array :Vec<Constraint> = (0..button_count).map(|_| {
            Constraint::Percentage(button_perc)
        }).collect();

        let button_layout = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(3)
            .vertical_margin(1)
            .constraints(constr_array.as_ref())
            .split(btns_rect);

        for (index, button) in self.buttons.iter().enumerate() {
            self.render_button(f, button, button_layout[index]);
        };

        RenderResult::Rendered
    }

    fn on_input(&mut self, key :&KeyEvent, app_ctx :&mut AppContext) -> EventResult {
        match key.code {
            KeyCode::Esc => {
                self.close();
                EventResult::Handled
            },
            KeyCode::Enter => {
                if self.action_on_close.is_some() {
                    app_ctx.actions.add(self.action_on_close.take().unwrap());
                }
                self.close();
                EventResult::Handled
            },
            _ => {
                EventResult::NotHandled
            }
        }
    }
}