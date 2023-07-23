mod app;
mod data;
mod ui;
mod map_api;
mod traits;
mod actions;

use actions::ActionProcessor;
use app::App;



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

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    // setup app
    let mut action_processor = ActionProcessor::default();

    let mut app = App::new();
    let result = app.run(&mut terminal, &mut action_processor);

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));

    // restore terminal
    reset_terminal()?;

    if result.is_err() {
        println!("Error: {:?}", result);
        return Err(result.unwrap_err());
    }
    Ok(())
}