use ratatui::{prelude::*, style::palette::tailwind};
use ratatui_widgets::{
    button,
    events::{self, *},
    Button,
};

#[derive(Debug, Clone)]
pub struct ButtonsExample {
    selected_index: usize,
    buttons: Vec<Button<'static>>,
    button_areas: Vec<Rect>,
}

impl Default for ButtonsExample {
    fn default() -> Self {
        Self {
            selected_index: 0,
            buttons: vec![
                Button::new("Button 1").with_theme(button::themes::RED),
                Button::new("Button 2").with_theme(button::themes::GREEN),
                Button::new("Button 3").with_theme(button::themes::BLUE),
            ],
            button_areas: vec![],
        }
    }
}

impl EventHandler for ButtonsExample {
    fn handle_key(&mut self, event: KeyPressedEvent) {
        use events::Key::*;
        match event.key {
            Char('j') | Left => self.select_previous(),
            Char('k') | Right => self.select_next(),
            _ => self.selected_button_mut().handle_key(event),
        }
    }

    fn handle_mouse(&mut self, event: MouseEvent) {
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => self.click(event.column, event.row),
            MouseEventKind::Up(_) => self.release(),
            _ => {}
        }
    }
}

impl ButtonsExample {
    pub fn selected_button_mut(&mut self) -> &mut Button<'static> {
        &mut self.buttons[self.selected_index]
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
                self.buttons[i].toggle_press();
                self.selected_index = i;
                break;
            }
        }
    }

    fn release(&mut self) {
        self.selected_button_mut().select();
    }

    pub fn select_next(&mut self) {
        self.select_index((self.selected_index + 1) % self.buttons.len())
    }

    pub fn select_previous(&mut self) {
        self.select_index((self.selected_index + self.buttons.len() - 1) % self.buttons.len());
    }

    pub fn select_index(&mut self, index: usize) {
        self.selected_button_mut().normal();
        self.selected_index = index % self.buttons.len();
        self.selected_button_mut().select();
    }

    pub fn press(&mut self) {
        self.buttons[self.selected_index].toggle_press();
    }
}

/// Required to be mutable because we need to store the button areas for hit testing
impl Widget for &mut ButtonsExample {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
