use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{prelude::*, style::palette::tailwind};
use ratatui_widgets::{button, Button};

#[derive(Debug, Clone)]
pub struct ButtonsTab {
    selected: usize,
    buttons: Vec<Button<'static>>,
    button_areas: Vec<Rect>,
}

impl Default for ButtonsTab {
    fn default() -> Self {
        Self {
            selected: 0,
            buttons: vec![
                Button::new("Button 1").with_theme(button::themes::RED),
                Button::new("Button 2").with_theme(button::themes::GREEN),
                Button::new("Button 3").with_theme(button::themes::BLUE),
            ],
            button_areas: vec![],
        }
    }
}

impl ButtonsTab {
    // TODO: this should hanle press and release events to allow for a "click" effect
    pub fn handle_key_press(&mut self, key: KeyEvent) {
        use KeyCode::*;
        match key.code {
            Char('j') | Left => self.select_previous(),
            Char('k') | Right => self.select_next(),
            Char(' ') | Enter => self.press(),
            _ => {}
        }
    }

    // TODO: this should be a method on the widget / state
    pub fn handle_mouse_event(&mut self, event: MouseEvent) {
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => self.click(event.column, event.row),
            MouseEventKind::Up(_) => self.release(),
            _ => {}
        }
    }

    // TODO hit test should be a method on the widget / state
    fn click(&mut self, column: u16, row: u16) {
        for (i, area) in self.button_areas.iter().enumerate() {
            // TODO Rect should have a contains method
            let area_contains_click = area.left() <= column
                && column < area.right()
                && area.top() <= row
                && row < area.bottom();
            if area_contains_click {
                // clear current selection
                self.release();
                self.buttons[i].press();
                self.selected = i;
                break;
            }
        }
    }

    fn release(&mut self) {
        self.buttons[self.selected].select();
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

impl Widget for &mut ButtonsTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_buttons(area, buf);
    }
}

impl ButtonsTab {
    fn render_buttons(&mut self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([3, 0]);
        let [buttons, instructions] = area.split(&layout);
        let layout = Layout::horizontal([20, 1, 20, 1, 20, 0]);
        let [left, _, middle, _, right, _] = buttons.split(&layout);

        self.button_areas = vec![left, middle, right];
        self.buttons[0].render(left, buf);
        self.buttons[1].render(middle, buf);
        self.buttons[2].render(right, buf);

        Line::raw("←/→: select, space/mouse: press")
            .style(tailwind::SLATE.c300)
            .render(instructions, buf);
    }
}
