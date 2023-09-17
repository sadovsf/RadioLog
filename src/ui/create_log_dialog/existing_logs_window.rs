use ratatui::{prelude::{Rect, Constraint}, widgets::{TableState, Table, Block, Borders, Row, Clear}, style::{Style, Color}};
use crate::{ui::define_typed_element, traits::{UIElement, RenderResult}, common_types::RenderFrame, app_context::AppContext, data::position::Position};





pub struct ExistingLogsWindow<'a> {
    rows :Vec<Row<'a>>,
    state :TableState
}
define_typed_element!(ExistingLogsWindow<'_>);

impl ExistingLogsWindow<'_> {
    pub fn from_call_search(app_ctx :&mut AppContext, call_term :&str, locator_term :&str) -> Result<Self, rusqlite::Error> {
        if call_term.len() < 2 && locator_term.len() < 2 {
            return Ok(Self {
                rows: vec!(),
                state: TableState::default(),
            });
        }

        let cur_race_id = match app_ctx.data.current_race_id {
        Some(id) => id,
            None => 0
        };

        let db = app_ctx.db.borrow();
        let conn = db.get_connection();
        let mut stmt;
        let mut rows;
        if call_term.len() > 0 && locator_term.len() > 0 {
            stmt = conn.prepare_cached("SELECT call, locator, COUNT(locator) FROM LogEntry WHERE like(?1, call) > 0 OR like(?2, locator) > 0 GROUP BY lower(call), lower(locator) ORDER BY call, locator")?;
            rows = stmt.query((format!("%{}%", call_term), format!("%{}%", locator_term)))?;
        } else if call_term.len() > 0 {
            stmt = conn.prepare_cached("SELECT call, locator, COUNT(locator) FROM LogEntry WHERE like(?1, call) > 0 GROUP BY lower(call), lower(locator) ORDER BY call, locator")?;
            rows = stmt.query([format!("%{}%", call_term)])?;
        } else {
            stmt = conn.prepare_cached("SELECT call, locator, COUNT(locator) FROM LogEntry WHERE like(?1, locator) > 0 GROUP BY lower(call), lower(locator) ORDER BY call, locator")?;
            rows = stmt.query([format!("%{}%", locator_term)])?;
        }


        let my_pos = app_ctx.data.my_position();
        let mut final_rows = vec![];
        while let Some(row) = rows.next()? {
            let call :String = row.get(0)?;
            let locator :String = row.get(1)?;
            let count :i64 = row.get(2)?;

            let pos = Position::from_qth(&locator).unwrap_or(my_pos);
            let azimuth = pos.azimuth_to(&my_pos);

            // TODO optimize original query to ned require this. Possibly using JOIN ?
            let mut stmt = conn.prepare_cached("SELECT id FROM LogEntry where race_id=?1 AND call=?2")?;
            let mut dupl_results = stmt.query((cur_race_id, &call))?;
            let has_duplicites = match dupl_results.next()? {
                Some(_) => true,
                None => false
            };


            let table_row = Row::new([
                call,
                locator,
                format!("{}", count),
                format!("{:.1}", azimuth),
                (if has_duplicites { "DUP" } else { "" }).to_string()
            ]);
            final_rows.push(table_row);
        }

        Ok(Self {
            rows: final_rows,
            state: TableState::default(),
        })
    }
}

impl UIElement for ExistingLogsWindow<'_> {
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, _app_ctx :&mut AppContext) -> RenderResult {
        if self.rows.len() == 0 {
            return Ok(());
        }

        //TODO add what call is bound to each row so we can glance over even partialy filtered known call signs and their usual positions.

        f.render_widget(Clear, rect); //this clears out the background
        f.render_stateful_widget(
            Table::new(self.rows.clone())
                .block(Block::default().title("Existing logs").borders(Borders::ALL))
                .header(
                    Row::new(["Call", "QTH", "#", "Azim", ""])
                        .style(Style::default().bg(Color::Cyan))
                )
                .widths(&[
                    Constraint::Percentage(30),
                    Constraint::Min(7),
                    Constraint::Min(3),
                    Constraint::Min(6),
                    Constraint::Min(6)
                ]),
            rect,
            &mut self.state
        );

        Ok(())
    }
}