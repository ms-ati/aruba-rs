use crate::api::command_run::PathOrTemp;

/// Either an existing `PathOrTemp` or a `String` prefix to create a new temp dir.
pub enum ExistingOrFromPrefix {
    PathOrTemp(PathOrTemp),
    FromPrefix(String),
}
