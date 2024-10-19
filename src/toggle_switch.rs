use crate::events::{EventHandler, Key, KeyPressedEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::prelude::{Color, Constraint, Position, Style, Stylize, Text, Widget};

#[derive(Debug, Clone)]
pub struct ToggleSwitch<'text> {
    text: Text<'text>,
    theme: Theme,
    state: State,
    is_selected: bool,
}

#[derive(Default, PartialEq, Eq, Clone, Debug, Copy)]
pub enum State {
    On,
    #[default]
    Off,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    idle_bg_on_main: Color,
    idle_bg_off_main: Color,
    idle_bg_on_highlight: Color,
    idle_bg_off_highlight: Color,
    idle_bg_on_shadow: Color,
    idle_bg_off_shadow: Color,

    selected_bg_on_main: Color,
    selected_bg_off_main: Color,
    selected_bg_on_highlight: Color,
    selected_bg_off_highlight: Color,
    selected_bg_on_shadow: Color,
    selected_bg_off_shadow: Color,

    idle_fg_on: Color,
    idle_fg_off: Color,
    selected_fg_on: Color,
    selected_fg_off: Color,

    idle_text: Color,
    selected_text: Color,
}

impl Default for Theme {
    fn default() -> Self {
        themes::NORMAL
    }
}

impl<'text> ToggleSwitch<'text> {
    pub fn new<T: Into<Text<'text>>>(is_on: bool, text: T) -> Self {
        Self {
            text: text.into(),
            theme: Theme::default(),
            state: if is_on { State::On } else { State::Off },
            is_selected: false,
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl EventHandler for ToggleSwitch<'_> {
    fn handle_key(&mut self, key_event: KeyPressedEvent) {
        match key_event.key {
            Key::Char(' ') | Key::Enter => self.toggle_press(),
            _ => {}
        }
    }
}

impl ToggleSwitch<'_> {
    pub fn toggle_press(&mut self) {
        match self.state {
            State::On => {
                self.toggle_off();
                self.select()
            }
            State::Off => {
                self.toggle_on();
                self.select()
            }
        }
    }

    pub fn toggle_on(&mut self) {
        self.state = State::On;
    }

    pub fn toggle_off(&mut self) {
        self.state = State::Off;
    }

    pub fn select(&mut self) {
        self.is_selected = true;
    }
    pub fn normal(&mut self) {
        self.is_selected = false;
    }
}

