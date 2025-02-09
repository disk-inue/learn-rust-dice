use std::collections::HashMap;
use std::io::{stdout, Write};

use crate::common::input;

#[derive(Clone, Debug)]
enum Status {
    Todo,
    Done,
}

#[derive(Clone, Debug)]
struct Todo {
    id: i32,
    title: String,
    status: Status,
}

impl Todo {
    fn new(title: String) -> Todo {
        Todo {
            id: 0,
            title,
            status: Status::Todo,
        }
    }
    fn done(&self) -> Todo {
        Todo {
            id: self.id,
            title: self.title.clone(),
            status: Status::Done,
        }
    }
    fn update(&self, title: String) -> Todo {
        Todo {
            id: self.id,
            title,
            status: self.status.clone(),
        }
    }
}

pub fn exec() {
    let mut todo_map: HashMap<u32, Todo> = HashMap::new();

    println!("start todo");
    loop {
        println!("select menu : 1. add, 2. edit, 3. done, 4. delete, 5. list");

        let select_number: u8 = match input("> ").parse() {
            Ok(num) => num,
            Err(_) => {
                println!("end todo");
                return;
            }
        };

        match select_number {
            1 => {
                let todo_map = add(&mut todo_map, input("title > "));
                show(&todo_map);
                stdout().flush().unwrap();
            }
            2 => {
                show(&todo_map);
                let target_number: u32 = match input("number > ").parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("end todo");
                        return;
                    }
                };
                let todo_map = edit(&mut todo_map, target_number);
                show(&todo_map);
            }
            3 => {
                show(&todo_map);
                let target_number: u32 = match input("number > ").parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("end todo");
                        return;
                    }
                };
                let todo_map = done(&mut todo_map, target_number);
                show(&todo_map);
            }
            4 => {
                show(&todo_map);
                let target_number: u32 = match input("number > ").parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("end todo");
                        return;
                    }
                };
                let todo_map = delete(&mut todo_map, target_number);
                show(&todo_map);
            }
            5 => {
                show(&todo_map);
                println!("{:?}", todo_map);
            }
            _ => {
                println!("end todo");
                return;
            }
        }
    }
}

fn add(todo_map: &mut HashMap<u32, Todo>, title: String) -> HashMap<u32, Todo> {
    let new_todo = Todo::new(title);
    let id = match u32::try_from(todo_map.len()) {
        Ok(num) => num,
        Err(_) => return todo_map.clone(),
    };
    todo_map.insert(id, new_todo);
    todo_map.clone()
}

fn edit(todo_map: &mut HashMap<u32, Todo>, number: u32) -> HashMap<u32, Todo> {
    let binding = todo_map.get(&number);
    let target_todo = match &binding {
        Some(todo) => todo,
        None => return todo_map.clone(),
    };
    let updated_todo = &target_todo.update(input("new title > "));
    todo_map.insert(number, updated_todo.clone());
    todo_map.clone()
}

fn done(todo_map: &mut HashMap<u32, Todo>, number: u32) -> HashMap<u32, Todo> {
    let binding = todo_map.get(&number);
    let target_todo = match &binding {
        Some(todo) => todo,
        None => return todo_map.clone(),
    };
    let done_todo = &target_todo.done();
    todo_map.insert(number, done_todo.clone());
    todo_map.clone()
}

fn delete(todo_map: &mut HashMap<u32, Todo>, number: u32) -> HashMap<u32, Todo> {
    todo_map.remove(&number);
    todo_map.clone()
}

fn show(todo_map: &HashMap<u32, Todo>) {
    println!("{}: {}: {}", "number", "title", "status");
    for (key, value) in todo_map {
        println!("{}: {}: {:?}", key, value.title, value.status);
    }
}
