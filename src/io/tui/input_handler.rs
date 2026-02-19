use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, poll, read};
use std::time::Duration;

use super::action::Action;

pub fn get_action(ticks_per_second: f32) -> Result<Action> {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    if poll(Duration::from_millis((1000.0 / ticks_per_second) as u64))? {
        Ok(match read()? {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Char('h') => Action::MoveLeft,
                KeyCode::Char('j') => Action::MoveDown,
                KeyCode::Char('l') => Action::MoveRight,
                KeyCode::Char('k') => Action::MoveUp,
                KeyCode::Char('q') => Action::Quit,
                _ => Action::Tick,
            },
            _ => Action::Tick,
        })
    } else {
        Ok(Action::Tick)
    }
}