impl Widget for &ToggleSwitch<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;

        // these are wrong
        let tick_fg = match (self.state, self.is_selected) {
            (State::On, true) => theme.selected_fg_on,
            (State::On, false) => theme.idle_fg_on,
            (State::Off, true) => theme.selected_fg_off,
            (State::Off, false) => theme.idle_fg_off,
        };
        let tick_bg = match (self.state, self.is_selected) {
            (State::On, true) => theme.selected_bg_on_main,
            (State::On, false) => theme.idle_bg_on_main,
            (State::Off, true) => theme.selected_bg_off_main,
            (State::Off, false) => theme.idle_bg_off_main,
        };

        let cross_fg = match (self.state, self.is_selected) {
            (State::On, true) => theme.selected_fg_off,
            (State::On, false) => theme.idle_fg_off,
            (State::Off, true) => theme.selected_fg_on,
            (State::Off, false) => theme.idle_fg_on,
        };
        let cross_bg = match (self.state, self.is_selected) {
            (State::On, true) => theme.selected_bg_off_main,
            (State::On, false) => theme.idle_bg_off_main,
            (State::Off, true) => theme.selected_bg_on_main,
            (State::Off, false) => theme.idle_bg_on_main,
        };

        let (tick_highlight, tick_shadow, cross_highlight, cross_shadow) =
            match (self.state, self.is_selected) {
                (State::On, true) => (
                    theme.selected_bg_on_highlight,
                    theme.selected_bg_on_shadow,
                    theme.selected_bg_off_highlight,
                    theme.selected_bg_off_shadow,
                ),
                (State::On, false) => (
                    theme.idle_bg_on_highlight,
                    theme.idle_bg_on_shadow,
                    theme.idle_bg_off_highlight,
                    theme.idle_bg_off_shadow,
                ),
                (State::Off, true) => (
                    theme.selected_bg_off_highlight,
                    theme.selected_bg_off_shadow,
                    theme.selected_bg_on_highlight,
                    theme.selected_bg_on_shadow,
                ),
                (State::Off, false) => (
                    theme.idle_bg_off_highlight,
                    theme.idle_bg_off_shadow,
                    theme.idle_bg_on_highlight,
                    theme.idle_bg_on_shadow,
                ),
            };

        let areas = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Max(10),
                Constraint::Length(2),
                Constraint::Fill(1),
            ],
        )
        .split(area);

        let (switch, label) = (areas[0], areas[2]);

        let switch_areas = Layout::new(
            Direction::Horizontal,
            [Constraint::Fill(1), Constraint::Fill(1)],
        )
        .split(switch);

        let (cross, tick) = (switch_areas[0], switch_areas[1]);

        buf.set_style(cross, (cross_fg, cross_bg));
        buf.set_style(tick, (tick_fg, tick_bg));

        let rows = switch.rows().collect::<Vec<_>>();
        let last_index = rows.len().saturating_sub(1);
        let (first, middle, last) = match rows.len() {
            0 | 1 => (None, &rows[..], None),
            2 => (None, &rows[..last_index], Some(rows[last_index])),
            _ => (Some(rows[0]), &rows[1..last_index], Some(rows[last_index])),
        };

        // render top line if there's enough space
        if let Some(first) = first {
            let parts = Layout::new(
                Direction::Horizontal,
                [Constraint::Fill(1), Constraint::Fill(1)],
            )
            .split(first);
            "▔"
                .repeat(cross.width as usize)
                .fg(cross_highlight)
                .bg(cross_bg)
                .render(parts[0], buf);
            "▔"
                .repeat(tick.width as usize)
                .fg(tick_highlight)
                .bg(tick_bg)
                .render(parts[1], buf);
        }
        // render bottom line if there's enough space
        if let Some(last) = last {
            let parts = Layout::new(
                Direction::Horizontal,
                [Constraint::Fill(1), Constraint::Fill(1)],
            )
            .split(last);
            "▁"
                .repeat(cross.width as usize)
                .fg(cross_shadow)
                .bg(cross_bg)
                .render(parts[0], buf);
            "▁"
                .repeat(tick.width as usize)
                .fg(tick_shadow)
                .bg(tick_bg)
                .render(parts[1], buf);
        }
        buf.set_style(
            label,
            Style::default().fg(if self.is_selected {
                theme.selected_text
            } else {
                theme.idle_text
            }),
        );
        self.text.clone().left_aligned().render(
            label.rows().collect::<Vec<_>>()[label.height as usize / 2],
            buf,
        );
        let text_areas = Layout::new(
            Direction::Horizontal,
            [Constraint::Fill(1), Constraint::Fill(1)],
        )
        .split(middle[middle.len() / 2]);
        Text::from("✗").centered().render(text_areas[0], buf);
        Text::from("✓").centered().render(text_areas[1], buf);
    }
}

pub mod themes {
    use super::Theme;
    use ratatui::style::palette::tailwind;

    pub const NORMAL: Theme = Theme {
        idle_bg_on_main: tailwind::BLUE.c600,
        idle_bg_off_main: tailwind::SLATE.c800,
        idle_bg_on_highlight: tailwind::BLUE.c400,
        idle_bg_off_highlight: tailwind::SLATE.c600,
        idle_bg_on_shadow: tailwind::BLUE.c800,
        idle_bg_off_shadow: tailwind::SLATE.c950,
        selected_bg_on_main: tailwind::BLUE.c500,
        selected_bg_off_main: tailwind::SLATE.c700,
        selected_bg_on_highlight: tailwind::BLUE.c300,
        selected_bg_off_highlight: tailwind::SLATE.c500,
        selected_bg_on_shadow: tailwind::BLUE.c700,
        selected_bg_off_shadow: tailwind::SLATE.c900,
        idle_fg_on: tailwind::BLUE.c200,
        idle_fg_off: tailwind::SLATE.c400,
        selected_fg_on: tailwind::BLUE.c100,
        selected_fg_off: tailwind::SLATE.c300,
        idle_text: tailwind::SLATE.c400,
        selected_text: tailwind::SLATE.c300,
    };
}
