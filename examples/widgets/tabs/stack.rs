use rand::Rng;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui_widgets::events::{EventHandler, Key, KeyPressedEvent, MouseEvent};
use ratatui_widgets::StackContainer;

#[derive(Debug)]
pub struct StackTab {
    counter: usize,
    stack: StackContainer,
}

impl Default for StackTab {
    fn default() -> Self {
        use Constraint::*;
        let stack = StackContainer::vertical().with_widgets(vec![
            // a couple of widgets to start with
            (
                Box::new(
                    Span::raw("First Span")
                        .bg(Color::LightBlue)
                        .fg(Color::Black),
                ),
                Length(1),
            ),
            (
                Box::new(
                    Paragraph::new("Second\n(paragraph)")
                        .bg(Color::LightYellow)
                        .fg(Color::Black),
                ),
                Length(2),
            ),
        ]);
        StackTab { counter: 2, stack }
    }
}

impl Widget for &StackTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let [instruction, items] = Layout::vertical([Length(1), Fill(1)]).areas(area);
        Line::from("Press space to add a new item, backspace to remove the last one.")
            .centered()
            .render(instruction, buf);
        self.stack.render(items, buf);
    }
}

impl EventHandler for StackTab {
    fn handle_key(&mut self, event: KeyPressedEvent) {
        match event.key {
            Key::Char(' ') => self.add_widget(),
            Key::Backspace => self.remove_widget(),
            _ => {}
        }
    }

    fn handle_mouse(&mut self, _event: MouseEvent) {}
}

impl StackTab {
    fn add_widget(&mut self) {
        self.counter += 1;
        let (widget, constraint) = self.random_widget();
        self.stack.push(widget, constraint);
    }

    fn remove_widget(&mut self) {
        if self.counter == 0 {
            return;
        }
        self.counter -= 1;
        self.stack.remove(self.counter);
    }

    fn random_widget(&mut self) -> (Box<dyn WidgetRef>, Constraint) {
        let text = format!("Item {}", self.counter);

        let mut rng = rand::thread_rng();

        let color_index = rng.gen_range(0..=255);
        let style = Style::new()
            .bg(Color::Indexed(color_index))
            .fg(Color::Black);

        let choice = rng.gen_range(0..3);

        let widget: Box<dyn WidgetRef> = match choice {
            0 => Box::new(Paragraph::new(format!("{}\n(paragraph)", text)).style(style)),
            1 => Box::new(Line::styled(format!("{} (line)", text), style)),
            2 => Box::new(Span::styled(format!("{} (span)", text), style)),
            _ => unreachable!(),
        };
        let constraint = match choice {
            0 => Constraint::Length(2),
            1 => Constraint::Length(1),
            2 => Constraint::Length(1),
            _ => unreachable!(),
        };

        (widget, constraint)
    }
}
