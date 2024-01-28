use std::io::{self};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::*, style::palette::tailwind, symbols::border::*, widgets::*};
use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};

use crate::buttons::ButtonsTab;

#[derive(Debug, Default, Clone)]
pub struct App {
    state: RunningState,
    selected_tab: ExampleTab,
    buttons_tab: ButtonsTab,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Quit,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Display, EnumIter, FromRepr, EnumCount)]
enum ExampleTab {
    #[default]
    Buttons,
    Widget2,
    Widget3,
}

impl App {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.is_running() {
            self.draw(&mut terminal)?;
            self.handle_events()?;
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
                Tab => self.selected_tab = self.selected_tab.next(),
                BackTab => self.selected_tab = self.selected_tab.prev(),
                Char('q') | Esc => self.quit(),
                _ => {
                    if self.selected_tab == ExampleTab::Buttons {
                        self.buttons_tab.handle_key_press(key);
                    }
                }
            },
            Event::Mouse(event) => {
                if self.selected_tab == ExampleTab::Buttons {
                    self.buttons_tab.handle_mouse_event(event);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = RunningState::Quit;
    }
}

impl ExampleTab {
    fn next(self) -> Self {
        let index = self as usize;
        let next = (index + 1) % Self::COUNT;
        Self::from_repr(next).unwrap()
    }

    fn prev(self) -> Self {
        let index = self as usize;
        let prev = (index + Self::COUNT - 1) % Self::COUNT;
        Self::from_repr(prev).unwrap()
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let layout = Layout::vertical([Length(1), Proportional(1), Length(1)]);
        let [header, body, footer] = area.split(&layout);
        let layout = Layout::horizontal([Proportional(1), Length(15)]);
        let [tabs, title] = header.split(&layout);

        self.footer().render(footer, buf);
        self.title().render(title, buf);
        self.tabs().render(tabs, buf);
        self.render_tab(body, buf);
    }
}

impl App {
    fn footer(&self) -> impl Widget {
        Line::raw("Esc: quit, Tab: next tab, Shift+Tab: prev tab")
            .style(tailwind::SLATE.c300)
            .centered()
    }

    fn title(&self) -> impl Widget {
        Line::raw("ratatui-widgets")
            .style(tailwind::SLATE.c300)
            .centered()
    }

    fn tabs(&self) -> impl Widget {
        Tabs::new(ExampleTab::titles())
            .select(self.selected_tab as usize)
            .divider(" ")
            .padding("", "")
            .highlight_style(Modifier::BOLD)
    }

    fn render_tab(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(PROPORTIONAL_TALL)
            .border_style(self.selected_tab.color())
            .padding(Padding::horizontal(1));
        let inner = block.inner(area);
        block.render(area, buf);

        match self.selected_tab {
            ExampleTab::Buttons => self.buttons_tab.render(inner, buf),
            ExampleTab::Widget2 => Line::raw("TODO").render(inner, buf),
            ExampleTab::Widget3 => Line::raw("TODO").render(inner, buf),
        }
    }
}

impl ExampleTab {
    fn titles() -> impl Iterator<Item = Line<'static>> {
        Self::iter().map(|tab| tab.title().into())
    }

    fn title(&self) -> Span<'static> {
        // use blue, emerald, indigo, red, yellow, ...
        let bg = match self {
            ExampleTab::Buttons => tailwind::BLUE.c700,
            ExampleTab::Widget2 => tailwind::EMERALD.c700,
            ExampleTab::Widget3 => tailwind::PURPLE.c700,
        };
        format!("  {self}  ").fg(tailwind::SLATE.c200).bg(bg)
    }

    fn color(&self) -> Color {
        match self {
            ExampleTab::Buttons => tailwind::BLUE.c700,
            ExampleTab::Widget2 => tailwind::EMERALD.c700,
            ExampleTab::Widget3 => tailwind::PURPLE.c700,
        }
    }
}
