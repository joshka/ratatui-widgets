use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::WidgetRef;
use std::fmt;

/// A container that stacks widgets in a given direction
///
/// # Examples
///
/// ```rust
/// use ratatui::prelude::*;
/// use ratatui::widgets::*;
/// use ratatui_widgets::StackContainer;
///
/// let mut stack = StackContainer::horizontal();
/// stack.push(Box::new(Paragraph::new("Left")), Constraint::Fill(1));
/// stack.push(Box::new(Paragraph::new("Right")), Constraint::Fill(1));
/// ```
#[derive(Default)]
pub struct StackContainer {
    direction: Direction,
    flex: Flex,
    margin: u16,
    spacing: u16,
    widgets: Vec<(Box<dyn WidgetRef>, Constraint)>,
}

impl fmt::Debug for StackContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StackContainer")
            .field("direction", &self.direction)
            .finish_non_exhaustive()
    }
}

impl StackContainer {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            ..Default::default()
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Direction::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new(Direction::Vertical)
    }

    pub fn with_margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    pub fn with_widget(mut self, widget: Box<dyn WidgetRef>, constraint: Constraint) -> Self {
        self.widgets.push((widget, constraint));
        self
    }

    pub fn with_widgets(mut self, widgets: Vec<(Box<dyn WidgetRef>, Constraint)>) -> Self {
        self.widgets = widgets;
        self
    }

    pub fn push(&mut self, widget: Box<dyn WidgetRef>, constraint: Constraint) {
        self.widgets.push((widget, constraint));
    }

    pub fn remove(&mut self, index: usize) {
        self.widgets.remove(index);
    }
}

impl Widget for &StackContainer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(self.direction)
            .flex(self.flex)
            .margin(self.margin)
            .spacing(self.spacing)
            .constraints(self.widgets.iter().map(|(_, c)| *c));
        let areas = layout.split(area);
        let widgets = self.widgets.iter().map(|(w, _)| w);
        for (widget, area) in widgets.zip(areas.iter()) {
            widget.render_ref(*area, buf);
        }
    }
}
