mod app;
mod data;
mod ui;
mod map_api;
mod traits;
mod actions;
mod common_types;
mod ui_handler;
mod app_context;
mod database;
mod app_errors;

use app::App;
use app_context::AppContext;
use app_errors::AppError;



use std::{io};
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result as CrosstermResult,
};



fn reset_terminal() -> CrosstermResult<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn main() -> Result<(), AppError> {
    let app_context = AppContext::new()?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = reset_terminal();
        original_hook(panic);
    }));

    // setup app
    let mut app = App::new();
    let result = app.run(&mut terminal, app_context);

    // restore terminal
    reset_terminal()?;
    Ok(result?)
}