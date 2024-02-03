use termion::event::Key as TermionKey;

use super::Key;

impl From<TermionKey> for Key {
    fn from(key: TermionKey) -> Self {
        use Key::*;
        match key {
            TermionKey::Backspace => Backspace,
            TermionKey::Left => Left,
            TermionKey::Right => Right,
            TermionKey::Up => Up,
            TermionKey::Down => Down,
            TermionKey::Home => Home,
            TermionKey::End => End,
            TermionKey::PageUp => PageUp,
            TermionKey::PageDown => PageDown,
            TermionKey::BackTab => BackTab,
            TermionKey::Delete => Delete,
            TermionKey::Insert => Insert,
            TermionKey::F(n) => F(n),
            TermionKey::Char(c) => Char(c),
            TermionKey::Alt(c) => Char(c),
            TermionKey::Ctrl(c) => Char(c),
            TermionKey::Null => Null,
            TermionKey::Esc => Esc,
            _ => Null,
        }
    }
}
