use ratatui::{prelude::*, style::palette::tailwind};
use ratatui_widgets::toggle_switch::ToggleSwitch;
use ratatui_widgets::{
    events::{self, *},
    toggle_switch::State,
};

#[derive(Debug, Clone)]
pub struct ToggleSwitchTab {
    selected_index: usize,
    switches: Vec<ToggleSwitch<'static>>,
    switch_areas: Vec<Rect>,
}

impl Default for ToggleSwitchTab {
    fn default() -> Self {
        Self {
            selected_index: 0,
            switches: vec![
                ToggleSwitch::new("Turned off", State::Off),
                ToggleSwitch::new("Turned on", State::On),
            ],
            switch_areas: vec![],
        }
    }
}

impl EventHandler for ToggleSwitchTab {
    fn handle_key(&mut self, event: KeyPressedEvent) {
        use events::Key::*;
        match event.key {
            Char('k') | Up => self.select_previous(),
            Char('j') | Down => self.select_next(),
            _ => self.selected_switch_mut().handle_key(event),
        }
    }

    fn handle_mouse(&mut self, event: MouseEvent) {
        if let MouseEventKind::Down(MouseButton::Left) = event.kind {
            self.click(event.column, event.row)
        }
    }
}

impl ToggleSwitchTab {
    pub fn selected_switch_mut(&mut self) -> &mut ToggleSwitch<'static> {
        &mut self.switches[self.selected_index]
    }

    // TODO hit test should be a method on the widget / state
    fn click(&mut self, column: u16, row: u16) {
        for (i, area) in self.switch_areas.iter().enumerate() {
            if area.contains(Position::new(column, row)) {
                // clear current selection
                self.release();
                self.switches[i].toggle_state();
                self.selected_index = i;
                break;
            }
        }
    }

    pub fn release(&mut self) {
        self.selected_switch_mut().blur();
    }

    pub fn select_next(&mut self) {
        self.select_index((self.selected_index + 1) % self.switches.len())
    }

    pub fn select_previous(&mut self) {
        self.select_index((self.selected_index + self.switches.len() - 1) % self.switches.len());
    }

    pub fn select_index(&mut self, index: usize) {
        self.selected_switch_mut().blur();
        self.selected_index = index % self.switches.len();
        self.selected_switch_mut().focus();
    }

    pub fn press(&mut self) {
        self.switches[self.selected_index].toggle_state();
    }
}

/// Required to be mutable because we need to store the button areas for hit testing
impl Widget for &mut ToggleSwitchTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([7, 0]);
        let [buttons, instructions] = layout.areas(area);
        let layout = Layout::vertical([3, 1, 3, 0]);
        let [top, _, bottom, _] = layout.areas(buttons);

        self.switch_areas = vec![top, bottom];

        self.switches[0].render(top, buf);
        self.switches[1].render(bottom, buf);

        Line::raw("←/→: select, space/mouse: press")
            .style(tailwind::SLATE.c300)
            .render(instructions, buf);
    }
}
