extern crate unicode_width;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{layout::{Rect, Layout, Direction, Constraint}, widgets::{Block, Clear, Borders}};

use crate::{data::{LogEntry, position::Position}, map_api::OnlineMap, traits::{DialogHelpers, EventResult, RenderResult, UIElement}, actions::Actions, common_types::RenderFrame, app_context::AppContext};

mod input_fields;
use input_fields::InputFields;
use crate::traits::DialogInterface;

use super::{define_typed_element, Input};

mod existing_logs_window;
use existing_logs_window::ExistingLogsWindow;

pub struct CreateLogDialogState {
    opened: bool,
    current_input: InputFields,
}

impl Default for CreateLogDialogState {
    fn default() -> Self {
        Self {
            opened: false,
            current_input: InputFields::Call,
        }
    }
}




pub struct CreateLogDialog {
    state: CreateLogDialogState,
    log_to_edit: Option<i64>,
    inputs: Vec<Input>,
}
define_typed_element!(CreateLogDialog);



impl Default for CreateLogDialog {
    fn default() -> Self {
        let mut me = Self {
            state: CreateLogDialogState::default(),
            log_to_edit: None,
            inputs: vec!(),
        };

        for idx in 0..InputFields::LAST as u8 {
            me.inputs.push(
                Input::default()
                    .set_label(InputFields::from(idx).to_string())
            );
        }
        me.set_focus(InputFields::Call);
        return me;
    }
}


impl CreateLogDialog {

    fn get_field(&self, field :InputFields) -> &String {
        self.inputs[field as usize].get()
    }

    fn set_field(&mut self, field :InputFields, value :String) {
        self.inputs[field as usize].set(value);
    }

    pub fn edit(&mut self, log :&LogEntry) {
        self.state.opened = true;
        self.set_field(InputFields::Call, log.call.clone());
        self.set_field(InputFields::Code, log.code.clone().unwrap_or("".to_string()));
        self.set_field(InputFields::QTH, log.position().map(|v| v.to_qth()).unwrap_or("".to_string()));

        self.log_to_edit = Some(log.id);
    }


    fn save(&mut self, app_ctx :&mut AppContext) {

        match self.log_to_edit.as_mut() {
            Some(row_id) => {
                let log = app_ctx.data.logs.get(*row_id);
                match log.map(|v| v.clone()) {
                    Some(mut log) => {
                        log.call = self.get_field(InputFields::Call).clone();
                        log.locator = self.get_field(InputFields::QTH).clone();
                        log.code = Some(self.get_field(InputFields::Code).clone());
                        let result = app_ctx.data.logs.edit(log.clone());
                        if result.is_err() {
                            app_ctx.actions.add(Actions::ShowError(format!("Error: {:?}", result.err().unwrap())));
                        }
                    },
                    None => {
                        app_ctx.actions.add(Actions::ShowError(format!("Error: Log with id {} not found", row_id)));
                    }
                }
            },
            None => {
                let res = app_ctx.data.logs.add(LogEntry{
                    call: self.get_field(InputFields::Call).clone(),
                    locator: self.get_field(InputFields::QTH).clone(),
                    code: Some(self.get_field(InputFields::Code).clone()),
                    race_id: app_ctx.data.current_race_id,
                    ..Default::default()
                });
                if res.is_err() {
                    app_ctx.actions.add(Actions::ShowError(format!("Error creating log: {:?}", res.err().unwrap())));
                }
            }
        }
        self.close();
    }

    fn set_focus(&mut self, field :InputFields) {
        let old_focus = &mut self.inputs[self.state.current_input as usize];
        old_focus.set_focused(false);

        self.state.current_input = field;

        let new_focus = &mut self.inputs[self.state.current_input as usize];
        new_focus.set_focused(true);
    }

    fn get_focused(&mut self) -> &mut Input {
        &mut self.inputs[self.state.current_input as usize]
    }

    fn clear_form(&mut self) {
        for idx in 0..InputFields::LAST as usize {
            self.inputs[idx].clear();
        }

        self.set_focus(InputFields::Call);
    }



    fn find_location(&mut self, name :&String, app_ctx :&mut AppContext) {
        let results = OnlineMap::query_location(&self.get_field(InputFields::Call));
        if results.is_err() {
            app_ctx.actions.add(Actions::ShowError(format!("Error: {:?}", results.err().unwrap())));
            return;
        }

        let list = results.unwrap();
        if list.len() == 0 {
            app_ctx.actions.add(Actions::ShowError(format!("Error: No locations found for {}", name)));
            return;
        }

        let top_location: &crate::map_api::LocationResult = &list[0];
        let parts :Vec<&str> = top_location.name.splitn(3, ',').collect();

        if parts.len() >= 2 {
            self.set_field(InputFields::Call, format!("{},{}", parts[0], parts[1]));
        } else {
            self.set_field(InputFields::Call, parts[0].to_string());
        }

        let top_position = Position::new(top_location.latitude, top_location.longitude);
        self.set_field(InputFields::QTH, top_position.to_qth());
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

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        if ! self.is_opened() {
            return Ok(());
        }

        let mut area = DialogHelpers::center_rect_size(rect.width / 2, 13, rect);
        area.x += rect.width / 5;
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(
            Block::default().title("Create log").borders(Borders::ALL),
            area
        );

        let mut constraints = vec!();
        for _ in 0..InputFields::LAST as u8 {
            constraints.push(Constraint::Length(3));
        }
        constraints.push(Constraint::Length(1));


        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints.as_ref())
            .split(area);

        for (idx, input) in self.inputs.iter_mut().enumerate() {
            input.on_draw(f, popup_layout[idx], app_ctx)?;
        };


        let logs_window = ExistingLogsWindow::from_search(
            app_ctx,
            self.get_field(InputFields::Call)
        );
        if logs_window.is_ok() {
            logs_window.unwrap().on_draw(f, Rect {
                x: 0,
                y: 0,
                width: 50,
                height: 10
            }, app_ctx)?;
        } else {
            app_ctx.actions.add(Actions::ShowError(format!("Error: {:?}", logs_window.err().unwrap())));
        }

        Ok(())
    }

    fn on_input(&mut self, key :&KeyEvent, app_ctx :&mut AppContext) -> EventResult {
        if ! self.is_opened() {
            return EventResult::NOOP;
        }

        match key.code {
            KeyCode::Esc => self.close(),
            KeyCode::Tab => self.set_focus(self.state.current_input.next()),
            KeyCode::BackTab => self.set_focus(self.state.current_input.prev()),
            KeyCode::Enter => self.save(app_ctx),
            KeyCode::PageDown => self.find_location(&self.get_field(InputFields::Call).clone(), app_ctx),
            KeyCode::F(2) => self.clear_form(),
            _ => {
                self.get_focused().on_input(key, app_ctx);
            }
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