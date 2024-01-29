use crossterm::event::{
    Event as CrosstermEvent, KeyCode as CrosstermKeyCode, KeyEvent as CrosstermKeyEvent,
    KeyEventKind, KeyModifiers as CrosstermKeyModifiers, MouseEvent as CrosstermMouseEvent,
};
use thiserror::Error;

use super::MouseEventKind;
use super::{Event, Key, KeyModifiers, KeyPressedEvent, MouseButton, MouseEvent};

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
            Mouse(mouse_event) => Event::Mouse(mouse_event.into()),
            _ => return Err(ConversionError::UnknownEvent { event }),
            // TODO maybe handle these later if needed
            // FocusGained => todo!(),
            // FocusLost => todo!(),
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
            KeyEventKind::Press => Ok(KeyPressedEvent {
                key: key_event.code.try_into()?,
                modifiers: key_event.modifiers.into(),
            }),
            KeyEventKind::Release => Err(ConversionError::KeyReleasedEventNotSupported),
            KeyEventKind::Repeat => Err(ConversionError::KeyRepeatedEventNotSupported),
        }
    }
}

impl TryFrom<CrosstermKeyCode> for Key {
    type Error = ConversionError;
    fn try_from(key_code: CrosstermKeyCode) -> Result<Self, Self::Error> {
        let key = match key_code {
            CrosstermKeyCode::Char(c) => Key::Char(c),
            CrosstermKeyCode::Esc => Key::Esc,
            CrosstermKeyCode::Enter => Key::Enter,
            CrosstermKeyCode::Tab => Key::Tab,
            CrosstermKeyCode::BackTab => Key::BackTab,
            CrosstermKeyCode::Backspace => Key::Backspace,
            CrosstermKeyCode::Delete => Key::Delete,
            CrosstermKeyCode::Insert => Key::Insert,
            CrosstermKeyCode::Left => Key::Left,
            CrosstermKeyCode::Right => Key::Right,
            CrosstermKeyCode::Up => Key::Up,
            CrosstermKeyCode::Down => Key::Down,
            CrosstermKeyCode::Home => Key::Home,
            CrosstermKeyCode::End => Key::End,
            CrosstermKeyCode::PageUp => Key::PageUp,
            CrosstermKeyCode::PageDown => Key::PageDown,
            CrosstermKeyCode::F(num) => Key::F(num),

            key_code => return Err(ConversionError::UnknownKey { key_code }),
            // TODO maybe handle these later if needed
            // CrosstermKeyCode::Null => todo!(),
            // CrosstermKeyCode::CapsLock => todo!(),
            // CrosstermKeyCode::ScrollLock => todo!(),
            // CrosstermKeyCode::NumLock => todo!(),
            // CrosstermKeyCode::PrintScreen => todo!(),
            // CrosstermKeyCode::Pause => todo!(),
            // CrosstermKeyCode::Menu => todo!(),
            // CrosstermKeyCode::KeypadBegin => todo!(),
            // CrosstermKeyCode::Media(_) => todo!(),
            // CrosstermKeyCode::Modifier(_) => todo!(),
        };
        Ok(key)
    }
}

impl From<CrosstermKeyModifiers> for KeyModifiers {
    fn from(modifiers: CrosstermKeyModifiers) -> Self {
        // our modifiers area superset of crossterm's modifiers and are bit compatible so this is
        // safe
        KeyModifiers::from_bits(modifiers.bits()).unwrap()
    }
}

impl From<CrosstermMouseEvent> for MouseEvent {
    fn from(mouse_event: CrosstermMouseEvent) -> Self {
        MouseEvent {
            column: mouse_event.column,
            row: mouse_event.row,
            kind: mouse_event.kind.into(),
            modifiers: mouse_event.modifiers.into(),
        }
    }
}
use crossterm::event::MouseButton as CrosstermMouseButton;
use crossterm::event::MouseEventKind as CrosstermMouseEventKind;

impl From<CrosstermMouseButton> for MouseButton {
    fn from(mouse_button: CrosstermMouseButton) -> Self {
        match mouse_button {
            CrosstermMouseButton::Left => MouseButton::Left,
            CrosstermMouseButton::Right => MouseButton::Right,
            CrosstermMouseButton::Middle => MouseButton::Middle,
        }
    }
}

impl From<CrosstermMouseEventKind> for MouseEventKind {
    fn from(mouse_event_kind: CrosstermMouseEventKind) -> Self {
        match mouse_event_kind {
            CrosstermMouseEventKind::Down(button) => MouseEventKind::Down(button.into()),
            CrosstermMouseEventKind::Up(button) => MouseEventKind::Up(button.into()),
            CrosstermMouseEventKind::Drag(button) => MouseEventKind::Drag(button.into()),
            CrosstermMouseEventKind::Moved => MouseEventKind::Moved,
            CrosstermMouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
            CrosstermMouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
            CrosstermMouseEventKind::ScrollLeft => MouseEventKind::ScrollLeft,
            CrosstermMouseEventKind::ScrollRight => MouseEventKind::ScrollRight,
        }
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
        let result = KeyModifiers::from(modifiers);
        assert_eq!(result, expected);
    }
}
