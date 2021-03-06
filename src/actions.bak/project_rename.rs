//! Rename the project.

use redo::Command;
use serde::{Serialize, Deserialize};
use simple_error::{SimpleResult, SimpleError, bail};
use std::mem;

use crate::project::h2project::H2Project;

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionProjectRenameForward {
    pub new_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActionProjectRenameBackward {
    old_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionProjectRename {
    forward: Option<ActionProjectRenameForward>,
    backward: Option<ActionProjectRenameBackward>,
}

impl ActionProjectRename {
    pub fn new(forward: ActionProjectRenameForward) -> Self {
        ActionProjectRename {
            forward: Some(forward),
            backward: None,
        }
    }
}

impl From<&str> for ActionProjectRename {
    fn from(o: &str) -> Self {
        ActionProjectRename {
            forward: Some(ActionProjectRenameForward {
                new_name: o.to_string()
            }),
            backward: None,
        }
    }
}

impl Command for ActionProjectRename {
    type Target = H2Project;
    type Error = SimpleError;

    fn apply(&mut self, project: &mut H2Project) -> SimpleResult<()> {
        // Get the forward instructions
        let forward = match &self.forward {
            Some(f) => f,
            None => bail!("Failed to apply: missing context"),
        };

        // Sanity check
        if forward.new_name.len() == 0 {
            bail!("Project name can't be blank");
        }

        // Apply the change
        let old_name = mem::replace(&mut project.name, forward.new_name.clone());

        // Populate backward for undo
        self.backward = Some(ActionProjectRenameBackward {
            old_name: old_name
        });
        self.forward = None;

        Ok(())
    }

    fn undo(&mut self, project: &mut H2Project) -> SimpleResult<()> {
        let backward = match &self.backward {
            Some(b) => b,
            None => bail!("Failed to undo: missing context"),
        };

        let new_name = mem::replace(&mut project.name, backward.old_name.clone());

        self.forward = Some(ActionProjectRenameForward {
            new_name: new_name
        });
        self.backward = None;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_error::SimpleResult;

    use crate::project::h2project::H2Project;
    use redo::Record;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_action() -> SimpleResult<()> {
        let mut record: Record<ActionProjectRename> = Record::new(
            H2Project::new("name", "1.0")
        );
        assert_eq!("name", record.target().name);

        record.apply("newname".into())?;
        assert_eq!("newname", record.target().name);

        record.undo()?;
        assert_eq!("name", record.target().name);

        record.redo()?;
        assert_eq!("newname", record.target().name);

        Ok(())
    }

    #[test]
    fn test_name_is_required() -> SimpleResult<()> {
        let mut record: Record<ActionProjectRename> = Record::new(
            H2Project::new("name", "1.0")
        );

        assert!(record.apply("".into()).is_err());
        assert_eq!("name", record.target().name);

        Ok(())
    }
}
