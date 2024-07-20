use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{widgets::{TableState, Table, Block, Borders, Row}, prelude::Rect, style::{Style, Modifier, Color}};
use crate::{traits::{UIElement, RenderResult, EventResult}, app_context::AppContext, common_types::RenderFrame, actions::Actions, data::LogEntry};

use super::{define_typed_element, AlertDialogStyle};



#[derive(Default)]
pub struct LogTable {
    state :TableState,
    border_style: Style,
}
define_typed_element!(LogTable);


impl LogTable {
    fn next(&self, app_ctx :&mut AppContext) -> i64 {
        let current_race_id = app_ctx.data.current_race_id;
        let log_count = app_ctx.data.race_logs(current_race_id).count();
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
        app_ctx.data.logs.get_by_index(i).unwrap().id
    }

    fn previous(&self, app_ctx :&mut AppContext) -> i64 {
        let current_race_id = app_ctx.data.current_race_id;
        let log_count = app_ctx.data.race_logs(current_race_id).count();
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
        app_ctx.data.logs.get_by_index(i).unwrap().id
    }

    fn selected_log(&self, app_ctx :&mut AppContext) -> Option<i64> {
        if let Some(i) = self.state.selected() {
            let current_race_id = app_ctx.data.current_race_id;
            return app_ctx.data.race_logs(current_race_id).nth(i).map_or(None, |item| Some(item.id))
        }
        None
    }
}



impl UIElement for LogTable {
    implement_typed_element!();
    fn render(&mut self, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header = LogEntry::table_header()
            .style(normal_style)
            .height(1);

        let current_race_id = app_ctx.data.current_race_id;
        let rows :Vec<Row> = app_ctx.data.race_logs(current_race_id)
            .map(|item| item.table_row(app_ctx)).collect();

        if let Some(sel_index) = self.state.selected() {
            if rows.len() == 0 {
                self.state.select(None);
            } else if sel_index >= rows.len() {
                self.state.select(Some(rows.len() - 1));
            }
        }

        let t = Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .border_style(self.border_style)
                    .borders(Borders::ALL).title("Logs - (a)dd, CTRL+(r)aces, toggle (m)ap")
            )
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(LogEntry::table_column_constraints());
        f.render_stateful_widget(t, rect, &mut self.state);
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
                if let Some(log_id) = self.selected_log(app_ctx) {
                    app_ctx.actions.add(Actions::EditLog(log_id));
                }
                EventResult::Handled
            },

            KeyCode::Delete => {
                let to_del = self.selected_log(app_ctx);
                if to_del.is_none() {
                    return EventResult::NotHandled;
                }

                let log_info :&LogEntry = app_ctx.data.logs.get(to_del.unwrap()).unwrap();
                app_ctx.actions.add(Actions::ShowConfirm(
                    format!("Are you sure you want to delete log '{}'?", log_info.call),
                    AlertDialogStyle::Warning,
                    Box::new(Actions::DeleteLog(log_info.id))
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

    fn set_focused(&mut self, focused :bool) {
        if focused {
            self.border_style = Style::default().fg(Color::Yellow);
        } else {
            self.border_style = Style::default();
        }
    }
}