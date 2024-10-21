mod task;
mod todolist;

use std::io::{self};
use std::io::prelude::*;
use todolist::Todolist;

fn main() {
    println!("================= Welcome to Todlist =================");
    let mut todos = Todolist::new();
    loop {
        println!("Choose the following option");
        println!("1.Add new task");
        println!("2.Read all tasks");
        println!("3.Delete a task");
        println!("4.Update task to done");
        println!("5.Update task description");
        println!("6.Exit");

        io::stdout().flush().unwrap();
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Invalid option");
        let option: u32 = option.trim().parse().expect("Invalid number ❌");

        match option {
            1 => {
                println!("Enter your task hear: 👇🏻");
                let mut task = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut task).expect("Invalid Task");
                todos.add_todo(task.trim().to_string());
            }
            2 => {
                todos.read_todos();
            }
            3 => {
                println!("If want to delet your task enter task Id");
                io::stdout().flush().unwrap();
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).expect("Invalid Id");

                 match task_id.trim().parse() {
                    Ok(number) => todos.delete_task(number),
                    Err(_) => {
                        println!("Enter Valid input ❌");
                        continue;
                    }
                };
            }

            4 => {
                println!("If you want to update the task to done Enter id");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).expect("Invalid input ❌");
                let id:usize = task_id.trim().parse().expect("Invalid id number ❌");
                todos.update_task_list(id);
            }
            5 => {
                println!("If you want to update the task description");
                println!("Enter Id");
                let mut id =  String::new();
                io::stdin().read_line(&mut id).expect("Invalid input");
                let id:usize = id.trim().parse().expect("Invalid Id ❌");
                println!("Enter your updated task");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Invalid Input");
                todos.update_desc_list(task, id);
            }
            6 =>{
                break;
            }
            _ => {
                println!("Invalid input ❌");
                continue;
            }
        }
    }
}
