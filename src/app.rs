use std::{time::{Instant, Duration}, io::Stdout};

use crossterm::{event::{Event, self, KeyCode}, Result};
use ratatui::{Terminal, backend::CrosstermBackend, prelude::Rect };

use crate::{actions::Actions, app_context::AppContext, common_types::RenderFrame, traits::{EventResult, RenderResult, UIEvents}, ui::{self, define_typed_element, AlertDialog, AlertDialogButton, AlertDialogStyle, CreateLogDialog, ManageRacesDialog}, ui_handler::UIHandler};
use crate::traits::UIElement;


const TICK_RATE :Duration = std::time::Duration::from_millis(60);
pub struct App {
    ui_elements :UIHandler,
    dialogs :UIHandler,

    alert_dialog :Option<AlertDialog>,
}
define_typed_element!(App);


impl App {
    pub fn new() -> App {
        let mut handler = UIHandler::default();
        handler.add(Box::new(ui::LogTable::default()));

        let mut dialogs = UIHandler::default();
        dialogs.add(Box::new(CreateLogDialog::default()));
        dialogs.add(Box::new(ManageRacesDialog::default()));
        dialogs.add(Box::new(ui::WorldMap::default()));


        App {
            alert_dialog: None,
            ui_elements: handler,
            dialogs,
        }
    }

    pub fn run(&mut self, terminal :&mut Terminal<CrosstermBackend<Stdout>>, app_context :AppContext) -> Result<()> {
        let mut app_context = app_context;

        let mut last_tick = Instant::now();
        let mut frame_index :u8 = 0;

        terminal.clear()?;

        loop {
            frame_index = frame_index.wrapping_add(1);
            terminal.draw(|f| {
                if let Err(error) = self.draw_app(f, frame_index, &mut app_context) {
                    self.pop_error(error.to_string());
                }

                if let Some(alert) = self.alert_dialog.as_mut() {
                    alert.on_draw(f, f.size(), &mut app_context).expect("Failed to draw alert dialog");
                }
            })?;

            let timeout = TICK_RATE
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    let event = UIEvents::Input(key);

                    let mut result = match self.alert_dialog.as_mut() {
                        Some(alert) => alert.on_event(&event, &mut app_context),
                        None => EventResult::NOOP
                    };

                    if result != EventResult::Handled {
                        result = self.dialogs.send_event(&event, &mut app_context);
                    }

                    if result != EventResult::Handled {
                        result = self.on_event(&event, &mut app_context);
                    }

                    if result != EventResult::Handled {
                        result = self.ui_elements.send_event(&event, &mut app_context);
                    }

                    if result != EventResult::Handled && key.code == KeyCode::Esc  {
                        return Ok(()); // Exit app
                    }
                }
            }

            if last_tick.elapsed() >= TICK_RATE {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn draw_app(&mut self, f :&mut RenderFrame, frame_index :u8, app_ctx :&mut AppContext) -> RenderResult {
        ///// Draw elements:
        self.ui_elements.draw_all(frame_index, f, app_ctx)?;

        ///// Render common dialogs on top:
        self.dialogs.draw_all(frame_index, f, app_ctx)?;

        ///// Process accumulated actions:
        self.process_actions(app_ctx);
        Ok(())
    }

    fn process_actions(&mut self, app_context :&mut AppContext) {
        // Make local copy to avoid writing into actions while iterating.
        let local_actions = app_context.actions.clone();
        app_context.actions.clear();

        for action in local_actions.iter() {
            let event = UIEvents::Action(action);
            let result = self.on_event(&event, app_context);
            if result != EventResult::Handled {
                self.dialogs.send_event(&event, app_context);
            }
            if result != EventResult::Handled {
                self.ui_elements.send_event(&event, app_context);
            }
        }
    }

    fn on_tick(&mut self) {

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
    implement_typed_element!();

    fn render(&mut self, _f :&mut RenderFrame, _rect :Rect, _app_ctx :&mut AppContext) -> RenderResult {
        Ok(())
    }

    fn on_input(&mut self, key :&event::KeyEvent, app_ctx :&mut AppContext) -> EventResult {
        match key.code {
            KeyCode::Tab => {
                self.ui_elements.focus_next();
                EventResult::Handled
            },
            KeyCode::Char('m') => {
                app_ctx.actions.add(Actions::ToggleMap);
                EventResult::Handled
            }
            _ => EventResult::NotHandled
        }
    }

    fn on_action(&mut self, action :&Actions, _app_ctx :&mut AppContext) -> EventResult {
        match action {
            Actions::ShowError(text) => {
                self.pop_error(text.clone());
                EventResult::Handled
            },

            Actions::ShowConfirm(msg, style, on_confirm) => {
                self.pop_confirm(msg.clone(), style.clone(), Some((**on_confirm).clone()));
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }
}
