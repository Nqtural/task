use std::cell::RefCell;
use std::rc::Rc;

use super::selection::Selection;
use crate::types::Project;

#[derive(Default)]
pub struct State {
    pub selection: Selection,
    pub _show_prompt: bool,
    pub _prompt_text: Option<String>,
    pub projects: Rc<RefCell<Vec<Project>>>,
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
