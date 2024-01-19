use std::io::{self, stdout};

use color_eyre::{config::HookBuilder, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{Paragraph, Widget},
};
use ratatui_widgets::button::*;

#[derive(Debug, Default, Clone)]
struct App {
    state: RunningState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Quit,
}

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;
    App::new().run(terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        self.draw(&mut terminal)?;
        while self.is_running() {
            self.handle_events()?;
            self.draw(&mut terminal)?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == RunningState::Running
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = RunningState::Quit;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
        let [title, body] = area.split(&layout);
        self.render_title(title, buf);
        self.render_buttons(body, buf);
    }
}

impl App {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Button example. Esc: quit, ↓: down, ↑: up, Home: top, End: bottom")
            .style((tailwind::SLATE.c900, tailwind::SLATE.c300))
            .render(area, buf);
    }

    fn render_buttons(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([1, 3, 1]);
        let [_, buttons, _] = area.split(&layout);
        let layout = Layout::horizontal([20, 2, 20, 2, 20, 0]);
        let [left, _, middle, _, right, _] = buttons.split(&layout);
        Button::from("Button 1").with_theme(RED).render(left, buf);
        Button::from("Button 2")
            .with_theme(BLUE)
            .render(middle, buf);
        Button::from("Button 3")
            .with_theme(GREEN)
            .render(right, buf);
    }
}

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
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
