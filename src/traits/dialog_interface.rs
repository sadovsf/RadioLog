use crossterm::event::KeyEvent;
use tui::{layout::{Rect, Layout, Direction, Constraint}, backend::Backend, Frame};


pub struct DialogHelpers {}

impl DialogHelpers {

    pub fn center_rect_perc(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

        Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
    }

    pub fn center_rect_size(size_x: u16, size_y: u16, r: Rect) -> Rect {
        Rect {
            x: r.x + (r.width - size_x) / 2,
            y: r.y + (r.height - size_y) / 2,
            width: size_x,
            height: size_y,
        }
    }
}


pub trait DialogInterface {
    fn set_opened(&mut self, opened :bool);
    fn is_opened(&self) -> bool;

    fn render<B: Backend>(&mut self, f :&mut Frame<B>) -> ();
    fn on_input(&mut self, key :KeyEvent) -> ();

    fn open(&mut self) {
        self.set_opened(true);
    }
    fn close(&mut self) {
        self.set_opened(false);
    }
}