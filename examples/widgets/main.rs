use std::io::stdout;

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    ExecutableCommand,
};
use ratatui::DefaultTerminal;

mod app;
mod tabs {
    mod buttons;
    mod stack;
    pub use buttons::ButtonsTab;
    pub use stack::StackTab;
}
use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(terminal: DefaultTerminal) -> Result<()> {
    stdout().execute(EnableMouseCapture)?;
    App::new().run(terminal)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}
