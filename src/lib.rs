use std::{fs::File, io::Write};

pub struct Todos {
    todos: Vec<String>,
}

impl Todos {
    pub fn from(contents: String) -> Todos {
        Todos { todos: contents.split("\n").map(String::from).collect() }
    }

    pub fn new() -> Todos {
        Todos {
            todos: Vec::new(),
        }
    }

    pub fn add_todo(&mut self, todo: String) {
        self.todos.push(todo);
    }

    pub fn get_todos(&self) -> &Vec<String> {
        &self.todos
    }

    pub fn delete_todo(&mut self, id: usize) {
        self.todos.remove(id);
    }

    pub fn update_todo(&mut self, id: usize, new_todo: String) {
        self.todos[id] = new_todo;
    }

    pub fn clear_todos(&mut self) {
        self.todos.clear();
    }

    pub fn write(&self, file: &mut File) -> Result<(), std::io::Error> {
        file.write_all(self.get_todos().join("\n").trim().as_bytes())
    }

}
