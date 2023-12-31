use ratatui::{Frame, widgets::{Block, Borders, Clear, Paragraph}, layout::Rect, text::Span};
use crate::{traits::{UIElement, RenderResult, EventResult}, data::LogEntry, actions::Actions, common_types::RenderFrame, app_context::AppContext};

use super::unique_ids::define_typed_element;



#[derive(Default)]
struct DetailsWindowState {
    selected_log :Option<LogEntry>,
}

#[derive(Default)]
pub struct DetailsWindow {
    state :DetailsWindowState
}
define_typed_element!(DetailsWindow);

impl DetailsWindow {
    pub fn set_log(&mut self, log :LogEntry) {
        self.state.selected_log = Some(log);
    }

    fn render_info<B: ratatui::backend::Backend>(&self, f :&mut Frame<B>, label :&str, text :&String, rect :&mut Rect) -> () {

        const LABELS_WIDTH :u16 = 13;
        f.render_widget(Paragraph::new(label), *rect);

        let mut val_rect = rect.clone();
        val_rect.x += LABELS_WIDTH;
        val_rect.width -= LABELS_WIDTH;
        f.render_widget(Paragraph::new(Span::raw(text)), val_rect);


        rect.y += 1;
    }
}


impl UIElement for DetailsWindow {
    implement_typed_element!();

    fn render(&mut self, f :&mut RenderFrame, rect :Rect, app_ctx :&mut AppContext) -> RenderResult {
        if self.state.selected_log.is_none() {
            return Ok(());
        }

        f.render_widget(Clear, rect);
        f.render_widget(
            Block::default()
                .title("Details")
                .borders(Borders::ALL),
            rect
        );

        let mut rect = Rect {
            x: rect.x + 3,
            y: rect.y + 2,
            width: rect.width - 5,
            height: 1,
        };

        let log = self.state.selected_log.as_ref().unwrap();
        self.render_info(f, "Name:", &log.call, &mut rect);
        self.render_info(f, "Locator:", &log.locator, &mut rect);

        match log.position() {
            Some(pos) => {
                let self_pos = &app_ctx.data.config.own_position;
                self.render_info(f, "QTH: ", &pos.to_qth(), &mut rect);
                self.render_info(f, "Distance: ", &format!("{:.2} km", self_pos.distance_to(&pos).km()), &mut rect);
            },
            None => {
                self.render_info(f, "QTH: ", &"Unknown".to_string(), &mut rect);
                self.render_info(f, "Distance: ", &"Unknown".to_string(), &mut rect);
            },
        }

        Ok(())
    }

    fn on_action(&mut self, action :&Actions, app_ctx :&mut AppContext) -> crate::traits::EventResult {
        match action {
            Actions::FocusLog(log_id) => {
                match log_id {
                    Some(log_id) => self.set_log(app_ctx.data.logs.get(*log_id).unwrap().clone()),
                    None => self.set_log(Default::default())
                }
                EventResult::NotHandled
            }
            _ => EventResult::NOOP
        }
    }
}