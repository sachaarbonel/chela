use core::fmt;
use std::fmt::Display;

pub use chela_derive::*;
pub trait ToEntity {
    fn to_entity(&self) -> Entity;
}

#[derive(Debug)]
pub struct Entity {
    pub table_name: String,
    pub struct_name: String, // the struct to be parse, like the User struct above.
    pub columns: Vec<Column>, // the struct's fields
}

#[derive(Debug, PartialEq)]
pub struct Column {
    pub column_name: String,
    pub column_type: String,
}

#[derive(PartialEq)]
pub struct CreateStmt {
    table_name: String,
    columns: Vec<Column>,
}

struct DisplaySeparated<'a, T>
where
    T: fmt::Display,
{
    slice: &'a [T],
    sep: &'static str,
}

impl<'a, T> fmt::Display for DisplaySeparated<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut delim = "";
        for t in self.slice {
            write!(f, "{}", delim)?;
            delim = self.sep;
            write!(f, "{}", t)?;
        }
        Ok(())
    }
}

fn display_separated<'a, T>(slice: &'a [T], sep: &'static str) -> DisplaySeparated<'a, T>
where
    T: fmt::Display,
{
    DisplaySeparated { slice, sep }
}

fn display_comma_separated<T>(slice: &[T]) -> DisplaySeparated<'_, T>
where
    T: fmt::Display,
{
    DisplaySeparated { slice, sep: ", " }
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.column_name, self.column_type)
    }
}

trait Migrator {
    fn create_table(self) -> CreateStmt;
}

impl Migrator for Entity {
    fn create_table(self) -> CreateStmt {
        let mut create_stmt = CreateStmt {
            table_name: self.table_name,
            columns: Vec::new(),
        };
        for column in self.columns.into_iter() {
            create_stmt.columns.push(column);
        }
        create_stmt
    }
}

pub struct Schema {
    pub entities: Vec<Box<dyn ToEntity>>,
}

pub enum Statement {
    CreateStmt(CreateStmt),
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::CreateStmt(create_stmt) => write!(f, "{};", create_stmt),
        }
    }
}

pub struct Statements(Vec<Statement>);

impl fmt::Display for Statements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

impl Schema {
    pub fn new(entities: Vec<Box<dyn ToEntity>>) -> Self {
        Schema { entities }
    }
    pub fn run(&self) -> Statements {
        let mut result = Vec::new();
        for component in self.entities.iter() {
            result.push(Statement::CreateStmt(component.to_entity().create_table()));
        }
        Statements(result)
    }
}
