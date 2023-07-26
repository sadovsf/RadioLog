use std::{time::{Instant, Duration}, io::Stdout};

use crossterm::{event::{Event, self, KeyCode, KeyEventKind, KeyEvent}, Result};
use tui::{layout::{Direction, Layout, Constraint}, Terminal, backend::CrosstermBackend };

use crate::{ui::{self, UIState, CreateLogDialog, AlertDialog, AlertDialogButton, AlertDialogStyle, DetailsWindow}, data::Data, actions::{Actions, ActionProcessor}, traits::{RenderResult, EventResult, UIEvents}, common_types::RenderFrame, ui_handler::UIHandler};
use crate::traits::UIElement;



pub struct App {
    state: UIState,

    ui_elements :UIHandler,
    alert_dialog :Option<AlertDialog>,
}


impl App {
    pub fn new() -> App {
        let mut handler = UIHandler::default();
        handler.add(Box::new(DetailsWindow::default()));
        handler.add(Box::new(CreateLogDialog::default()));

        App {
            state: UIState::default(),

            alert_dialog: None,
            ui_elements: handler,
        }
    }

    pub fn run(&mut self, terminal :&mut Terminal<CrosstermBackend<Stdout>>, actions :&mut ActionProcessor) -> Result<()> {
        const TICK_RATE :Duration = std::time::Duration::from_millis(250);

        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| {
                self.draw_app(f, actions);
            })?;

            let timeout = TICK_RATE
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    let event = UIEvents::Input(key);
                    let mut result = match self.alert_dialog.as_mut() {
                        Some(alert) => alert.on_event(&event, actions),
                        None => EventResult::NOOP
                    };

                    if result != EventResult::Handled {
                        result = self.ui_elements.send_event(&event, actions);
                    }

                    if result != EventResult::Handled {
                        match key.code {
                            KeyCode::Esc => return Ok(()),
                            _ => {
                                self.on_input(&key, actions);
                            }
                        }
                    }
                }
            }

            if last_tick.elapsed() >= TICK_RATE {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn draw_app(&mut self, f :&mut RenderFrame, actions :&mut ActionProcessor) {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ].as_ref()
            )
            .split(f.size());

        let log_block = ui::LogList::default();
        f.render_stateful_widget(log_block, rects[0], &mut self.state.log_list_state);

        ui::WorldMap::render(f, rects[1], &self.state.world_map_state);
        self.ui_elements.draw(f, actions);

        ///// Dialogs:
        if let Some(alert) = self.alert_dialog.as_ref() {
            alert.on_draw(f, actions);
        }



        ///// Accumulated actions processing:
        self.process_actions(actions);
    }

    fn process_actions(&mut self, actions :&mut ActionProcessor) {
        let local_actions = actions.clone();
        actions.clear();

        for action in local_actions.iter() {
            let event = UIEvents::Action(action);
            let result = self.on_event(&event, actions);
            if result != EventResult::Handled {
                self.ui_elements.send_event(&event, actions);
            }
        }
    }

    fn select_log(&mut self, id :i64, actions :&mut ActionProcessor) {
        self.state.log_list_state.select(id);
        self.state.world_map_state.selected_position = self.state.log_list_state.selected_location();
        self.ui_elements.send_event(&UIEvents::Action(
            &Actions::FocusLog(
                Some(id)
            )
        ), actions);
    }

    fn deselect_all(&mut self, actions :&mut ActionProcessor) {
        self.state.log_list_state.deselect();
        self.state.world_map_state.selected_position = None;

        self.ui_elements.send_event(&UIEvents::Action(
            &Actions::FocusLog(None)
        ), actions);
    }

    fn on_tick(&mut self) {

    }

    fn zoom_map(&mut self, zoom :f64) {
        let old_center = ui::WorldMap::map_center(&self.state.world_map_state);
        self.state.world_map_state.zoom += zoom;
        self.state.world_map_state.zoom = self.state.world_map_state.zoom.clamp(0.05, 5.0);
        let new_center = ui::WorldMap::map_center(&self.state.world_map_state);

        self.state.world_map_state.top_left.longitude += old_center.longitude - new_center.longitude;
        self.state.world_map_state.top_left.latitude += old_center.latitude - new_center.latitude;
    }

    fn pop_error(&mut self, text :String) {
        self.alert_dialog = Some(AlertDialog::new(
            text,
            AlertDialogButton::OK,
            AlertDialogStyle::Error,
            None)
        );
    }

    fn pop_confirm(&mut self, text :String, style :AlertDialogStyle, action_after :Option<Actions>) {
        self.alert_dialog = Some(AlertDialog::new(
            text,
            AlertDialogButton::YES | AlertDialogButton::NO,
            style,
            action_after
        ));
    }
}



