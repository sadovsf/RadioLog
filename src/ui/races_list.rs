use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{widgets::{ListItem, List, Borders, Block, ListState}, style::{Style, Color, Modifier}, prelude::Rect};

use crate::{traits::{UIElement, RenderResult, EventResult}, common_types::RenderFrame, actions::Actions, app_context::AppContext};
use super::{AlertDialogStyle, unique_ids::define_typed_element};

#[derive(Default)]
pub struct RacesList<'a> {
    state :ListState,

    border_style: Style,
    logs_cache :Vec<ListItem<'a>>,
    logs_cache_version :u32,
}
define_typed_element!(RacesList<'_>);


impl<'a> RacesList<'a> {
    fn next(&mut self, app_ctx :&mut AppContext) {
        let log_count = app_ctx.data.races.len();
        if log_count == 0 {
            self.state.select(None);
        }
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
        self.state.select(Some(i));
    }

    fn previous(&mut self, app_ctx :&mut AppContext) {
    let log_count = app_ctx.data.races.len();
        if log_count == 0 {
            self.state.select(None);
            return;
        }
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
        self.state.select(Some(i));
    }

    fn selected_race(&self, app_ctx :&mut AppContext) -> Option<i64> {
        self.state.selected().map_or(None, |i| {
            app_ctx.data.races.get_by_index(i).map_or(None, |v| Some(v.id))
        })
    }
}

impl<'a> UIElement for RacesList<'a> {
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        if self.logs_cache_version != app_ctx.data.races.get_version() {
            self.logs_cache = app_ctx.data.races
                .iter()
                .map(|race| race.clone().into())
                .collect();
            self.logs_cache_version = app_ctx.data.races.get_version();
        }

        f.render_stateful_widget(List::new(self.logs_cache.clone())
            .block(
                Block::default()
                    .title("(u)nset, (s)et, (d)elete")
                    .borders(Borders::ALL)
                    .style(self.border_style)
            )
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
            )
            .highlight_symbol(">> ")
        , rect, &mut self.state);
        Ok(())
    }

    fn on_input(&mut self, key :&KeyEvent, app_ctx :&mut AppContext) -> EventResult {
        match key.code {
            KeyCode::Up => {
                self.previous(app_ctx);
                EventResult::Handled
            },
            KeyCode::Down => {
                self.next(app_ctx);
                EventResult::Handled
            },
            KeyCode::Char('s') => {
                app_ctx.data.current_race_id = self.selected_race(app_ctx);
                EventResult::Handled
            },
            KeyCode::Char('u') => {
                app_ctx.data.current_race_id = None;
                EventResult::Handled
            },
            KeyCode::Delete => {
                if let Some(race_id) = self.selected_race(app_ctx) {
                    app_ctx.actions.add(Actions::ShowConfirm(
                        "Do you really want to delete selected race?".to_string(),
                        AlertDialogStyle::Warning,
                        Box::new(Actions::DeleteRace(race_id))
                    ));
                }
                EventResult::Handled
            }

            _ => EventResult::NotHandled
        }
    }

    fn on_action(&mut self, action :&Actions, app_ctx :&mut AppContext) -> EventResult {
        match action {
            Actions::DeleteRace(race_id) => {
                let res = app_ctx.data.races.remove(*race_id);
                if res.is_err() {
                    app_ctx.actions.add(
                        Actions::ShowError(
                            format!("Error deleting log: {}", res.err().unwrap()
                        )
                    ));
                    return EventResult::Handled;
                }
                EventResult::Handled
            }

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