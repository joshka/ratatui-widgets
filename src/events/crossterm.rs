use crossterm::event::{
    Event as CrosstermEvent, KeyCode as CrosstermKeyCode, KeyEvent as CrosstermKeyEvent,
    KeyEventKind, KeyModifiers as CrosstermKeyModifiers,
};
use thiserror::Error;

use super::{Event, Key, KeyModifiers, KeyPressedEvent};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ConversionError {
    #[error("Unknown crossterm event: {event:?}")]
    UnknownEvent { event: CrosstermEvent },
    #[error("Unknown crossterm key code: {key_code:?}")]
    UnknownKey { key_code: CrosstermKeyCode },
    #[error("Unknown crossterm modifiers: {modifiers:?}")]
    UnknownModifiers { modifiers: u8 },
    #[error("Key repeated event not supported")]
    KeyReleasedEventNotSupported,
    #[error("Key repeated event not supported")]
    KeyRepeatedEventNotSupported,
}

impl TryFrom<CrosstermEvent> for Event {
    type Error = ConversionError;
    fn try_from(event: CrosstermEvent) -> Result<Self, Self::Error> {
        use CrosstermEvent::*;
        let event = match event {
            Key(key_event) => Event::KeyPressed(key_event.try_into()?),
            _ => return Err(ConversionError::UnknownEvent { event }),
            // TODO maybe handle these later if needed
            // FocusGained => todo!(),
            // FocusLost => todo!(),
            // Mouse(_) => todo!(),
            // Paste(_) => todo!(),
            // Resize(_, _) => todo!(),
        };
        Ok(event)
    }
}

impl TryFrom<CrosstermKeyEvent> for KeyPressedEvent {
    type Error = ConversionError;
    fn try_from(key_event: CrosstermKeyEvent) -> Result<Self, Self::Error> {
        match key_event.kind {
            KeyEventKind::Press => {
                let key = Key::try_from(key_event.code)?;
                let modifiers = KeyModifiers::try_from(key_event.modifiers)?;
                Ok(KeyPressedEvent { key, modifiers })
            }
            KeyEventKind::Release => Err(ConversionError::KeyReleasedEventNotSupported),
            KeyEventKind::Repeat => Err(ConversionError::KeyRepeatedEventNotSupported),
        }
    }
}

impl TryFrom<CrosstermKeyCode> for Key {
    type Error = ConversionError;
    fn try_from(key_code: CrosstermKeyCode) -> Result<Self, Self::Error> {
        use CrosstermKeyCode::*;
        let key = match key_code {
            Char(c) => Key::Char(c),
            Esc => Key::Esc,
            Enter => Key::Enter,
            Tab => Key::Tab,
            BackTab => Key::BackTab,
            Backspace => Key::Backspace,
            Delete => Key::Delete,
            Insert => Key::Insert,
            Left => Key::Left,
            Right => Key::Right,
            Up => Key::Up,
            Down => Key::Down,
            Home => Key::Home,
            End => Key::End,
            PageUp => Key::PageUp,
            PageDown => Key::PageDown,
            F(num) => Key::F(num),

            key_code => return Err(ConversionError::UnknownKey { key_code }),
            // TODO maybe handle these later if needed
            // Null => todo!(),
            // CapsLock => todo!(),
            // ScrollLock => todo!(),
            // NumLock => todo!(),
            // PrintScreen => todo!(),
            // Pause => todo!(),
            // Menu => todo!(),
            // KeypadBegin => todo!(),
            // Media(_) => todo!(),
            // Modifier(_) => todo!(),
        };
        Ok(key)
    }
}

impl TryFrom<CrosstermKeyModifiers> for KeyModifiers {
    type Error = ConversionError;
    fn try_from(modifiers: CrosstermKeyModifiers) -> Result<Self, Self::Error> {
        // our modifiers area superset of crossterm's modifiers and are bit compatible
        KeyModifiers::from_bits(modifiers.bits()).ok_or(ConversionError::UnknownModifiers {
            modifiers: modifiers.bits(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(CrosstermKeyModifiers::empty(), KeyModifiers::empty())]
    #[case(CrosstermKeyModifiers::NONE, KeyModifiers::empty())]
    #[case(CrosstermKeyModifiers::SHIFT, KeyModifiers::SHIFT)]
    #[case(CrosstermKeyModifiers::CONTROL, KeyModifiers::CTRL)]
    #[case(CrosstermKeyModifiers::CONTROL, KeyModifiers::CONTROL)]
    #[case(CrosstermKeyModifiers::ALT, KeyModifiers::ALT)]
    #[case(CrosstermKeyModifiers::ALT, KeyModifiers::OPTION)]
    #[case(CrosstermKeyModifiers::SUPER, KeyModifiers::SUPER)]
    #[case(CrosstermKeyModifiers::SUPER, KeyModifiers::WIN)]
    #[case(CrosstermKeyModifiers::SUPER, KeyModifiers::COMMAND)]
    #[case(CrosstermKeyModifiers::HYPER, KeyModifiers::HYPER)]
    #[case(CrosstermKeyModifiers::META, KeyModifiers::META)]
    #[case(
        CrosstermKeyModifiers::all(),
        KeyModifiers::SHIFT | KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SUPER |
        KeyModifiers::HYPER | KeyModifiers::META,
    )]
    fn try_from_modifiers(
        #[case] modifiers: CrosstermKeyModifiers,
        #[case] expected: KeyModifiers,
    ) {
        let result = KeyModifiers::try_from(modifiers);
        assert_eq!(result, Ok(expected));
    }
}
