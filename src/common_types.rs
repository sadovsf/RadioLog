use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};




pub type RenderFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;