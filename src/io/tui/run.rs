use anyhow::Result;

use super::Tui;
use super::action::Action;
use super::input_handler::get_action;
use super::pending_action::PendingAction;
use super::prompt::Prompt;

const TICKS_PER_SECOND: f32 = 1.0;

impl Tui {
    pub fn run(&mut self) -> Result<()> {
        *self.state.projects.borrow_mut() = self.storage.get_all_projects()?;
        if !self.state.projects.borrow().is_empty() {
            match self.storage.get_current_project_id()? {
                Some(project_id) => {
                    self.state.selection.project = self
                        .state
                        .projects
                        .borrow_mut()
                        .iter()
                        .position(|p| project_id == p.id);

                    if self
                        .state
                        .projects
                        .borrow()
                        .get(self.state.selection.project.unwrap())
                        .is_some()
                    {
                        self.state.selection.task = Some(0);
                    }
                }
                None => self.state.selection.project = Some(0),
            }
        }
        loop {
            self.ui.draw(&self.state)?;

            match get_action(TICKS_PER_SECOND, &self.state.prompt)? {
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
                Action::None => continue,
                Action::PromptAccept => {
                    self.execute_pending_action()?;
                    self.state.pending_action = PendingAction::None;
                    self.state.prompt = Prompt::None;
                }
                Action::PromptBackspace => {
                    if let Prompt::Text((_, text)) = &mut self.state.prompt {
                        text.pop();
                    }
                }
                Action::PromptCancel => {
                    self.state.pending_action = PendingAction::None;
                    self.state.prompt = Prompt::None;
                }
                Action::PromptInput(c) => {
                    if let Prompt::Text((_, text)) = &mut self.state.prompt {
                        text.push(c);
                    }
                }
                Action::Quit => return Ok(()),
                Action::Tick => {}
                Action::ToggleFinish => self.toggle_finish(),
            }

            *self.state.projects.borrow_mut() = self.storage.get_all_projects()?;
        }
    }
}
