use crate::events::{EventHandler, Key, KeyPressedEvent};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Text,
    widgets::Widget,
};

/// A toggle switch widget
///
/// Displays a switch that can be toggled on or off
///
/// # Examples
///
/// ```rust
/// use ratatui::widgets::Widget;
/// use ratatui_widgets::toggle_switch::{ToggleSwitch, State};
///
/// # fn draw(frame: &mut ratatui::Frame) {
/// let toggle_switch = ToggleSwitch::new("Toggle me", State::Off);
/// frame.render_widget(toggle_switch, frame.area());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ToggleSwitch<'text> {
    text: Text<'text>,
    theme: Theme,
    state: State,
    focus: Focus,
}

#[derive(Default, PartialEq, Eq, Clone, Debug, Copy)]
pub enum State {
    On,
    #[default]
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Focused,
    Unfocused,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    focused_text: Color,

    focused_on_fg: Color,
    focused_on_bg_main: Color,
    focused_on_bg_highlight: Color,
    focused_on_bg_shadow: Color,

    focused_off_fg: Color,
    focused_off_bg_main: Color,
    focused_off_bg_highlight: Color,
    focused_off_bg_shadow: Color,

    unfocused_text: Color,

    unfocused_on_fg: Color,
    unfocused_on_bg_main: Color,
    unfocused_on_bg_highlight: Color,
    unfocused_on_bg_shadow: Color,

    unfocused_off_fg: Color,
    unfocused_off_bg_main: Color,
    unfocused_off_bg_highlight: Color,
    unfocused_off_bg_shadow: Color,
}

impl Default for Theme {
    fn default() -> Self {
        themes::NORMAL
    }
}

impl<'text> ToggleSwitch<'text> {
    pub fn new<T: Into<Text<'text>>>(text: T, default_state: State) -> Self {
        Self {
            text: text.into(),
            theme: Theme::default(),
            state: default_state,
            focus: Focus::Unfocused,
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
            Key::Char(' ') | Key::Enter => self.toggle_state(),
            Key::Char('h') | Key::Left => self.toggle_off(),
            Key::Char('l') | Key::Right => self.toggle_on(),
            _ => {}
        }
    }
}

impl ToggleSwitch<'_> {
    pub fn toggle_state(&mut self) {
        self.focus();
        match self.state {
            State::On => self.toggle_off(),
            State::Off => self.toggle_on(),
        }
    }

    pub fn toggle_on(&mut self) {
        self.state = State::On;
    }

    pub fn toggle_off(&mut self) {
        self.state = State::Off;
    }

    pub fn focus(&mut self) {
        self.focus = Focus::Focused;
    }

    pub fn blur(&mut self) {
        self.focus = Focus::Unfocused;
    }
}

