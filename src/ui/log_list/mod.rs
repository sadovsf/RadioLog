use crossterm::event::{KeyEvent, KeyCode};
use tui::layout::{Layout, Direction, Constraint};

use crate::{traits::{UIElement, RenderResult, EventResult}, common_types::RenderFrame, actions::{ActionProcessor, Actions}, data::Data};

use super::AlertDialogStyle;

mod log_list_widget;

#[derive(Default)]
pub struct LogList {
    state :log_list_widget::LogListState,
}

impl LogList {

}

impl UIElement for LogList {
    fn render(&mut self, f :&mut RenderFrame, _actions :&mut ActionProcessor) -> RenderResult {
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

        f.render_stateful_widget(log_list_widget::LogListWidget::default(), rects[0], &mut self.state);
        RenderResult::Rendered
    }

    fn on_input(&mut self, key :&KeyEvent, actions :&mut ActionProcessor) -> EventResult {
        match key.code {
            KeyCode::Up => {
                let new_id = self.state.previous();
                actions.add(Actions::FocusLog(Some(new_id)));
                EventResult::Handled
            },
            KeyCode::Down => {
                let new_id = self.state.next();
                actions.add(Actions::FocusLog(Some(new_id)));
                EventResult::Handled
            },
            KeyCode::Left => {
                actions.add(Actions::FocusLog(None));
                EventResult::Handled
            },

            KeyCode::Enter => {
                if let Some(log_id) = self.state.selected() {
                    actions.add(Actions::EditLog(log_id));
                }
                EventResult::Handled
            },

            KeyCode::Delete => {
                let to_del = self.state.selected();
                if to_del.is_none() {
                    return EventResult::NotHandled;
                }

                let log_info = Data::get_log(to_del.unwrap()).unwrap();
                actions.add(Actions::ShowConfirm(
                    format!("Are you sure you want to delete log '{}'?", log_info.name.unwrap()),
                    AlertDialogStyle::Warning,
                    Box::new(Actions::DeleteLog(to_del.unwrap()))
                ));
                EventResult::Handled
            },

            KeyCode::Char('a') => {
                actions.add(Actions::CreateLogWanted);
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }

    fn on_action(&mut self, action :&Actions, actions :&mut ActionProcessor) -> EventResult {
        match action {
            Actions::FocusLog(log_id) => {
                match log_id {
                    Some(id) => self.state.select(*id),
                    None => self.state.deselect(),
                }
                EventResult::NotHandled
            },
            Actions::DeleteLog(log_id) => {
                let res = Data::delete_log(*log_id);
                if res.is_err() {
                    actions.add(
                        Actions::ShowError(
                            format!("Error deleting log: {}", res.err().unwrap()
                        )
                    ));
                    return EventResult::Handled;
                }
                EventResult::Handled
            },

            _ => EventResult::NotHandled
        }
    }
}