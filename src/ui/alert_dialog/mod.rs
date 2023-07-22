use crossterm::event::KeyCode;
use tui::{layout::{Constraint, Layout, Direction, Rect}, widgets::{Clear, Block, Borders, Paragraph}, style::{Style, Color}};
use unicode_width::UnicodeWidthStr;

use crate::traits::{DialogInterface, DialogHelpers};


bitflags::bitflags! {

    #[derive(Clone, Eq, PartialEq)]
    pub struct AlertDialogButton : u8 {
        const OK     = 0b00000001;
        const CANCEL = 0b00000010;
        const YES    = 0b00000100;
        const NO     = 0b00001000;
    }
}

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
    focused_button :AlertDialogButton,
}


pub struct AlertDialog {
    state :AlertDialogState,

    message :String,
    buttons :AlertDialogButton,
    style :AlertDialogStyle,
    dialog_result :Option<AlertDialogButton>,
}

impl AlertDialog {
    pub fn new(message :String, buttons :AlertDialogButton, style :AlertDialogStyle) -> Self {
        Self {
            state :AlertDialogState {
                opened: true,
                focused_button: buttons.iter().next().unwrap().clone(),
            },
            message: message,
            buttons: buttons,
            style: style,
            dialog_result :None,
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

    fn render_button<B: tui::backend::Backend>(&self, f :&mut tui::Frame<B>, button :AlertDialogButton, layout :Rect) {
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

    pub fn get_result(&self) -> Option<AlertDialogButton> {
        self.dialog_result.clone()
    }

    fn set_result(&mut self, result :AlertDialogButton) -> () {
        self.dialog_result = Some(result);
        self.close();
    }
}

impl DialogInterface for AlertDialog {
    fn set_opened(&mut self, opened :bool) {
        self.state.opened = opened;
    }

    fn is_opened(&self) -> bool {
        self.state.opened
    }

    fn render<B: tui::backend::Backend>(&mut self, f :&mut tui::Frame<B>) -> () {
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

        let mut layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
            ].as_ref())
            .split(area);

        layout[0].y += 1;
        layout[0].height -= 1;

        layout[0].x += 5;
        layout[0].width -= 5;
        f.render_widget(
            Paragraph::new(self.message.clone())
                .style(Style::default().fg(Color::White)),
            layout[0]
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
            .split(layout[1]);

        for (index, button) in self.buttons.iter().enumerate() {
            self.render_button(f, button, button_layout[index]);
        }

    }

    fn on_input(&mut self, key :crossterm::event::KeyEvent) -> () {
        match key.code {
            KeyCode::Esc => {
                self.set_result(AlertDialogButton::CANCEL);
            },
            KeyCode::Enter => {
                self.set_result(self.state.focused_button.clone());
            },
            _ => {}
        }
    }
}

