use std::cell::RefCell;
use std::rc::Rc;

use crate::types::Project;

#[derive(Default)]
pub enum Level {
    #[default]
    Project,
    Task,
}

#[derive(Default)]
pub struct Selection {
    pub project: Option<usize>,
    pub task: Option<usize>,
    pub level: Level,
    projects: Rc<RefCell<Vec<Project>>>,
}

impl Selection {
    pub fn new(projects: Rc<RefCell<Vec<Project>>>) -> Self {
        Self {
            projects,
            ..Default::default()
        }
    }

    fn current_vector_len(&self) -> usize {
        match self.level {
            Level::Project => self.projects.borrow().len(),
            Level::Task => {
                if let Some(proj_idx) = self.project {
                    self.projects.borrow()[proj_idx].tasks.len()
                } else {
                    0
                }
            }
        }
    }

    fn advance_index(index: &mut Option<usize>, len: usize) {
        *index = index.map(|i| (i + 1).min(len.saturating_sub(1))).or(None);
    }

    pub fn next(&mut self) {
        let len = self.current_vector_len();
        match self.level {
            Level::Project => {
                Self::advance_index(&mut self.project, len);

                // select last task if the new project does
                // not contain as many tasks as the last one
                let projects = self.projects.borrow();
                if let Some(selected_project) = self.project.and_then(|pi| projects.get(pi)) {
                    if let Some(task_index) = self.task {
                        if task_index >= selected_project.tasks.len() {
                            self.task = Some(selected_project.tasks.len().saturating_sub(1));
                        }
                    } else {
                        // current project has tasks but self.task is None
                        self.task = Some(0);
                    }
                }
            }
            Level::Task => Self::advance_index(&mut self.task, len),
        }
    }

    pub fn previous(&mut self) {
        let len = self.current_vector_len();
        let advance_down = |i: usize| i.saturating_sub(1);

        match self.level {
            Level::Project => {
                self.project = self
                    .project
                    .map(advance_down)
                    .or_else(|| if len > 0 { Some(len - 1) } else { None });
            }
            Level::Task => {
                self.task = self
                    .task
                    .map(advance_down)
                    .or_else(|| if len > 0 { Some(len - 1) } else { None });
            }
        }
    }

    pub fn toggle_level(&mut self) {
        self.level = if matches!(self.level, Level::Project) {
            Level::Task
        } else {
            Level::Project
        }
    }

    pub fn get_selected_project_id(&self) -> Option<u32> {
        self.projects.borrow().get(self.project?).map(|p| p.id)
    }

    pub fn get_selected_task_id(&self) -> Option<u32> {
        self.projects
            .borrow()
            .get(self.project?)
            .map(|p| &p.tasks)?
            .get(self.task?)
            .map(|t| t.id)
    }
}
