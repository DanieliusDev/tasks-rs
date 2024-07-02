use clap::ValueEnum;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: u16,
    pub content: String,
    pub status: TaskStatus,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {}", self.id.green(), self.content.bright_purple())
    }
}

#[derive(Clone, Copy, Default, Deserialize, PartialEq, Serialize, ValueEnum)]
pub enum TaskStatus {
    Complete,
    InProgress,
    #[default]
    ToDo,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Complete => write!(f, "Complete"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::ToDo => write!(f, "TO-DO"),
        }
    }
}
