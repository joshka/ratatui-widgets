use termion::event::Key as TermionKey;

use super::KeyPressedEvent;

impl From<TermionKey> for KeyPressedEvent {
    fn from(key: TermionKey) -> Self {
        use TermionKey::*;
        match key {
            Backspace => todo!(),
            Left => todo!(),
            Right => todo!(),
            Up => todo!(),
            Down => todo!(),
            Home => todo!(),
            End => todo!(),
            PageUp => todo!(),
            PageDown => todo!(),
            BackTab => todo!(),
            Delete => todo!(),
            Insert => todo!(),
            F(_) => todo!(),
            Char(_) => todo!(),
            Alt(_) => todo!(),
            Ctrl(_) => todo!(),
            Null => todo!(),
            Esc => todo!(),
            _ => todo!(),
        }
    }
}
