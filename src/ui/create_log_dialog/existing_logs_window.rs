use std::collections::HashMap;

use ratatui::{prelude::{Rect, Constraint}, widgets::{TableState, Table, Block, Borders, Row, Clear}, style::{Style, Color}};

use crate::{ui::define_typed_element, traits::{UIElement, RenderResult}, common_types::RenderFrame, app_context::AppContext, data::position::Position};





pub struct ExistingLogsWindow<'a> {
    rows :Vec<Row<'a>>,
    state :TableState
}
define_typed_element!(ExistingLogsWindow<'_>);

impl ExistingLogsWindow<'_> {
    pub fn from_search(app_ctx :&mut AppContext, term :&str) -> Result<Self, rusqlite::Error> {
        if term.len() < 2 {
            return Ok(Self {
                rows: vec!(),
                state: TableState::default(),
            });
        }

        let logs = app_ctx.data.logs.get_where(
            "like(?1, call) > 0", [format!("%{}%", term)]
        )?;

        let mut map :HashMap<&str, u32> = HashMap::new();
        for log in &logs {
            let count = map.get(log.locator.as_str()).unwrap_or(&0);
            map.insert(log.locator.as_str(), count + 1);
        }

        let my_pos = app_ctx.data.my_position();
        let rows :Vec<Row> = map.iter().map(|(k, v)|
            Row::new([
                v.to_string(),
                k.to_string(),
                format!("{:.1}", Position::from_qth(k).map_or(0.0, |v| v.azimuth_to(&my_pos)))
            ])
        ).collect();

        Ok(Self {
            rows,
            state: TableState::default(),
        })
    }
}

impl UIElement for ExistingLogsWindow<'_> {
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        if self.rows.len() == 0 {
            return Ok(());
        }

        //TODO add what call is bound to each row so we can glance over even partialy filtered known call signs and their usual positions.

        f.render_widget(Clear, rect); //this clears out the background
        f.render_stateful_widget(
            Table::new(self.rows.clone())
                .block(Block::default().title("Existing logs").borders(Borders::ALL))
                .header(
                    Row::new(["Count", "QTH", "Azim"])
                        .style(Style::default().bg(Color::Cyan))
                )
                .widths(&[
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40)
                ]),
            rect,
            &mut self.state
        );

        Ok(())
    }
}