use tui::{Frame, widgets::{Block, Borders, Clear, Paragraph}, layout::Rect, text::{Span}};

use crate::{traits::{UIElement, RenderResult}, data::{LogEntry, Data}, actions::ActionProcessor, common_types::RenderFrame};



#[derive(Default)]
struct DetailsWindowState {
    selected_log :Option<LogEntry>,
}

#[derive(Default)]
pub struct DetailsWindow {
    state :DetailsWindowState,
}

impl DetailsWindow {
    pub fn set_log(&mut self, log :LogEntry) {
        self.state.selected_log = Some(log);
    }

    fn render_info<B: tui::backend::Backend>(&self, f :&mut Frame<B>, label :&str, text :&String, rect :&mut Rect) -> () {

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

    fn render(&mut self, f :&mut RenderFrame, _actions :&mut ActionProcessor) -> RenderResult {
        if self.state.selected_log.is_none() {
            return RenderResult::NOOP;
        }

        let area = f.size();
        let rect = Rect {
            x: area.width - 70,
            y: area.height - 10,
            width: 70,
            height: 10,
        };

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
        self.render_info(f, "Name:", log.name.as_ref().unwrap(), &mut rect);
        self.render_info(f, "Latitude:", &format!("{}", log.lat.unwrap()), &mut rect);
        self.render_info(f, "Longitude:", &format!("{}", log.long.unwrap()), &mut rect);

        match log.position() {
            Some(pos) => {
                let self_pos = Data::get_config().own_position;
                self.render_info(f, "QTH: ", &pos.to_qth(), &mut rect);
                self.render_info(f, "Distance: ", &format!("{:.2} km", self_pos.distance_to(&pos).km()), &mut rect);
            },
            None => {
                self.render_info(f, "QTH: ", &"Unknown".to_string(), &mut rect);
                self.render_info(f, "Distance: ", &"Unknown".to_string(), &mut rect);
            },
        }

        RenderResult::Rendered
    }
}