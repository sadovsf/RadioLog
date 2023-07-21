use std::time::{Instant, Duration};

use crossterm::{event::{Event, self, KeyCode, KeyEventKind}, Result};
use tui::{Terminal, backend::Backend, Frame, layout::{Direction, Layout, Constraint} };

use crate::{ui::{self, UIState, CreateLogDialog}, data::{Data}};



pub struct App {
    state: UIState,
    create_dialog :CreateLogDialog,
}

impl App {
    pub fn new() -> App {
        App {
            state: UIState::default(),
            create_dialog: CreateLogDialog::default(),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal :&mut Terminal<B>) -> Result<()> {
        const TICK_RATE :Duration = std::time::Duration::from_millis(250);

        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| self.draw(f) )?;

            let timeout = TICK_RATE
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if self.create_dialog.is_opened() {
                        self.create_dialog.on_input(key);
                    } else {
                        match key.code {
                            KeyCode::Esc => return Ok(()),
                            _ => self.handle_input(key)
                        }
                    }
                }
            }

            if last_tick.elapsed() >= TICK_RATE {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }



    fn on_tick(&mut self) {

    }


    fn draw<B: Backend>(&mut self, f :&mut Frame<B>) {
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

        let log_block = ui::LogList::default();
        f.render_stateful_widget(log_block, rects[0], &mut self.state.log_list_state);

        ui::WorldMap::render(f, rects[1], &self.state.world_map_state);

        if self.create_dialog.is_opened() {
            self.create_dialog.render(f);
        } else {
            match self.create_dialog.get_last_created_log() {
                Some(log) => {
                    self.state.log_list_state.select(&log);
                    self.state.world_map_state.selected_position = log.position();
                    self.create_dialog.clear_last_created_log();
                },
                None => {}
            }
        }
    }


    fn zoom_map(&mut self, zoom :f64) {
        let old_center = ui::WorldMap::map_center(&self.state.world_map_state);
        self.state.world_map_state.zoom += zoom;
        self.state.world_map_state.zoom = self.state.world_map_state.zoom.clamp(0.1, 10.0);
        let new_center = ui::WorldMap::map_center(&self.state.world_map_state);

        self.state.world_map_state.top_left.longitude += old_center.longitude - new_center.longitude;
        self.state.world_map_state.top_left.latitude += old_center.latitude - new_center.latitude;
    }

    fn pop_error(&self, _text :String) {
        // TBD
    }

    fn handle_input(&mut self, key :event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Down => {
                self.state.log_list_state.next();
                self.state.world_map_state.selected_position = self.state.log_list_state.selected_location();
            },
            KeyCode::Up => {
                self.state.log_list_state.previous();
                self.state.world_map_state.selected_position = self.state.log_list_state.selected_location();
            },

            KeyCode::Left => {
                self.state.log_list_state.deselect();
                self.state.world_map_state.selected_position = None;
            },

            KeyCode::Enter => {
                let id = self.state.log_list_state.selected().unwrap();
                let log = Data::get_log(id);
                if log.is_some() {
                    self.create_dialog.edit(log.unwrap());
                }
            },

            // Map controls:
            KeyCode::Char('+') => self.zoom_map(-0.05),
            KeyCode::Char('-') => self.zoom_map(0.05),

            KeyCode::Char('8') => self.state.world_map_state.top_left.latitude += 5.0,
            KeyCode::Char('5') => self.state.world_map_state.top_left.latitude -= 5.0,
            KeyCode::Char('4') => self.state.world_map_state.top_left.longitude -= 5.0,
            KeyCode::Char('6') => self.state.world_map_state.top_left.longitude += 5.0,

            KeyCode::Delete => {
                let to_del = self.state.log_list_state.selected();
                if to_del.is_none() {
                    return;
                }
                let result = Data::delete_log(to_del.unwrap());
                if result.is_err() {
                    self.pop_error(format!("Error deleting log: {}", result.err().unwrap()));
                    return;
                }
                self.state.world_map_state.selected_position = None;
                self.state.log_list_state.deselect();
            },

            KeyCode::Char('a') => self.create_dialog.open(),
            _ => {}
        };
    }


}