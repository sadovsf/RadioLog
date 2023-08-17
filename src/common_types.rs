use std::io::Stdout;
use ratatui::{backend::CrosstermBackend, Frame};




pub type RenderFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;