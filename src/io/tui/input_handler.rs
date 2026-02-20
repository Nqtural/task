use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, poll, read};
use std::time::Duration;

use super::action::Action;
use super::prompt::Prompt;

pub fn get_action(ticks_per_second: f32, prompt: &Prompt) -> Result<Action> {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    if poll(Duration::from_millis((1000.0 / ticks_per_second) as u64))? {
        Ok(match read()? {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match *prompt {
                Prompt::Confirm(_) => handle_confirm_prompt_keys(code),
                Prompt::None => handle_normal_keys(code),
                Prompt::Text(_) => handle_text_prompt_keys(code),
            },
            _ => Action::Tick,
        })
    } else {
        Ok(Action::Tick)
    }
}

fn handle_confirm_prompt_keys(code: KeyCode) -> Action {
    match code {
        KeyCode::Char('y') => Action::PromptAccept,
        _ => Action::PromptCancel,
    }
}

fn handle_text_prompt_keys(code: KeyCode) -> Action {
    match code {
        KeyCode::Backspace => Action::PromptBackspace,
        KeyCode::Char(c) => Action::PromptInput(c),
        KeyCode::Enter => Action::PromptAccept,
        KeyCode::Esc => Action::PromptCancel,
        _ => Action::None,
    }
}

fn handle_normal_keys(code: KeyCode) -> Action {
    match code {
        KeyCode::Char('d') => Action::Delete,
        KeyCode::Char('e') => Action::EditExpiration,
        KeyCode::Char('f') => Action::ToggleFinish,
        KeyCode::Char('h') => Action::MoveLeft,
        KeyCode::Char('j') => Action::MoveDown,
        KeyCode::Char('k') => Action::MoveUp,
        KeyCode::Char('l') => Action::MoveRight,
        KeyCode::Char('n') => Action::EditName,
        KeyCode::Char('q') => Action::Quit,
        _ => Action::Tick,
    }
}
