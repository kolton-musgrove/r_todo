use crate::models::todo::Todo;
use chrono::{DateTime, Local};
use rusqlite::{params, Connection, Result as SqlResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to database: {0}")]
    ConnectionError(#[from] rusqlite::Error),

    #[error("Failed to exeute database operation: {0}")]
    OperationError(String),
}

pub struct DatabaseHandler {
    conn: Connection,
}

impl DatabaseHandler {
    pub fn new(db_path: &str) -> Result<Self, DatabaseError> {
        let conn = Connection::open(db_path)?;
        let handler = DatabaseHandler { conn };
        handler.init_database()?;
        Ok(handler)
    }

    fn init_database(&self) -> Result<(), DatabaseError> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0,
                created_at DATETIME NOT NULL,
                modified_at DATETIME,
                completed_at DATETIME,
                deleted_at DATETIME
            )",
            [],
        )?;

        Ok(())
    }

    pub fn load_todos(&self) -> Result<Vec<Todo>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, text, completed, created_at, completed_at
             FROM todos
             WHERE deleted_at IS NULL
             ORDER BY created_at DESC
            ",
        )?;

        let todos = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                completed: row.get(2)?,
                created_at: row.get(3)?,
                completed_at: row.get::<_, Option<DateTime<Local>>>(4)?,
            })
        })?;

        todos
            .collect::<SqlResult<Vec<Todo>>>()
            .map_err(DatabaseError::from)
    }

    pub fn insert_todo(&mut self, todo: &Todo) -> Result<i64, DatabaseError> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "INSERT INTO todos (text, completed, created_at) VALUES (?1, ?2, ?3)",
            params![todo.text, todo.completed, todo.created_at],
        )?;

        let id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(id)
    }

    pub fn update_todo(&mut self, todo: &Todo) -> Result<(), DatabaseError> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "UPDATE todos
             SET text = ?1,
                 completed = ?2,
                 modified_at = ?3,
                 completed_at = ?4
             WHERE id = ?5 AND deleted_at IS NULL",
            params![
                todo.text,
                todo.completed,
                Local::now(),
                todo.completed_at,
                todo.id
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn delete_todo(&mut self, id: i64) -> Result<(), DatabaseError> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "UPDATE todos SET deleted_at = ?1 WHERE id = ?2",
            params![Local::now(), id],
        )?;

        tx.commit()?;
        Ok(())
    }
}
