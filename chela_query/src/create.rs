use core::fmt::{self, Formatter};
use std::fmt::Display;

use crate::{display::display_comma_separated, statement::Statement};

#[derive(Debug, PartialEq, Clone)]
pub struct CreateStmt {
    pub table_name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Column {
    pub column_name: String,
    pub column_type: ColumnType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ColumnType {
    Integer,
    Text,
    Date,
    Boolean,
    VARCHAR(i64),
    BigInt,
    SmallInt,
    Real,
    TimestampWithTz,
    TimestampNoTz,
    UUID,
    UserDefined,
}

impl From<String> for ColumnType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "i64" => ColumnType::Integer,
            "bool" => ColumnType::Boolean,
            "String" => ColumnType::VARCHAR(150),
            "NaiveDate" => ColumnType::Date,
            "i32" => ColumnType::Integer,
            "f32" => ColumnType::Real,
            "i16" => ColumnType::SmallInt,
            "DateTime<chrono::Local>" => ColumnType::TimestampWithTz,
            "NaiveDateTime" => ColumnType::TimestampNoTz,
            "Uuid" => ColumnType::UUID,
            _ => panic!("not supported type: {}", s),
        }
    }
}

impl Display for CreateStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CREATE TABLE {} ({})",
            self.table_name,
            display_comma_separated(&self.columns)
        )
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.column_name, self.column_type)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::CreateStmt(create_stmt) => write!(f, "{};", create_stmt),
            Statement::QueryStmt(_) => todo!(),
            Statement::InsertStmt(_) => todo!(),
        }
    }
}

impl Display for ColumnType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ColumnType::Integer => write!(f, "INTEGER"),
            ColumnType::Text => write!(f, "TEXT"),
            ColumnType::Date => write!(f, "DATE"),
            ColumnType::VARCHAR(n) => write!(f, "VARCHAR({})", n),
            ColumnType::BigInt => write!(f, "BIGINT"),
            ColumnType::SmallInt => write!(f, "SMALLINT"),
            ColumnType::Real => write!(f, "REAL"),
            ColumnType::TimestampWithTz => write!(f, "TIMESTAMP WITH TIME ZONE"),
            ColumnType::TimestampNoTz => write!(f, "TIMESTAMP WITHOUT TIME ZONE"),
            ColumnType::UUID => write!(f, "UUID"),
            ColumnType::UserDefined => write!(f, "USER-DEFINED"),
            ColumnType::Boolean => write!(f, "boolean"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ColumnType;

    #[test]
    fn it_works() {
        let expected_ty = ColumnType::Integer;
        let ty = ColumnType::from("i64".to_string());
        assert_eq!(ty, expected_ty);
    }
}
