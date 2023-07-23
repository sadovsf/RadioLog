use tui::{widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget}, layout::Rect, buffer::Buffer, text::Span, style::{Style, Color, Modifier} };

use crate::{data::{Data, position::Position, LogEntry}};



pub struct LogListState {
    list_state :ListState,
}

impl Default for LogListState {
    fn default() -> Self {
        LogListState {
            list_state: ListState::default(),
        }
    }
}

impl LogListState {
    pub fn next(&self) -> i64 {
        let all_logs = Data::get_logs();
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= all_logs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            },
            None => 0,
        };
        all_logs[i].rowid.unwrap()
    }

    pub fn previous(&self) -> i64 {
        let all_logs = Data::get_logs();
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    all_logs.len() - 1
                } else {
                    i - 1
                }
            },
            None => 0,
        };
        all_logs[i].rowid.unwrap()
    }

    pub fn selected_location(&self) -> Option<Position> {
        match self.list_state.selected() {
            Some(i) => Data::get_logs()[i].position(),
            None => None,
        }
    }

    pub fn selected(&self) -> Option<i64> {
        match self.list_state.selected() {
            Some(i) => Data::get_logs()[i].rowid,
            None => None,
        }
    }

    pub fn deselect(&mut self) {
        self.list_state.select(None);
    }

    pub fn select(&mut self, log_id :i64) {
        let pos = Data::get_logs().iter().position(|l| l.rowid == Some(log_id));
        self.list_state.select(pos);
    }
}



pub struct LogList<'a> {
    list :List<'a>,
}

impl<'a> Default for LogList<'a> {
    fn default() -> Self {
        let list_items :Vec<ListItem> = Data::get_logs()
            .iter()
            .map(|log| {
                let span = Span::raw(log.name.as_ref().unwrap().clone());
                ListItem::new(span)
            })
            .collect();

        LogList {
            list: List::new(list_items)
                    .block(Block::default().title("Logs").borders(Borders::ALL))
                    .highlight_style(
                        Style::default()
                            .bg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD)
                    )
                    .highlight_symbol(">> ")

        }
    }
}

impl<'a> StatefulWidget for LogList<'a> {
    type State = LogListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // match state.list_state.selected() {
        //     Some(val) => println!("{}", val),
        //     None => println!("None"),
        // }
        self.list.render(area, buf, &mut state.list_state);
    }
}