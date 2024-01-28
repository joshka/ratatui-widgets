use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui_widgets::{button, Button};

pub struct ButtonsTab {
    selected: usize,
    buttons: Vec<Button<'static>>,
}

impl ButtonsTab {
    pub fn new() -> Self {
        Self {
            selected: 0,
            buttons: vec![
                Button::new("Button 1").with_theme(button::themes::RED),
                Button::new("Button 2").with_theme(button::themes::GREEN),
                Button::new("Button 3").with_theme(button::themes::BLUE),
            ],
        }
    }
}

impl ButtonsTab {
    pub fn handle_event(&mut self, event: Event) {
        use KeyCode::*;
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('j') | Left => self.select_previous(),
                Char('k') | Right => self.select_next(),
                Char(' ') | Enter => self.press(),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn select_next(&mut self) {
        self.buttons[self.selected].normal();
        self.selected = (self.selected + 1) % self.buttons.len();
        self.buttons[self.selected].select();
    }

    pub fn select_previous(&mut self) {
        self.buttons[self.selected].normal();
        self.selected = (self.selected + self.buttons.len() - 1) % self.buttons.len();
        self.buttons[self.selected].select();
    }

    pub fn press(&mut self) {
        self.buttons[self.selected].press();
    }
}

impl Widget for &ButtonsTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_buttons(area, buf);
    }
}

impl ButtonsTab {
    fn render_buttons(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([3, 0]);
        let [buttons, _] = area.split(&layout);
        let layout = Layout::horizontal([20, 1, 20, 1, 20, 0]);
        let [left, _, middle, _, right, _] = buttons.split(&layout);
        self.buttons[0].render(left, buf);
        self.buttons[1].render(middle, buf);
        self.buttons[2].render(right, buf);
    }
}
