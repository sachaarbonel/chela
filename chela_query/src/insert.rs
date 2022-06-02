use std::fmt::Display;

use crate::{create::Column, display::display_comma_separated, query::QueryStmt};

#[derive(Debug, PartialEq)]
pub struct InsertStmt {
    /// Only for Sqlite
    // or: Option<SqliteOnConflict>,
    /// INTO - optional keyword
    pub into: bool,
    /// TABLE
    pub table_name: String,
    /// COLUMNS
    pub columns: Vec<Column>, //Vec<Ident>,
    /// Overwrite (Hive)
    // pub overwrite: bool,
    // A SQL query that specifies what to insert
    pub source: QueryStmt,
    // partitioned insert (Hive)
    // partitioned: Option<Vec<Expr>>,
    // Columns defined after PARTITION
    // after_columns: Vec<Ident>,
    // whether the insert has the table keyword (Hive)
    // table: bool,
    // on: Option<OnInsert>,
}

impl Display for InsertStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "INSERT INTO {} ({}) {}",
            self.table_name,
            display_comma_separated(&self.columns),
            self.source
        )
    }
}
