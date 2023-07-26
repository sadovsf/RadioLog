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
        handler.add(Box::new(ui::LogList::default()));

        App {
            state: UIState::default(),

            alert_dialog: None,
            ui_elements: handler,
        }
    }

    pub fn run(&mut self, terminal :&mut Terminal<CrosstermBackend<Stdout>>, actions :&mut ActionProcessor) -> Result<()> {
        const TICK_RATE :Duration = std::time::Duration::from_millis(60);

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

        ui::WorldMap::render(f, rects[1], &self.state.world_map_state);
        self.ui_elements.draw(f, actions);

        ///// Dialogs:
        if let Some(alert) = self.alert_dialog.as_mut() {
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
    fn render(&mut self, _f :&mut RenderFrame, _actions :&mut ActionProcessor) -> RenderResult {
        RenderResult::NOOP
    }

    fn on_action(&mut self, action :&Actions, _actions :&mut ActionProcessor) -> EventResult {
        match action {
            Actions::ShowError(text) => {
                self.pop_error(text.clone());
                EventResult::Handled
            },

            Actions::FocusLog(log_id) => {
                if let Some(log_id) = log_id {
                    if let Some(log) = Data::get_log(*log_id) {
                        self.state.world_map_state.selected_position = log.position();
                    } else {
                        self.state.world_map_state.selected_position = None;
                    }
                } else {
                    self.state.world_map_state.selected_position = None;
                }
                EventResult::NotHandled
            }

            Actions::ShowConfirm(msg, style, on_confirm) => {
                self.pop_confirm(msg.clone(), style.clone(), Some((**on_confirm).clone()));
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }

    fn on_input(&mut self, key :&KeyEvent, _actions :&mut ActionProcessor) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::NotHandled;
        }

        match key.code {

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

            _ => EventResult::NotHandled
        }
    }
}
