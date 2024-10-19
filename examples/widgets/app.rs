use std::io::{self};

use color_eyre::Result;
use ratatui::{prelude::*, style::palette::tailwind, symbols::border::*, widgets::*};
use ratatui_widgets::events::*;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::tabs::*;

#[derive(Debug)]
pub struct App {
    state: RunningState,
    selected_tab_index: usize,
    tabs: Vec<Tab>,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Quit,
}

#[derive(Debug, Display, EnumIter)]
enum Tab {
    Buttons(ButtonsTab),
    Stack(StackTab),
    ToggleSwitch(ToggleSwitchTab),
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            state: RunningState::Running,
            selected_tab_index: 0,
            tabs: Tab::iter().collect(),
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
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match Event::try_from(crossterm::event::read()?) {
            Ok(event) => self.handle_event(event),
            Err(_) => {
                // ignore for now. Perhaps change the try_from approach to a method that returns
                // Option instead of Result
            }
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = RunningState::Quit;
    }
}

impl EventHandler for App {
    fn handle_key(&mut self, key_pressed_event: KeyPressedEvent) {
        use Key::*;
        match key_pressed_event.key {
            Tab => self.next_tab(),
            BackTab => self.prev_tab(),
            Char('q') | Esc => self.quit(),
            _ => {
                self.selected_tab_mut().handle_key(key_pressed_event);
            }
        }
    }

    fn handle_mouse(&mut self, event: MouseEvent) {
        self.selected_tab_mut().handle_mouse(event);
    }
}

impl App {
    fn selected_tab_mut(&mut self) -> &mut Tab {
        self.tabs.get_mut(self.selected_tab_index).unwrap()
    }

    pub fn next_tab(&mut self) {
        let tab_count = self.tabs.len();
        self.selected_tab_index = (self.selected_tab_index + 1) % tab_count;
    }

    pub fn prev_tab(&mut self) {
        let tab_count = self.tabs.len();
        self.selected_tab_index = (self.selected_tab_index + tab_count - 1) % tab_count;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let layout = Layout::vertical([Length(1), Fill(1), Length(1)]);
        let [header, body, footer] = layout.areas(area);
        let layout = Layout::horizontal([Fill(1), Length(15)]);
        let [tabs, title] = layout.areas(header);

        self.footer().render(footer, buf);
        self.title().render(title, buf);
        self.tabs().render(tabs, buf);
        self.selected_tab_mut().render(body, buf);
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
        Tabs::new(self.tabs.iter().map(Tab::title))
            .select(self.selected_tab_index)
            .divider(" ")
            .padding("", "")
            .highlight_style(Modifier::BOLD)
    }
}

impl Widget for &mut Tab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(PROPORTIONAL_TALL)
            .border_style(self.color())
            .padding(Padding::horizontal(1));
        let inner = block.inner(area);
        block.render(area, buf);

        match self {
            Tab::Buttons(buttons) => buttons.render(inner, buf),
            Tab::Stack(stack) => stack.render(inner, buf),
            Tab::ToggleSwitch(switches) => switches.render(inner, buf),
        }
    }
}

impl EventHandler for Tab {
    fn handle_key(&mut self, event: KeyPressedEvent) {
        match self {
            Tab::Buttons(buttons) => buttons.handle_key(event),
            Tab::Stack(stack) => stack.handle_key(event),
            Tab::ToggleSwitch(switches) => switches.handle_key(event),
        }
    }

    fn handle_mouse(&mut self, event: MouseEvent) {
        match self {
            Tab::Buttons(buttons) => buttons.handle_mouse(event),
            Tab::Stack(_) => {}
            Tab::ToggleSwitch(switches) => switches.handle_mouse(event),
        }
    }
}

impl Tab {
    fn title(&self) -> Span<'static> {
        // use blue, emerald, indigo, red, yellow, ...
        let bg = match self {
            Tab::Buttons(_) => tailwind::BLUE.c700,
            Tab::Stack(_) => tailwind::EMERALD.c700,
            Tab::ToggleSwitch(_) => tailwind::PURPLE.c700,
        };
        format!("  {self}  ").fg(tailwind::SLATE.c200).bg(bg)
    }

    fn color(&self) -> Color {
        match self {
            Tab::Buttons(_) => tailwind::BLUE.c700,
            Tab::Stack(_) => tailwind::EMERALD.c700,
            Tab::ToggleSwitch(_) => tailwind::PURPLE.c700,
        }
    }
}
