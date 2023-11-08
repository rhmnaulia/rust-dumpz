use std::io;

struct Task {
    description: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let new_task = Task {
            description,
            completed: false,
        };
        self.tasks.push(new_task);
        println!("Task added: {}", description);
    }

    fn mark_task_completed(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
            println!("Task marked as completed: {}", task.description);
        } else {
            println!("Invalid task index");
        }
    }

    fn display_tasks(&self) {
        println!("Tasks:");

        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "Completed" } else { "Incomplete" };
            println!("{}: {} ({})", index, task.description, status);
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        println!("Menu:");
        println!("1. Add Task");
        println!("2. Mark Task as Completed");
        println!("3. Display Tasks");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");
                task_manager.add_task(description.trim().to_string());
            }
            "2" => {
                println!("Enter task index to mark as completed:");
                let mut index_str = String::new();
                io::stdin()
                    .read_line(&mut index_str)
                    .expect("Failed to read line");

                if let Ok(index) = index_str.trim().parse::<usize>() {
                    task_manager.mark_task_completed(index);
                } else {
                    println!("Invalid index. Please enter a valid number.");
                }
            }
            "3" => {
                task_manager.display_tasks();
            }
            "4" => {
                println!("Exiting. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}
