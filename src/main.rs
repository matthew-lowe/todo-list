use std::{env, fs::{File, OpenOptions}, io::{Error, Read, Write}, path::Path};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Todo {
    text: String,
    tag: String,
}

impl Todo {
    fn new(text: String, tag: String) -> Self {
        Self {
            text, tag
        }
    }
}

type Todos = Vec<Todo>;

// Get a file from a path, creating it if it doesn't exist, and returning Err(why) if there's a problem
fn get_file(path: &Path) -> Result<File, Error> {
    OpenOptions::new().read(true).write(true).open(path)
}

// Read the data in a file and parse the JSON
fn parse_file(file: &mut File) -> Result<Todos, Error> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(serde_json::from_str(&buffer[..])?)
}

// Save todos into the todo list file
fn save_todos(file: &mut File, todos: &Todos) -> Result<(), Error> {
    let data = serde_json::to_string(todos)?; 
    file.write_all(data.as_bytes())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new("todos.json");
    let display = path.display();

    match &args[1][..] {
        "list" => println!("List"),
        _ => println!("Unrecognised command!"),
    }

    // Read the file into the buffer
    let mut file = get_file(path).expect(&format!("Error opening file: {}", display)[..]);

    let mut todo_list: Todos = parse_file(&mut file).unwrap();

    println!("Before:");
    for todo in &todo_list {
        println!("Todo: {}, Tag: {}", todo.text, todo.tag);
    };

    let new_todo = Todo::new(String::from("Todo5"), String::from("Todo5 Tag"));
    todo_list.append(&mut vec![new_todo]);
    save_todos(&mut file, &todo_list).unwrap();

    println!("After:");
    for todo in &todo_list {
        println!("Todo: {}, Tag: {}", todo.text, todo.tag);
    };
}
