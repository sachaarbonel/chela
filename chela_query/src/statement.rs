use std::fmt::{Display, Formatter};

use crate::{create::CreateStmt, insert::InsertStmt, query::QueryStmt};

#[derive(Debug, PartialEq)]
pub enum Statement {
    CreateStmt(CreateStmt),
    QueryStmt(QueryStmt),
    InsertStmt(InsertStmt),
    //TODO: update statement
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::CreateStmt(create_stmt) => write!(f, "{};", create_stmt),
            Statement::QueryStmt(_) => todo!(),
            Statement::InsertStmt(_) => todo!(),
        }
    }
}
