extern crate unicode_width;

use unicode_width::UnicodeWidthStr;

use crossterm::event::{KeyEvent, KeyCode};
use tui::{backend::Backend, Frame, layout::{Rect, Layout, Direction, Constraint}, widgets::{Block, Clear, Borders, Paragraph}, style::{Style, Color}, text::Span};

use crate::{data::LogEntry, map_api::OnlineMap, traits::{DialogHelpers, EventResult, RenderResult, UIElement}, actions::Actions, common_types::RenderFrame, app_context::AppContext};

mod input_fields;
use input_fields::InputFields;
use crate::traits::DialogInterface;

use super::define_typed_element;


pub struct CreateLogDialogState {
    opened: bool,

    current_input: InputFields,

    name: String,
    latitude: String,
    longtitude: String,
}

impl Default for CreateLogDialogState {
    fn default() -> Self {
        Self {
            opened: false,

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
    log_to_edit: Option<i64>,
}
define_typed_element!(CreateLogDialog);


impl CreateLogDialog {
    pub fn edit(&mut self, log :&LogEntry) {
        if log.rowid.is_none() {
            return;
        }

        self.state.opened = true;
        self.state.name = log.name.clone().unwrap_or("".to_string());
        self.state.latitude = log.lat.map(|v| v.to_string()).unwrap_or("".to_string());
        self.state.longtitude = log.long.map(|v| v.to_string()).unwrap_or("".to_string());
        self.log_to_edit = log.rowid;
    }

    fn create_input<'a, B: Backend>(&'a self, f :&mut Frame<B>, content :&'a String, field :InputFields, area :Rect) {
        let row_layout = Layout::default()
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

        let input = Paragraph::new(content.clone())
            .style(match self.state.current_input == field {
                true => Style::default().fg(Color::Yellow),
                false => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(
            Paragraph::new(Span::raw(format!("{}:", field) )),
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


    fn save(&mut self, app_ctx :&mut AppContext) {
        match self.log_to_edit.as_mut() {
            Some(row_id) => {

                let result = app_ctx.data.logs.edit(LogEntry{
                    rowid: Some(*row_id),
                    name: Some(self.state.name.clone()),
                    lat: self.state.latitude.parse().ok(),
                    long: self.state.longtitude.parse().ok(),
                    ..Default::default()
                });
                if result.is_err() {
                    app_ctx.actions.add(Actions::ShowError(format!("Error: {:?}", result.err().unwrap())));
                    return;
                }
            },
            None => {
                let res = app_ctx.data.logs.add(LogEntry{
                    name: Some(self.state.name.clone()),
                    lat: self.state.latitude.parse().ok(),
                    long: self.state.longtitude.parse().ok(),
                    ..Default::default()
                });
                if res.is_err() {
                    app_ctx.actions.add(Actions::ShowError(format!("Error creating log: {:?}", res.err().unwrap())));
                }
            }
        }
        self.close();
    }


    fn clear_form(&mut self) {
        for idx in 0..InputFields::LAST as u8 {
            let input = InputFields::from(idx);
            let field_content = input.to_field_mut(&mut self.state);
            field_content.clear();
        }
        self.state.current_input = InputFields::Name;
    }



    fn find_location(&mut self, name :&String, app_ctx :&mut AppContext) {
        let results = OnlineMap::query_location(&self.state.name);
        if results.is_err() {
            app_ctx.actions.add(Actions::ShowError(format!("Error: {:?}", results.err().unwrap())));
            return;
        }

        let list = results.unwrap();
        if list.len() == 0 {
            app_ctx.actions.add(Actions::ShowError(format!("Error: No locations found for {}", name)));
            return;
        }

        let top_location = &list[0];
        self.state.name = top_location.name.clone();
        self.state.latitude = top_location.latitude.to_string();
        self.state.longtitude = top_location.longitude.to_string();
    }
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
}

impl UIElement for CreateLogDialog {
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, _app_ctx :&mut AppContext) -> RenderResult {
        if ! self.is_opened() {
            return Ok(());
        }

        let area = DialogHelpers::center_rect_perc(50, 35, rect);
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
        };

        Ok(())
    }

    fn on_input(&mut self, key :&KeyEvent, app_ctx :&mut AppContext) -> EventResult {
        if ! self.is_opened() {
            return EventResult::NOOP;
        }

        match key.code {
            KeyCode::Esc => self.close(),
            KeyCode::Tab => self.state.current_input = self.state.current_input.next(),
            KeyCode::BackTab => self.state.current_input = self.state.current_input.prev(),
            KeyCode::Delete => self.clear_form(),
            KeyCode::Enter => self.save(app_ctx),
            KeyCode::Backspace => {
                self.state.current_input.to_field_mut(&mut self.state).pop();
            },
            KeyCode::Char(c) => {
                self.state.current_input.to_field_mut(&mut self.state).push(c);
            },
            KeyCode::PageDown => self.find_location(&self.state.name.clone(), app_ctx),
            _ => {}
        };
        EventResult::Handled
    }

    fn on_action(&mut self, action :&Actions, app_ctx :&mut AppContext) -> EventResult {
        match action {
            Actions::EditLog(rowid) => {
                if let Some(log) = app_ctx.data.logs.get(*rowid) {
                    self.edit(log);
                }
                EventResult::Handled
            },
            Actions::CreateLogWanted => {
                self.open();
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }
}