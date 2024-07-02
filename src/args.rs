use crate::TaskStatus;
use clap::Args;

#[derive(Args, Clone, Copy)]
pub struct GetArgs {
    /// Only list the tasks with the specified statusc
    #[arg(long, short)]
    pub status: Option<TaskStatus>,
}

#[derive(Args)]
pub struct AddArgs {
    /// The content of the task
    pub content: String,
    /// The status of the task
    #[arg(default_value_t, long, short, value_enum)]
    pub status: TaskStatus,
}

#[derive(Args)]
pub struct EditArgs {
    /// The ID of the task to edit
    pub id: u16,
    /// The content of the task
    pub content: Option<String>,
    /// The status of the task
    #[arg(long, short)]
    pub status: Option<TaskStatus>,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// The ID of the task to remove
    pub id: u16,
}
