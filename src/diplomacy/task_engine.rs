#[derive(Debug)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

pub struct Task {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: u8,
}

impl Task {
    pub fn new(id: &str, description: &str, priority: u8) -> Self {
        Self {
            id: id.to_string(),
            description: description.to_string(),
            status: TaskStatus::Pending,
            priority,
        }
    }

    pub fn update_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }
}
