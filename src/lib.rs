use clap::{Parser, Subcommand};
use rusqlite::{params, Connection, Result};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { todo: Option<Vec<String>> },
    Done { todo: Option<Vec<String>> },
}

#[derive(Debug)]
struct TodoItem {
    id: i32,
    task: String,
    done: bool,
}

impl TodoItem {
    fn new(task: String) -> Self {
        TodoItem {
            id: 0,
            task,
            done: false,
        }
    }
}

pub fn parse_command(cli: Cli) {
    let conn = Connection::open("todo.db").expect("Failed to open database");

    initialize_database(&conn).expect("Failed to initialize database");

    match cli.command {
        Some(command) => {
            match command {
                Commands::Add { todo } => {
                    if let Some(tasks) = todo {
                        for task in tasks {
                            add_todo(&conn, &task).unwrap_or_else(|err| {
                                println!("Failed to add todo '{}': {}", task, err);
                            });
                        }
                    }
                }
                Commands::Done { todo } => {
                    if let Some(tasks) = todo {
                        for task in tasks {
                            mark_done(&conn, &task).expect("Failed to mark todo as done");
                        }
                    }
                }
            }
        }
        None => {
            let _ = print_items(&conn);
        }
    }
    let _ = print_items(&conn);
}

fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            task TEXT NOT NULL UNIQUE,
            done INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn add_todo(conn: &Connection, task: &str) -> Result<()> {
    let todo_item = TodoItem::new(task.to_string());
    conn.execute(
        "INSERT INTO todo (task, done) VALUES (?1, ?2)",
        params![todo_item.task, todo_item.done as i32],
    )?;
    Ok(())
}

fn mark_done(conn: &Connection, task: &str) -> Result<()> {
    conn.execute(
        "UPDATE todo SET done = 1 WHERE task = ?1",
        params![task],
    )?;
    Ok(())
}

fn print_items(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, task, done FROM todo")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            task: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    for todo in todo_iter {
        let todo = todo?;
        if todo.done {
            println!("{}: \x1B[9m{}\x1B[0m", todo.id, todo.task)
        } else {
            println!("{}: {}", todo.id, todo.task)
        }
    }
    Ok(())
}
