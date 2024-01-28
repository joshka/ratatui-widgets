#![allow(unused)]

use ratatui::{prelude::*, widgets::Widget};

#[derive(Debug, Clone)]
pub struct Button<'text> {
    text: Text<'text>,
    theme: Theme,
    state: State,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum State {
    #[default]
    Normal,
    Selected,
    Pressed,
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
    selected: Color,
    pressed: Color,
}

impl Default for Theme {
    fn default() -> Self {
        themes::NORMAL
    }
}

/// Config
impl<'text> Button<'text> {
    pub fn new<T: Into<Text<'text>>>(text: T) -> Self {
        Self {
            text: text.into(),
            theme: Theme::default(),
            state: State::default(),
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn press(&mut self) {
        if self.state == State::Selected {
            self.state = State::Pressed;
        } else {
            self.state = State::Selected;
        }
    }

    pub fn normal(&mut self) {
        self.state = State::Normal;
    }

    pub fn select(&mut self) {
        self.state = State::Selected;
    }
}

impl Widget for &Button<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Theme {
            text,
            background,
            highlight,
            shadow,
            selected,
            pressed,
        } = self.theme;

        // these are wrong
        let text = match self.state {
            State::Normal => text,
            State::Selected => highlight,
            State::Pressed => background,
        };
        let background = match self.state {
            State::Normal => background,
            State::Selected => selected,
            State::Pressed => pressed,
        };

        buf.set_style(area, Style::new().bg(background).fg(text));
        // render top line if there's enough space

        let rows = area.rows().collect::<Vec<_>>();
        let last_index = rows.len().saturating_sub(1);
        let (first, middle, last) = match rows.len() {
            0 | 1 => (None, &rows[..], None),
            2 => (None, &rows[..last_index], Some(rows[last_index])),
            _ => (Some(rows[0]), &rows[1..last_index], Some(rows[last_index])),
        };
        if let Some(first) = first {
            "▔"
                .repeat(area.width as usize)
                .fg(highlight)
                .bg(background)
                .render(first, buf);
        }
        if let Some(last) = last {
            "▁"
                .repeat(area.width as usize)
                .fg(shadow)
                .bg(background)
                .render(last, buf);
        }
        self.text.clone().centered().render(middle[0], buf);
    }
}

pub mod themes {
    use super::Theme;
    use ratatui::style::palette::tailwind;

    pub const NORMAL: Theme = Theme {
        text: tailwind::GRAY.c900,
        background: tailwind::GRAY.c500,
        highlight: tailwind::GRAY.c700,
        shadow: tailwind::GRAY.c300,
        selected: tailwind::GRAY.c700,
        pressed: tailwind::GRAY.c900,
    };

    pub const BLUE: Theme = Theme {
        text: tailwind::BLUE.c900,
        background: tailwind::BLUE.c500,
        highlight: tailwind::BLUE.c700,
        shadow: tailwind::BLUE.c300,
        selected: tailwind::BLUE.c700,
        pressed: tailwind::BLUE.c900,
    };

    pub const RED: Theme = Theme {
        text: tailwind::RED.c900,
        background: tailwind::RED.c500,
        highlight: tailwind::RED.c700,
        shadow: tailwind::RED.c300,
        selected: tailwind::RED.c700,
        pressed: tailwind::RED.c900,
    };

    pub const GREEN: Theme = Theme {
        text: tailwind::GREEN.c900,
        background: tailwind::GREEN.c500,
        highlight: tailwind::GREEN.c700,
        shadow: tailwind::GREEN.c300,
        selected: tailwind::GREEN.c700,
        pressed: tailwind::GREEN.c900,
    };
}
