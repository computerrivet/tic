use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::task::Task;

const STORE_DIR: &str = ".config/tic";
const STORE_FILE: &str = "tasks.json";

/// Persists the task list to disk as JSON.
pub struct Store {
    path: PathBuf,
    tasks: Vec<Task>,
}

impl Store {
    /// Open (or create) the store rooted at `dir`.
    ///
    /// If `dir` is `None`, a local `.config/tic` directory is used.
    pub fn open(dir: Option<PathBuf>) -> Result<Self> {
        let dir = dir.unwrap_or_else(|| PathBuf::from(STORE_DIR));
        let path = dir.join(STORE_FILE);

        let tasks = if path.exists() {
            let raw = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str(&raw)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            Vec::new()
        };

        Ok(Self { path, tasks })
    }

    /// Add a task and return it.
    pub fn add(&mut self, text: String) -> Result<Task> {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let task = Task::new(id, text);
        self.tasks.push(task.clone());
        self.save()?;
        Ok(task)
    }

    /// Return an immutable view of all tasks.
    pub fn list(&self) -> Result<&[Task]> {
        Ok(&self.tasks)
    }

    /// Mark the task with `id` as done.
    pub fn done(&mut self, id: usize) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .with_context(|| format!("no task with id {}", id))?;
        task.done = true;
        self.save()
    }

    /// Update the text of the task with `id`.
    pub fn edit(&mut self, id: usize, text: String) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .with_context(|| format!("no task with id {}", id))?;
        task.text = text;
        self.save()
    }

    /// Remove the task with `id`.
    pub fn remove(&mut self, id: usize) -> Result<()> {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() == before {
            anyhow::bail!("no task with id {}", id);
        }
        self.save()
    }

    /// Remove all tasks.
    pub fn clear(&mut self) -> Result<()> {
        self.tasks.clear();
        self.save()
    }

    fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        let raw = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.path, raw)
            .with_context(|| format!("failed to write {}", self.path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn store(dir: &Path) -> Store {
        Store::open(Some(dir.to_path_buf())).unwrap()
    }

    #[test]
    fn add_and_list() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());

        let t = s.add("first".to_string()).unwrap();
        assert_eq!(t.id, 1);
        assert_eq!(t.text, "first");

        assert_eq!(s.list().unwrap().len(), 1);
    }

    #[test]
    fn ids_increment() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());

        let a = s.add("a".to_string()).unwrap();
        let b = s.add("b".to_string()).unwrap();
        assert_eq!(a.id, 1);
        assert_eq!(b.id, 2);
    }

    #[test]
    fn done_marks_task() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());
        let t = s.add("x".to_string()).unwrap();

        s.done(t.id).unwrap();
        assert!(s.list().unwrap()[0].done);
    }

    #[test]
    fn done_missing_id_errors() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());
        assert!(s.done(99).is_err());
    }

    #[test]
    fn edit_updates_text() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());
        let t = s.add("old".to_string()).unwrap();

        s.edit(t.id, "new".to_string()).unwrap();
        assert_eq!(s.list().unwrap()[0].text, "new");
    }

    #[test]
    fn edit_missing_id_errors() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());
        assert!(s.edit(99, "x".to_string()).is_err());
    }

    #[test]
    fn remove_deletes_task() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());
        let t = s.add("x".to_string()).unwrap();

        s.remove(t.id).unwrap();
        assert!(s.list().unwrap().is_empty());
    }

    #[test]
    fn remove_missing_id_errors() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());
        assert!(s.remove(99).is_err());
    }

    #[test]
    fn clear_removes_all() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = store(dir.path());
        s.add("a".to_string()).unwrap();
        s.add("b".to_string()).unwrap();

        s.clear().unwrap();
        assert!(s.list().unwrap().is_empty());
    }
}
