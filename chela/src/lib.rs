use core::fmt;
use std::fmt::Display;
mod display;

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

pub struct Statements(Vec<Statement>);

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
