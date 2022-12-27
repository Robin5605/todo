use std::{fs::File, io::{self, Read}};

use clap::{Parser, Subcommand};
use crossterm::style::Stylize;
use todo::Todos;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Mode {
    New {
        todo: String,
    },

    List,
    Update {
        id: usize,
        new_todo: String,
    },
    Delete {
        id: usize,
    },
    Clear,
}


#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}
fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let contents = match File::open("todos.txt") {
        Ok(mut f) => {
            let mut c = String::new();
            f.read_to_string(&mut c)?;
            c
        }
        Err(_) => String::new(),
    };
    let mut todos = Todos::from(contents);

    match args.mode {
        Mode::New { todo } => todos.add_todo(todo),
        Mode::List => {
            for (i, todo) in todos.get_todos().iter().enumerate() {
                println!("{}: {}", format!("[{}]", i).cyan().bold(), todo);
            }
        },
        Mode::Delete { id } => todos.delete_todo(id),
        Mode::Update { id, new_todo } => todos.update_todo(id, new_todo),
        Mode::Clear => todos.clear_todos(),
    }

    let mut write_file = File::create("todos.txt")?;
    todos.write(&mut write_file)?;

    Ok(())
}
