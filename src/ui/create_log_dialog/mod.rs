extern crate unicode_width;
use unicode_width::UnicodeWidthStr;

use crossterm::event::{KeyEvent, KeyCode};
use tui::{backend::Backend, Frame, layout::{Rect, Layout, Direction, Constraint}, widgets::{Block, Clear, Borders, Paragraph}, style::{Style, Color}, text::Spans};

use crate::{data::{Data, LogEntry}, map_api::OnlineMap, traits::DialogHelpers};

use turbosql::Turbosql;

mod input_fields;
use input_fields::InputFields;
use crate::traits::DialogInterface;


pub struct CreateLogDialogState {
    opened: bool,

    has_error: bool,
    error_message: String,

    current_input: InputFields,

    name: String,
    latitude: String,
    longtitude: String,
}

impl Default for CreateLogDialogState {
    fn default() -> Self {
        Self {
            opened: false,

            has_error: false,
            error_message: String::new(),

            current_input: InputFields::Name,

            name: String::new(),
            latitude: String::new(),
            longtitude: String::new(),
        }
    }
}




#[derive(Default)]
pub struct CreateLogDialog {
    state: CreateLogDialogState,
    last_log_id: Option<i64>,
    log_to_edit: Option<LogEntry>,
}


impl DialogInterface for CreateLogDialog {
    fn set_opened(&mut self, opened :bool) {
        self.state.opened = opened;
    }

    fn is_opened(&self) -> bool {
        self.state.opened
    }

    fn open(&mut self) {
        self.log_to_edit = None;
        self.set_opened(true);
    }

    fn close(&mut self) {
        self.clear_form();
        self.log_to_edit = None;
        self.set_opened(false);
    }

    fn render<B: Backend>(&mut self, f :&mut Frame<B>) {
        let area = DialogHelpers::center_rect_perc(50, 35, f.size());
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(
            Block::default().title("Create log").borders(Borders::ALL),
            area
        );

        let mut constraints = vec!();
        for _ in 0..InputFields::LAST as u8 {
            constraints.push(Constraint::Length(5));
        }
        constraints.push(Constraint::Length(1));


        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints.as_ref())
            .split(area);

        for idx in 0..InputFields::LAST as u8 {
            let input = InputFields::from(idx);
            let field_content = input.to_field(&self.state);

            self.create_input(
                f,
                field_content,
                input,
                popup_layout[idx as usize]
            );
        }
    }


    fn on_input(&mut self, key :KeyEvent) {
        match key.code {
            KeyCode::Esc => self.close(),
            KeyCode::Tab => self.state.current_input = self.state.current_input.next(),
            KeyCode::BackTab => self.state.current_input = self.state.current_input.prev(),
            KeyCode::Delete => self.clear_form(),
            KeyCode::Enter => {
                if self.state.has_error {
                    self.state.has_error = false;
                    return;
                }
                self.save();
            },
            KeyCode::Backspace => {
                self.state.current_input.to_field_mut(&mut self.state).pop();
            },
            KeyCode::Char(c) => {
                self.state.current_input.to_field_mut(&mut self.state).push(c);
            },
            KeyCode::PageDown => self.find_location(&self.state.name.clone()),
            _ => {}
        }
    }
}



impl CreateLogDialog {
    pub fn edit(&mut self, log :LogEntry) {
        if log.rowid.is_none() {
            return;
        }

        self.state.opened = true;
        self.state.name = log.name.clone().unwrap_or("".to_string());
        self.state.latitude = log.lat.map(|v| v.to_string()).unwrap_or("".to_string());
        self.state.longtitude = log.long.map(|v| v.to_string()).unwrap_or("".to_string());
        self.log_to_edit = Some(log);
    }

    fn create_input<'a, B: Backend>(&'a self, f :&mut Frame<B>, content :&'a String, field :InputFields, area :Rect) {
        let mut row_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                ]
                .as_ref(),
            )
            .split(area);

        if row_layout[1].y > 0 {
            row_layout[1].y -= 1;
        }

        let input = Paragraph::new(content.as_ref())
            .style(match self.state.current_input == field {
                true => Style::default().fg(Color::Yellow),
                false => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(
            Paragraph::new(Spans::from(format!("{}:", field) )),
            row_layout[0]
        );
        f.render_widget(input, row_layout[1]);

        if self.state.current_input == field {
            f.set_cursor(
                row_layout[1].x + content.width() as u16 + 1,
                row_layout[1].y + 1,
            )
        }
    }


    fn save(&mut self) {
        let result :Result<i64, turbosql::Error>;
        match self.log_to_edit.as_mut() {
            Some(log) => {
                log.name = Some(self.state.name.clone());
                log.lat = self.state.latitude.parse().ok();
                log.long = self.state.longtitude.parse().ok();
                result = log.update().map(|_| log.rowid.unwrap());
            },
            None => {
                result = Data::insert_log(LogEntry {
                    name: Some(self.state.name.clone()),
                    lat: self.state.latitude.parse().ok(),
                    long: self.state.longtitude.parse().ok(),
                    ..Default::default()
                });
            }
        }


        if result.is_err() {
            self.state.has_error = true;
            self.state.error_message = format!("Error: {}", result.err().unwrap());
            return;
        }

        self.last_log_id = result.ok();
        self.close();
    }

    pub fn get_last_created_log(&self) -> Option<LogEntry> {
        if self.last_log_id.is_none() {
            return None;
        }
        Data::get_log(self.last_log_id.unwrap())
    }

    pub fn clear_last_created_log(&mut self) {
        self.last_log_id = None;
    }


    fn clear_form(&mut self) {
        for idx in 0..InputFields::LAST as u8 {
            let input = InputFields::from(idx);
            let field_content = input.to_field_mut(&mut self.state);
            field_content.clear();
        }
    }



    fn find_location(&mut self, name :&String) {
        let results = OnlineMap::query_location(&self.state.name);
        if results.is_err() {
            self.state.has_error = true;
            self.state.error_message = format!("Error: {:?}", results.err().unwrap());
            return;
        }

        let list = results.unwrap();
        if list.len() == 0 {
            self.state.has_error = true;
            self.state.error_message = format!("Error: No locations found for {}", name);
            return;
        }

        let top_location = &list[0];
        self.state.name = top_location.name.clone();
        self.state.latitude = top_location.latitude.to_string();
        self.state.longtitude = top_location.longitude.to_string();
    }
}