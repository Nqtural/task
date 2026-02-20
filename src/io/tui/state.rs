use std::cell::RefCell;
use std::rc::Rc;

use super::pending_action::PendingAction;
use super::prompt::Prompt;
use super::selection::Selection;
use crate::types::Project;

#[derive(Default)]
pub struct State {
    pub pending_action: PendingAction,
    pub projects: Rc<RefCell<Vec<Project>>>,
    pub prompt: Prompt,
    pub selection: Selection,
}

impl State {
    pub fn new() -> Self {
        let projects = Rc::new(RefCell::new(Vec::new()));
        Self {
            selection: Selection::new(projects.clone()),
            projects,
            ..Default::default()
        }
    }
}
