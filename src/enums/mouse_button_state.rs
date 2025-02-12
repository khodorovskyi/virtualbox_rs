#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum MouseButtonState {
    None,
    Left,
    Right,
    Middle,
}

impl Into<i32> for MouseButtonState {
    fn into(self) -> i32 {
        match self {
            MouseButtonState::None => 0x00,
            MouseButtonState::Left => 0x01,
            MouseButtonState::Right => 0x02,
            MouseButtonState::Middle => 0x04,
        }
    }
}

impl From<i32> for MouseButtonState {
    fn from(value: i32) -> Self {
        match value {
            1 => MouseButtonState::Left,
            2 => MouseButtonState::Right,
            4 => MouseButtonState::Middle,
            _ => MouseButtonState::None,
        }
    }
}
