use anyhow::Result;

use super::Tui;
use super::action::Action;
use super::input_handler::get_action;

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

            match get_action(TICKS_PER_SECOND)? {
                Action::MoveDown => self.state.selection.next(),
                Action::MoveLeft | Action::MoveRight => self.state.selection.toggle_level(),
                Action::MoveUp => self.state.selection.previous(),
                Action::Quit => return Ok(()),
                Action::Tick => {}
            }

            *self.state.projects.borrow_mut() = self.storage.get_all_projects()?;
        }
    }
}