impl Widget for &ToggleSwitch<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;

        // TODO: refactor this to use a more generic approach
        let (tick_fg, tick_bg, cross_fg, cross_bg) = match (self.focus, self.state) {
            (Focus::Focused, State::On) => (
                theme.focused_on_fg,
                theme.focused_on_bg_main,
                theme.focused_off_fg,
                theme.focused_off_bg_main,
            ),
            (Focus::Focused, State::Off) => (
                theme.focused_off_fg,
                theme.focused_off_bg_main,
                theme.focused_on_fg,
                theme.focused_on_bg_main,
            ),
            (Focus::Unfocused, State::On) => (
                theme.unfocused_on_fg,
                theme.unfocused_on_bg_main,
                theme.unfocused_off_fg,
                theme.unfocused_off_bg_main,
            ),
            (Focus::Unfocused, State::Off) => (
                theme.unfocused_off_fg,
                theme.unfocused_off_bg_main,
                theme.unfocused_on_fg,
                theme.unfocused_on_bg_main,
            ),
        };

        let (tick_highlight, tick_shadow, cross_highlight, cross_shadow) =
            match (self.state, self.focus) {
                (State::On, Focus::Focused) => (
                    theme.focused_on_bg_highlight,
                    theme.focused_on_bg_shadow,
                    theme.focused_off_bg_highlight,
                    theme.focused_off_bg_shadow,
                ),
                (State::On, Focus::Unfocused) => (
                    theme.unfocused_on_bg_highlight,
                    theme.unfocused_on_bg_shadow,
                    theme.unfocused_off_bg_highlight,
                    theme.unfocused_off_bg_shadow,
                ),
                (State::Off, Focus::Focused) => (
                    theme.focused_off_bg_highlight,
                    theme.focused_off_bg_shadow,
                    theme.focused_on_bg_highlight,
                    theme.focused_on_bg_shadow,
                ),
                (State::Off, Focus::Unfocused) => (
                    theme.unfocused_off_bg_highlight,
                    theme.unfocused_off_bg_shadow,
                    theme.unfocused_on_bg_highlight,
                    theme.unfocused_on_bg_shadow,
                ),
            };

        let [switch, label] = Layout::horizontal([Constraint::Max(10), Constraint::Fill(1)])
            .spacing(2)
            .areas(area);
        let [cross, tick] = Layout::horizontal([Constraint::Fill(1); 2]).areas(switch);

        buf.set_style(cross, (cross_fg, cross_bg));
        buf.set_style(tick, (tick_fg, tick_bg));

        let rows = switch.rows().collect_vec();
        let last_index = rows.len().saturating_sub(1);
        let (first, middle, last) = match rows.len() {
            0 | 1 => (None, &rows[..], None),
            2 => (None, &rows[..last_index], Some(rows[last_index])),
            _ => (Some(rows[0]), &rows[1..last_index], Some(rows[last_index])),
        };

        // render top line if there's enough space
        if let Some(first) = first {
            let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(first);
            "▔"
                .repeat(cross.width as usize)
                .fg(cross_highlight)
                .bg(cross_bg)
                .render(left, buf);
            "▔"
                .repeat(tick.width as usize)
                .fg(tick_highlight)
                .bg(tick_bg)
                .render(right, buf);
        }
        // render bottom line if there's enough space
        if let Some(last) = last {
            let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(last);
            "▁"
                .repeat(cross.width as usize)
                .fg(cross_shadow)
                .bg(cross_bg)
                .render(left, buf);
            "▁"
                .repeat(tick.width as usize)
                .fg(tick_shadow)
                .bg(tick_bg)
                .render(right, buf);
        }
        let text_style = match self.focus {
            Focus::Focused => theme.focused_text,
            Focus::Unfocused => theme.unfocused_text,
        };
        buf.set_style(label, text_style);
        let middle_row_index = label.height as usize / 2;
        let middle_row = label.rows().collect_vec()[middle_row_index];
        self.text.clone().left_aligned().render(middle_row, buf);

        let middle_row_index = middle.len() / 2;
        let middle_row = middle[middle_row_index];
        let [cross, tick] = Layout::horizontal([Constraint::Fill(1); 2]).areas(middle_row);
        Text::from("✗").centered().render(cross, buf);
        Text::from("✓").centered().render(tick, buf);
    }
}

pub mod themes {
    use super::Theme;
    use ratatui::style::palette::tailwind;

    pub const NORMAL: Theme = Theme {
        focused_text: tailwind::SLATE.c300,

        focused_on_fg: tailwind::BLUE.c100,
        focused_on_bg_main: tailwind::BLUE.c500,
        focused_on_bg_highlight: tailwind::BLUE.c300,
        focused_on_bg_shadow: tailwind::BLUE.c700,

        focused_off_fg: tailwind::SLATE.c300,
        focused_off_bg_main: tailwind::SLATE.c700,
        focused_off_bg_highlight: tailwind::SLATE.c500,
        focused_off_bg_shadow: tailwind::SLATE.c900,

        unfocused_text: tailwind::SLATE.c400,

        unfocused_on_fg: tailwind::BLUE.c200,
        unfocused_on_bg_main: tailwind::BLUE.c600,
        unfocused_on_bg_highlight: tailwind::BLUE.c400,
        unfocused_on_bg_shadow: tailwind::BLUE.c800,

        unfocused_off_fg: tailwind::SLATE.c400,
        unfocused_off_bg_main: tailwind::SLATE.c800,
        unfocused_off_bg_highlight: tailwind::SLATE.c600,
        unfocused_off_bg_shadow: tailwind::SLATE.c950,
    };
}
