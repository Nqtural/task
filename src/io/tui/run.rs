use anyhow::Result;

use super::Tui;
use super::action::{Action, Amount, Direction};
use super::input_handler::get_action;
use super::pending_action::PendingAction;
use super::prompt::Prompt;

const TICKS_PER_SECOND: f32 = 1.0;

impl Tui {
    pub fn run(&mut self) -> Result<()> {
        self.initialize()?;
        loop {
            self.ui.draw(&self.state)?;

            let action = get_action(TICKS_PER_SECOND, &self.state.prompt)?;
            if !self.handle_action(action)? {
                return Ok(());
            }

            *self.state.projects.borrow_mut() = self.storage.get_all_projects()?;
        }
    }

    fn handle_action(&mut self, action: Action) -> Result<bool> {
        match action {
            Action::Add => {
                self.state.pending_action = PendingAction::Add;
                self.prompt_add();
            }
            Action::Delete => {
                self.state.pending_action = PendingAction::Delete;
                self.prompt_delete_selected();
            }
            Action::EditExpiration => {
                self.state.pending_action = PendingAction::EditExpiration;
                self.prompt_edit_expiration()?;
            }
            Action::EditName => {
                self.state.pending_action = PendingAction::EditName;
                self.prompt_edit_name()?;
            }
            Action::MoveDown => self.state.selection.next(),
            Action::MoveLeft | Action::MoveRight => self.state.selection.toggle_level(),
            Action::MoveUp => self.state.selection.previous(),
            Action::None => {}
            Action::PromptAccept => {
                self.execute_pending_action()?;
                self.state.pending_action = PendingAction::None;
                self.state.prompt = Prompt::None;
            }
            Action::PromptCancel => {
                self.state.pending_action = PendingAction::None;
                self.state.prompt = Prompt::None;
            }
            Action::PromptDelete(amount) => self.handle_prompt_delete(amount)?,
            Action::PromptInput(c) => {
                if let Prompt::Text((_, text, cursor)) = &mut self.state.prompt {
                    text.insert(*cursor, c);
                    *cursor += 1;
                }
            }
            Action::PromptNavigate(direction) => self.handle_prompt_navigation(direction)?,
            Action::Quit => return Ok(false),
            Action::Tick => {}
            Action::ToggleFinish => self.toggle_finish(),
        }

        Ok(true)
    }

    fn handle_prompt_delete(&mut self, amount: Amount) -> Result<()> {
        let (text, cursor) = match &mut self.state.prompt {
            Prompt::Text((_, text, cursor)) => (text, cursor),
            _ => return Ok(()),
        };

        match amount {
            Amount::CurrentCharacter => {
                if *cursor < text.len() {
                    text.remove(*cursor);
                }
            }
            Amount::PreviousCharacter => {
                if *cursor > 0 {
                    text.remove(*cursor - 1);
                    *cursor -= 1;
                }
            }
            Amount::Word => delete_word(text, cursor),
        }

        Ok(())
    }

    fn handle_prompt_navigation(&mut self, direction: Direction) -> Result<()> {
        let (text, cursor) = match &mut self.state.prompt {
            Prompt::Text((_, text, cursor)) => (text, cursor),
            _ => return Ok(()),
        };

        match direction {
            Direction::Backwards => {
                if *cursor > 0 {
                    *cursor -= 1;
                }
            }
            Direction::BackwardsWord => move_word_forward(text, cursor),
            Direction::End => *cursor = text.len(),
            Direction::Forwards => {
                if *cursor < text.len() {
                    *cursor += 1;
                }
            }
            Direction::ForwardsWord => move_word_backward(text, cursor),
            Direction::Start => *cursor = 0,
        }

        Ok(())
    }

    fn initialize(&mut self) -> Result<()> {
        let mut projects = self.state.projects.borrow_mut();
        *projects = self.storage.get_all_projects()?;
        if projects.is_empty() {
            self.state.selection.project = None;
            self.state.selection.task = None;
            return Ok(());
        }

        self.state.selection.project = match self.storage.get_current_project_id()? {
            Some(project_id) => projects.iter().position(|p| p.id == project_id),
            None => Some(0),
        };

        self.state.selection.task = Some(0);

        Ok(())
    }
}

fn delete_word(text: &mut Vec<char>, cursor: &mut usize) {
    if *cursor == 0 {
        return;
    }

    let mut start = *cursor;

    while start > 0 && text[start - 1].is_whitespace() {
        start -= 1;
    }

    while start > 0 && !text[start - 1].is_whitespace() {
        start -= 1;
    }

    text.drain(start..*cursor);
    *cursor = start;
}

fn move_word_backward(text: &mut [char], cursor: &mut usize) {
    let len = text.len();
    if *cursor >= len {
        return;
    }

    while *cursor < len && text[*cursor].is_whitespace() {
        *cursor += 1;
    }

    while *cursor < len && !text[*cursor].is_whitespace() {
        *cursor += 1;
    }
}

fn move_word_forward(text: &[char], cursor: &mut usize) {
    if *cursor == 0 {
        return;
    }

    while *cursor > 0 && text[*cursor - 1].is_whitespace() {
        *cursor -= 1;
    }

    while *cursor > 0 && !text[*cursor - 1].is_whitespace() {
        *cursor -= 1;
    }
}
