use crate::{AddArgs, EditArgs, Error, GetArgs, RemoveArgs, Task};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct TaskManager {
    path: PathBuf,
    tasks: Vec<Task>,
}

impl TaskManager {
    fn save(&self) {
        let contents = serde_json::to_string(&self.tasks).unwrap();
        fs::write(&self.path, contents).unwrap();
    }

    pub fn load(path: impl AsRef<Path>) -> Self {
        let tasks = fs::read_to_string(&path).unwrap_or("[]".to_string());
        let tasks = serde_json::from_str(&tasks).unwrap();
        Self {
            tasks,
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn get(&self, args: GetArgs) -> Vec<&Task> {
        if let Some(status) = args.status {
            let tasks = self.tasks.iter().filter(|task| task.status == status);
            tasks.collect()
        } else {
            self.tasks.iter().collect()
        }
    }

    pub fn add(&mut self, args: AddArgs) -> &Task {
        let id = self.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
        let task = Task {
            id,
            content: args.content,
            status: args.status,
        };
        self.tasks.push(task);
        self.save();
        self.tasks.last().unwrap()
    }

    pub fn edit(&mut self, args: EditArgs) -> Result<&Task, Error> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == args.id) {
            let task = self.tasks.get_mut(index).unwrap();
            if let Some(content) = args.content {
                task.content = content;
            }
            if let Some(status) = args.status {
                task.status = status;
            }
            self.save();
            Ok(self.tasks.get(index).unwrap())
        } else {
            Err(Error::TaskNotFound)
        }
    }

    pub fn remove(&mut self, args: RemoveArgs) -> Result<(), Error> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == args.id) {
            self.tasks.remove(index);
            self.save();
            Ok(())
        } else {
            Err(Error::TaskNotFound)
        }
    }
}