impl UIElement for App {
    fn render(&self, _f :&mut RenderFrame, _actions :&mut ActionProcessor) -> RenderResult {
        RenderResult::NOOP
    }

    fn on_action(&mut self, action :&Actions, actions :&mut ActionProcessor) -> EventResult {
        match action {
            Actions::DeleteLog(log_id) => {
                let deselect = self.state.log_list_state.selected() == Some(*log_id);
                let res = Data::delete_log(*log_id);
                if res.is_err() {
                    self.pop_error(format!("Error deleting log: {}", res.err().unwrap()));
                    return EventResult::Handled;
                }

                if deselect {
                    self.state.world_map_state.selected_position = None;
                    self.state.log_list_state.deselect();
                }
                EventResult::Handled
            },

            Actions::CreateLog(log_data) => {
                let res = Data::insert_log(log_data);
                if res.is_err() {
                    self.pop_error(format!("Error creating log: {}", res.err().unwrap()));
                    return EventResult::Handled;
                }
                self.select_log(res.unwrap(), actions);
                EventResult::Handled
            }

            Actions::ShowError(text) => {
                self.pop_error(text.clone());
                EventResult::Handled
            },

            Actions::FocusLog(log_id) => {
                match log_id {
                    Some(log_id) => self.select_log(*log_id, actions),
                    None => self.deselect_all(actions)
                };
                EventResult::Handled
            }

            _ => EventResult::NotHandled
        }
    }

    fn on_input(&mut self, key :&KeyEvent, actions :&mut ActionProcessor) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::NotHandled;
        }

        match key.code {
            KeyCode::Down => {
                let id = self.state.log_list_state.next();
                self.select_log(id, actions);
                EventResult::Handled
            },
            KeyCode::Up => {
                let id = self.state.log_list_state.previous();
                self.select_log(id, actions);
                EventResult::Handled
            },

            KeyCode::Left => {
                self.deselect_all(actions);
                EventResult::Handled
            },

            KeyCode::Enter => {
                if let Some(log_id) = self.state.log_list_state.selected() {
                    self.ui_elements.send_event(&UIEvents::Action(
                        &Actions::EditLog(log_id)
                    ), actions);
                }
                EventResult::Handled
            },

            // Map controls:
            KeyCode::Char('+') => {
                self.zoom_map(-0.05);
                EventResult::Handled
            },
            KeyCode::Char('-') => {
                self.zoom_map(0.05);
                EventResult::Handled
            },

            KeyCode::Char('8') => {
                self.state.world_map_state.top_left.latitude += 5.0;
                EventResult::Handled
            },
            KeyCode::Char('5') => {
                self.state.world_map_state.top_left.latitude -= 5.0;
                EventResult::Handled
            },
            KeyCode::Char('4') => {
                self.state.world_map_state.top_left.longitude -= 5.0;
                EventResult::Handled
            },
            KeyCode::Char('6') => {
                self.state.world_map_state.top_left.longitude += 5.0;
                EventResult::Handled
            },

            KeyCode::Delete => {
                let to_del = self.state.log_list_state.selected();
                if to_del.is_none() {
                    return EventResult::NotHandled;
                }

                let log_info = Data::get_log(to_del.unwrap()).unwrap();
                self.pop_confirm(
                    format!("Are you sure you want to delete log '{}'?", log_info.name.unwrap()),
                    AlertDialogStyle::Warning,
                    Some(
                        Actions::DeleteLog(to_del.unwrap())
                    )
                );
                EventResult::Handled
            },

            KeyCode::Char('a') => {
                self.ui_elements.send_event(&UIEvents::Action(
                    &Actions::CreateLogWanted
                ), actions);
                EventResult::Handled
            },
            _ => EventResult::NotHandled
        }
    }
}
