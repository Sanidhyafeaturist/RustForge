pub struct QueryBuilder {
    query: String,
}

impl QueryBuilder {
    pub fn new() -> Self {
        QueryBuilder {
            query: String::new(),
        }
    }

    pub fn select(mut self, table: &str) -> Self {
        self.query.push_str(&format!("SELECT * FROM {}", table));
        self
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        self.query.push_str(&format!(" WHERE {}", condition));
        self
    }

    pub fn build(self) -> String {
        self.query
    }
}
