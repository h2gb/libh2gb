use redo::Command;
use serde::{Serialize, Deserialize};
use simple_error::{SimpleResult, SimpleError};

use crate::h2project::H2Project;

pub mod null;
pub mod project_rename;
pub mod buffer_create_empty;

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Null(null::NullAction),
    ProjectRename(project_rename::ActionProjectRename),
    BufferCreateEmpty(buffer_create_empty::ActionBufferCreateEmpty),
}

impl Command for Action {
    type Target = H2Project;
    type Error = SimpleError;

    fn apply(&mut self, project: &mut H2Project) -> SimpleResult<()> {
        match self {
            Action::Null(a) => a.apply(project),
            Action::ProjectRename(a) => a.apply(project),
            Action::BufferCreateEmpty(a) => a.apply(project),
        }
    }

    fn undo(&mut self, project: &mut H2Project) -> SimpleResult<()> {
        match self {
            Action::Null(a) => a.undo(project),
            Action::ProjectRename(a) => a.undo(project),
            Action::BufferCreateEmpty(a) => a.undo(project),
        }
    }
}