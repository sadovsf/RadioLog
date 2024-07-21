use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::{Constraint, Direction, Layout, Rect}, widgets::{Block, Borders, Clear, Paragraph}};

use crate::{traits::{DialogInterface, UIElement, RenderResult, DialogHelpers, EventResult, UIEvents}, common_types::RenderFrame, app_context::AppContext, actions::Actions, ui_handler::{UIHandler, UIElementID}, data::Race};

use super::{define_typed_element, RacesList, Input};


#[derive(Default)]
pub struct ManageRacesDialogState {
    opened: bool,
    edited_race_id: Option<i64>,
}

pub struct ManageRacesDialog {
    state: ManageRacesDialogState,
    frame_index: u8,

    handler: UIHandler,
    race_list: UIElementID,
    race_name_inp: UIElementID,
    race_my_loc_inp: UIElementID,
    race_my_call_inp: UIElementID,
}
define_typed_element!(ManageRacesDialog);


impl Default for ManageRacesDialog {
    fn default() -> Self {
        let mut handler = UIHandler::default();
        let race_list = handler.add(Box::new(RacesList::default()));
        let race_name_inp = handler.add(Box::new(Input::default().set_label("Race name".to_string())));
        let race_my_loc_inp = handler.add(Box::new(Input::default().set_label("My location".to_string())));
        let race_my_call_inp = handler.add(Box::new(Input::default().set_label("My call".to_string())));

        Self {
            state: ManageRacesDialogState::default(),
            frame_index: 0,
            handler,
            race_list,
            race_name_inp,
            race_my_loc_inp,
            race_my_call_inp
        }
    }
}


impl ManageRacesDialog {
    fn save(&mut self, app_ctx :&mut AppContext) {
        let race_name = self.get_val(self.race_name_inp);
        if race_name.len() < 3 {
            app_ctx.actions.add(Actions::ShowError("You have to provide race name with at least 3 characters".to_string()));
            return;
        }

        let res;
        if let Some(race_id) = self.state.edited_race_id {
            match app_ctx.data.races.get(race_id).map(|race| race.clone()) {
                Some(mut race) => {
                    race.my_call = self.get_val(self.race_my_call_inp);
                    race.my_location = self.get_val(self.race_my_loc_inp);
                    race.name = self.get_val(self.race_name_inp);
                    res = app_ctx.data.races.edit(race);

                    self.state.edited_race_id = None;
                }
                None => {
                    app_ctx.actions.add(Actions::ShowError("Unable to update selected race".to_string()));
                    return;
                }
            }
        } else {
            res = app_ctx.data.races.add(Race {
                name: race_name,
                my_location: self.get_val(self.race_my_loc_inp),
                my_call: self.get_val(self.race_my_call_inp),
                ..Default::default()
            });
        }

        if res.is_err() {
            app_ctx.actions.add(Actions::ShowError(format!("Error managing race: {:?}", res.err().unwrap())));
        }
    }

    fn clear_inputs(&mut self) {
        self.state.edited_race_id = None;
        [
            &self.race_name_inp,
            &self.race_my_loc_inp,
            &self.race_my_call_inp
        ].iter().for_each(|id| {
            self.handler.get::<Input>(id).expect("Invalid UI state").clear();
        });
    }

    fn get_val(&mut self, id :UIElementID) -> String {
        self.handler.get::<Input>(&id).expect("Invalid UI state").get().clone()
    }

    fn set_val(&mut self, id :UIElementID, val :String) {
        self.handler.get::<Input>(&id).expect("Invalid UI state").set(val);
    }

    fn edit_race(&mut self, id :i64, app_ctx :&mut AppContext) {
        self.clear_inputs();

        if let Some(race) = app_ctx.data.races.get(id) {
            self.state.edited_race_id = Some(id);
            self.set_val(self.race_name_inp, race.name.clone());
            self.set_val(self.race_my_loc_inp, race.my_location.clone());
            self.set_val(self.race_my_call_inp, race.my_call.clone());
        }
    }
}



impl DialogInterface for ManageRacesDialog {
    fn set_opened(&mut self, opened :bool) {
        self.state.opened = opened;
    }

    fn is_opened(&self) -> bool {
        self.state.opened
    }

    fn open(&mut self) {
        self.clear_inputs();
        self.set_opened(true);
    }

    fn close(&mut self) {
        self.set_opened(false);
    }
}

impl UIElement for ManageRacesDialog {
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        if ! self.is_opened() {
            return Ok(());
        }

        let area = DialogHelpers::center_rect_size((rect.width / 4) * 3, 15, rect);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(
            Block::default().title("Manage races").borders(Borders::ALL),
            area
        );

        let layout = Layout::default()
            .margin(2)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Min(1),
            ]).split(area);

        let inputs_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(2)
                ]).split(layout[1]);

        self.frame_index = self.frame_index.wrapping_add(1);

        let help_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Min(1),
            ]).split(inputs_layout[3]);
        f.render_widget(Paragraph::new("CTRL+S to save"), help_layout[1]);

        self.handler.draw_single(&self.race_list, self.frame_index, f, layout[0], app_ctx)?;
        self.handler.draw_single(&self.race_name_inp, self.frame_index, f, inputs_layout[0], app_ctx)?;
        self.handler.draw_single(&self.race_my_loc_inp, self.frame_index, f, inputs_layout[1], app_ctx)?;
        self.handler.draw_single(&self.race_my_call_inp, self.frame_index, f, inputs_layout[2], app_ctx)?;
        self.handler.draw_all(self.frame_index, f, app_ctx)?;

        Ok(())
    }

    fn on_input(&mut self, key :&KeyEvent, app_ctx :&mut AppContext) -> EventResult {
        if key.code == KeyCode::Char('r') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.open();
            return EventResult::Handled;
        }
        if ! self.is_opened() {
            return EventResult::NOOP;
        }

        match key.code {
            KeyCode::Esc => {
                self.close();
                return EventResult::Handled;
            },
            KeyCode::Tab => {
                self.handler.focus_next();
                return EventResult::Handled;
            },
            KeyCode::BackTab => {
                self.handler.focus_previous();
                return EventResult::Handled;
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.save(app_ctx);
                self.clear_inputs();
                return EventResult::Handled;
            }
            _ => {}
        }

        self.handler.send_event(&UIEvents::Input(key.clone()), app_ctx);
        EventResult::Handled
    }

    fn on_action(&mut self, action :&Actions, app_ctx :&mut AppContext) -> EventResult {
        match action {
            &Actions::EditRace(race_id) => {
                self.open();
                self.edit_race(race_id, app_ctx);
                return EventResult::Handled;
            }
            _ => {}
        }
        EventResult::NotHandled
    }
}