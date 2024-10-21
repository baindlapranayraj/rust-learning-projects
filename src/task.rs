#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub is_completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            is_completed: false,
        }
    }
    pub fn update_task(&mut self) {
        self.is_completed = true;
    }
    pub fn update_task_desc(&mut self, update_description: String) {
        self.description = update_description;
    }
}
