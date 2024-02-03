use std::io::stdout;

use color_eyre::{config::HookBuilder, Result};
use crossterm::{event::*, terminal::*, ExecutableCommand};
use ratatui::prelude::*;

mod app;
mod tabs {
    mod buttons;
    mod stack;
    pub use buttons::ButtonsTab;
    pub use stack::StackTab;
}
use app::App;

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;
    App::new().run(terminal)?;
    let _ = restore_terminal();
    Ok(())
}

/// install panic and error hooks that restore the terminal before displaying the error messages
fn init_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}
