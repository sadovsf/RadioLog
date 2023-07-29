use std::{time::{Instant, Duration}, io::Stdout};

use crossterm::{event::{Event, self, KeyCode}, Result};
use tui::{Terminal, backend::CrosstermBackend };

use crate::{ui::{self, CreateLogDialog, AlertDialog, AlertDialogButton, AlertDialogStyle}, actions::Actions, traits::{RenderResult, EventResult, UIEvents}, common_types::RenderFrame, ui_handler::UIHandler, app_context::AppContext};
use crate::traits::UIElement;



pub struct App {
    ui_elements :UIHandler,
    dialogs :UIHandler,
    alert_dialog :Option<AlertDialog>,
}


impl App {
    pub fn new() -> App {
        let mut handler = UIHandler::default();
        handler.add(Box::new(ui::LogList::default()));
        handler.add(Box::new(ui::WorldMap::default()));
        handler.add(Box::new(ui::DetailsWindow::default()));

        let mut dialogs = UIHandler::default();
        dialogs.add(Box::new(CreateLogDialog::default()));


        App {
            alert_dialog: None,
            ui_elements: handler,
            dialogs: dialogs,
        }
    }

    pub fn run(&mut self, terminal :&mut Terminal<CrosstermBackend<Stdout>>, app_context :AppContext) -> Result<()> {
        const TICK_RATE :Duration = std::time::Duration::from_millis(60);
        let mut app_context = app_context;

        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| {
                if let Err(error) = self.draw_app(f, &mut app_context) {
                    self.pop_error(error.to_string());
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

    fn draw_app(&mut self, f :&mut RenderFrame, app_ctx :&mut AppContext) -> RenderResult {
        ///// Draw elements:
        self.ui_elements.draw(f, app_ctx)?;

        ///// Render common dialogs on top:
        self.dialogs.draw(f, app_ctx)?;
        if let Some(alert) = self.alert_dialog.as_mut() {
            alert.on_draw(f, app_ctx)?;
        }

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
    fn render(&mut self, _f :&mut RenderFrame, _app_ctx :&mut AppContext) -> RenderResult {
        Ok(())
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
