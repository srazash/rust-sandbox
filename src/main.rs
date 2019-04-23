/*
    COMMAND LINE TODO APPLICATION.
*/

// use env module form the standard library
use std::env;

struct TodoItem {
    name: String,
    completed: bool
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem{
            name: name,
            completed: false
        };
    }
}

fn main() {
    // get arguments from the cli
    let args: Vec<String> = env::args().collect();
    let cmd = args[1].clone();
    let todo_item = TodoItem::new("Pick up milk".to_string());
    let todo_item_2 = TodoItem::new("Work in Rust".to_string());
    let todo_list = vec![todo_item, todo_item_2];

        

    if cmd == "get" {
        for item in todo_list {
            println!("{} - Complete: {}", item.name, item.completed);
        }
    }

    // debug print, the # gives us "pretty print" - each element is per-line
    // println!("{:#?}", args);

    
}
