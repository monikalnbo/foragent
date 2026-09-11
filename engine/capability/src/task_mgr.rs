use myagent_types::{TaskItem, TaskStatus};

#[derive(Debug, Default, Clone)]
pub struct TaskManager {
    tasks: Vec<TaskItem>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn init_tasks(&mut self, titles: Vec<String>) {
        self.tasks = titles
            .into_iter()
            .enumerate()
            .map(|(idx, title)| TaskItem {
                id: format!("T-{}", idx + 1),
                title,
                status: TaskStatus::Pending,
                summary: None,
            })
            .collect();
    }

    pub fn update_status(&mut self, id: &str, status: TaskStatus, summary: Option<String>) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = status;
            if summary.is_some() {
                task.summary = summary;
            }
            true
        } else {
            false
        }
    }

    pub fn get_tasks(&self) -> &[TaskItem] {
        &self.tasks
    }

    pub fn get_next_pending(&self) -> Option<&TaskItem> {
        self.tasks
            .iter()
            .find(|t| t.status == TaskStatus::Pending || t.status == TaskStatus::InProgress)
    }

    pub fn is_all_completed(&self) -> bool {
        !self.tasks.is_empty()
            && self
                .tasks
                .iter()
                .all(|t| matches!(t.status, TaskStatus::Completed))
    }

    pub fn to_markdown(&self) -> String {
        let mut md = String::from("### 任务执行清单\n");
        for t in &self.tasks {
            let icon = match &t.status {
                TaskStatus::Pending => "☐",
                TaskStatus::InProgress => "▶",
                TaskStatus::Completed => "☑",
                TaskStatus::Failed(_) => "☒",
            };
            md.push_str(&format!("- {} **[{}]** {}", icon, t.id, t.title));
            if let Some(sum) = &t.summary {
                md.push_str(&format!(" (进展: {})", sum));
            }
            md.push('\n');
        }
        md
    }
}
