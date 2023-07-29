use crossterm::event::{KeyEvent, KeyCode};
use tui::{layout::{Layout, Direction, Constraint}, text::Span, widgets::{ListItem, List, Borders, Block, ListState}, style::{Style, Color, Modifier}};

use crate::{traits::{UIElement, RenderResult, EventResult}, common_types::RenderFrame, actions::Actions, app_context::AppContext, data::LogEntry};

use super::AlertDialogStyle;

#[derive(Default)]
pub struct LogList<'a> {
    state :ListState,

    logs_cache :Vec<ListItem<'a>>,
    logs_cache_version :u32,
}

impl<'a> LogList<'a> {
    pub fn next(&self, app_ctx :&mut AppContext) -> i64 {
        let log_count = app_ctx.data.logs.len();
        let i = match self.state.selected() {
            Some(i) => {
                if i >= log_count - 1 {
                    0
                } else {
                    i + 1
                }
            },
            None => 0,
        };
        app_ctx.data.logs.get_by_index(i).unwrap().rowid.unwrap()
    }

    pub fn previous(&self, app_ctx :&mut AppContext) -> i64 {
        let log_count = app_ctx.data.logs.len();
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    log_count - 1
                } else {
                    i - 1
                }
            },
            None => 0,
        };
        app_ctx.data.logs.get_by_index(i).unwrap().rowid.unwrap()
    }
}

impl<'a> UIElement for LogList<'a> {
    fn render(&mut self, f :&mut RenderFrame, app_ctx :&mut AppContext) -> RenderResult {
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

        if self.logs_cache_version != app_ctx.data.logs.get_version() {
            self.logs_cache = app_ctx.data.logs
                .iter()
                .map(|log| {
                    if log.name.is_none() {
                        return ListItem::new(Span::raw("Unknown"));
                    }
                    let span = Span::raw(log.name.as_ref().unwrap().clone());
                    ListItem::new(span)
                })
                .collect();
            self.logs_cache_version = app_ctx.data.logs.get_version();
        }



        f.render_stateful_widget(List::new(self.logs_cache.clone())
            .block(Block::default().title("Logs").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
            )
            .highlight_symbol(">> ")
        , rects[0], &mut self.state);
        Ok(())
    }

    fn on_input(&mut self, key :&KeyEvent, app_ctx :&mut AppContext) -> EventResult {
        match key.code {
            KeyCode::Up => {
                let new_id = self.previous(app_ctx);
                app_ctx.actions.add(Actions::FocusLog(Some(new_id)));
                EventResult::Handled
            },
            KeyCode::Down => {
                let new_id = self.next(app_ctx);
                app_ctx.actions.add(Actions::FocusLog(Some(new_id)));
                EventResult::Handled
            },
            KeyCode::Left => {
                app_ctx.actions.add(Actions::FocusLog(None));
                EventResult::Handled
            },

            KeyCode::Enter => {
                if let Some(log_idx) = self.state.selected() {
                    let log = app_ctx.data.logs.get_by_index(log_idx).unwrap();
                    app_ctx.actions.add(Actions::EditLog(log.rowid.unwrap()));
                }
                EventResult::Handled
            },

            KeyCode::Delete => {
                let to_del = self.state.selected();
                if to_del.is_none() {
                    return EventResult::NotHandled;
                }

                let log_info :&LogEntry = app_ctx.data.logs.get_by_index(to_del.unwrap()).unwrap();
                app_ctx.actions.add(Actions::ShowConfirm(
                    format!("Are you sure you want to delete log '{}'?", log_info.name.as_ref().unwrap()),
                    AlertDialogStyle::Warning,
                    Box::new(Actions::DeleteLog(log_info.rowid.unwrap()))
                ));
                EventResult::Handled
            },

            KeyCode::Char('a') => {
                app_ctx.actions.add(Actions::CreateLogWanted);
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }

    fn on_action(&mut self, action :&Actions, app_ctx :&mut AppContext) -> EventResult {
        match action {
            Actions::FocusLog(log_id) => {
                match log_id {
                    Some(id) => self.state.select(app_ctx.data.logs.find_index_of(*id)),
                    None => self.state.select(None),
                }
                EventResult::NotHandled
            },
            Actions::DeleteLog(log_id) => {
                let res = app_ctx.data.logs.remove(*log_id);
                if res.is_err() {
                    app_ctx.actions.add(
                        Actions::ShowError(
                            format!("Error deleting log: {}", res.err().unwrap()
                        )
                    ));
                    return EventResult::Handled;
                }
                app_ctx.actions.add(Actions::FocusLog(None));
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }
}