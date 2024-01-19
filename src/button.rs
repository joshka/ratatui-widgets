#![allow(unused)]

use ratatui::{prelude::*, style::palette::tailwind, widgets::Widget};

#[derive(Debug, Default, Clone)]
pub struct Button<'a> {
    text: Text<'a>,
    theme: Theme,
    state: State,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}

impl Default for Theme {
    fn default() -> Self {
        NORMAL
    }
}

pub const NORMAL: Theme = Theme {
    text: tailwind::GRAY.c900,
    background: tailwind::GRAY.c500,
    highlight: tailwind::GRAY.c700,
    shadow: tailwind::GRAY.c300,
};

pub const BLUE: Theme = Theme {
    text: tailwind::BLUE.c900,
    background: tailwind::BLUE.c500,
    highlight: tailwind::BLUE.c700,
    shadow: tailwind::BLUE.c300,
};

pub const RED: Theme = Theme {
    text: tailwind::RED.c900,
    background: tailwind::RED.c500,
    highlight: tailwind::RED.c700,
    shadow: tailwind::RED.c300,
};

pub const GREEN: Theme = Theme {
    text: tailwind::GREEN.c900,
    background: tailwind::GREEN.c500,
    highlight: tailwind::GREEN.c700,
    shadow: tailwind::GREEN.c300,
};

impl<'text> Button<'text> {
    pub fn new(text: Text<'text>) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl<'text, T: Into<Text<'text>>> From<T> for Button<'text> {
    fn from(text: T) -> Self {
        Self::new(text.into())
    }
}

impl<'text> Widget for &Button<'text> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Theme {
            text,
            background,
            highlight,
            shadow,
        } = self.theme;
        buf.set_style(area, Style::new().bg(background).fg(text));
        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new().fg(highlight).bg(background),
            );
        }
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new().fg(shadow).bg(background),
            );
        }

        // TODO: this becomes just: self.text.render(area, buf);
        for (line, row) in self.text.lines.iter().zip(area.rows().skip(1)) {
            let x = area.x + (area.width - line.width() as u16) / 2;
            buf.set_line(x, row.y, &line, row.width);
        }
    }
}
