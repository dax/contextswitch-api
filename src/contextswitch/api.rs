use crate::contextswitch::taskwarrior;
use contextswitch_types::Task;
use serde_json;

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

impl std::fmt::Debug for ContextswitchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum ContextswitchError {
    #[error("Invalid Contextswitch metadata: {metadata}")]
    InvalidMetadataError {
        #[source]
        source: serde_json::Error,
        metadata: String,
    },
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(level = "debug")]
pub fn list_tasks(filters: Vec<&str>) -> Result<Vec<Task>, ContextswitchError> {
    let tasks: Result<Vec<Task>, ContextswitchError> = taskwarrior::list_tasks(filters)
        .map_err(|e| ContextswitchError::UnexpectedError(e.into()))?
        .iter()
        .map(Task::try_from)
        .collect();
    tasks
}

#[tracing::instrument(level = "debug")]
pub async fn add_task(add_args: Vec<&str>) -> Result<Task, ContextswitchError> {
    let taskwarrior_task = taskwarrior::add_task(add_args)
        .await
        .map_err(|e| ContextswitchError::UnexpectedError(e.into()))?;
    (&taskwarrior_task).try_into()
}
