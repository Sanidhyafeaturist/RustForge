use rusqlite::{Connection, Result};

pub struct ORM {
    connection: Connection,
}

impl ORM {
    pub fn new(database_url: &str) -> Result<Self> {
        let connection = Connection::open(database_url)?;
        Ok(ORM { connection })
    }

    pub fn execute(&self, sql: &str) -> Result<usize> {
        self.connection.execute(sql, []) // Execute SQL command
    }
}
