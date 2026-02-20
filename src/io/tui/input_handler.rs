use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll, read};
use std::time::Duration;

use super::action::{Action, Amount, Direction};
use super::prompt::Prompt;

pub fn get_action(ticks_per_second: f32, prompt: &Prompt) -> Result<Action> {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    if poll(Duration::from_millis((1000.0 / ticks_per_second) as u64))? {
        Ok(match read()? {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) => match *prompt {
                Prompt::Confirm(_) => handle_confirm_prompt_keys(code),
                Prompt::Info(_) => Action::PromptCancel,
                Prompt::None => handle_normal_keys(code),
                Prompt::Text(_) => handle_text_prompt_keys(code, modifiers),
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

fn handle_text_prompt_keys(code: KeyCode, modifiers: KeyModifiers) -> Action {
    if modifiers.contains(KeyModifiers::CONTROL) {
        match code {
            KeyCode::Char('a') => Action::PromptNavigate(Direction::Start),
            KeyCode::Char('c') => Action::PromptCancel,
            KeyCode::Char('e') => Action::PromptNavigate(Direction::End),
            KeyCode::Char('w') => Action::PromptDelete(Amount::Word),
            KeyCode::Left => Action::PromptNavigate(Direction::BackwardsWord),
            KeyCode::Right => Action::PromptNavigate(Direction::ForwardsWord),
            _ => Action::None,
        }
    } else {
        match code {
            KeyCode::Backspace => Action::PromptDelete(Amount::PreviousCharacter),
            KeyCode::Char(c) => Action::PromptInput(c),
            KeyCode::Delete => Action::PromptDelete(Amount::CurrentCharacter),
            KeyCode::End => Action::PromptNavigate(Direction::End),
            KeyCode::Enter => Action::PromptAccept,
            KeyCode::Esc => Action::PromptCancel,
            KeyCode::Home => Action::PromptNavigate(Direction::Start),
            KeyCode::Left => Action::PromptNavigate(Direction::Backwards),
            KeyCode::Right => Action::PromptNavigate(Direction::Forwards),
            _ => Action::None,
        }
    }
}

fn handle_normal_keys(code: KeyCode) -> Action {
    match code {
        KeyCode::Char('a') => Action::Add,
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
