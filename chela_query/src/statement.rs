use std::fmt::Display;

use crate::{create::CreateStmt, insert::InsertStmt, query::QueryStmt};

#[derive(Debug, PartialEq)]
pub enum Statement {
    CreateStmt(CreateStmt),
    QueryStmt(QueryStmt),
    InsertStmt(InsertStmt),
    //TODO: update statement
}
