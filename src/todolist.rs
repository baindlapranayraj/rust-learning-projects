use crate::task::Task;

pub struct Todolist {
    todos: Vec<Task>,
    id: usize,
}

impl Todolist {
    pub fn new() -> Todolist {
        Todolist {
            todos: Vec::new(),
            id: 1,
        }
    }

    pub fn add_todo(&mut self, task_desc: String) {
        let task = Task::new(self.id, task_desc);
        self.todos.push(task);
        self.id += 1;
        println!("===================================================");
        println!("Your Task is added ✅");
        println!("===================================================");
    }

    pub fn read_todos(&self) {
        if self.todos.len() == 0 {
            println!("No tasks are present");
        }else {
            for task in self.todos.iter() {
                println!("===================================================");
                println!("{}-{}-{}",task.id,task.description,task.is_completed);
                println!("===================================================");
            }
        }
    }

    pub fn update_task_list(&mut self, id: usize) {
        if let Some(task) = self.todos.get_mut(id-1) {
            task.update_task();
            println!("===================================================");
            println!("Task is updated to done ✅");
            println!("===================================================");
        } else {
            println!("===================================================");
            println!("Invalid Id ❌ from function itself");
            println!("===================================================");
        }
    }

    pub fn update_desc_list(&mut self, desc: String, id: usize) {
        if let Some(task) = self.todos.get_mut(id-1) {
            task.update_task_desc(desc);
        }
    }

    pub fn delete_task(&mut self, id: usize) {
        self.todos.remove(id-1);
        println!("===================================================");
        println!("Removed Task {:?}",id);
        println!("===================================================");
    }
}
