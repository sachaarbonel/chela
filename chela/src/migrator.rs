use std::fmt::Display;

use async_trait::async_trait;
use chela_query::{
    builder::create_table,
    create::{ColumnDef, CreateStmt},
    query::Ident,
};
use futures::future::join_all;
use tokio_postgres::Client;

use crate::{Column, Entity, Statement};

pub trait Migrator {
    fn create_table(self) -> CreateStmt;
}

#[async_trait]
pub trait MigrationRunner {
    async fn run(&self, client: &Client);
}

impl From<Column> for ColumnDef {
    fn from(column: Column) -> Self {
        ColumnDef {
            name: Ident { value: column.name },
            data_type: column.data_type,
            options: column.options,
        }
    }
}

impl Migrator for Entity {
    fn create_table(self) -> CreateStmt {
        let columns = self
            .columns
            .clone()
            .into_iter()
            .map(|c| ColumnDef::from(c))
            .collect::<Vec<ColumnDef>>();
        let stmt = create_table(self.table_name.to_string(), columns);
        if !self.belongs_to.clone().is_empty() {
            stmt.foreign_key_constraint(
                self.belongs_to[0].constraint_name.to_string(),
                self.belongs_to[0].column_name.to_string(),
                self.belongs_to[0].table_name.to_string(),
                self.belongs_to[0].foreign_key.to_string(),
            )
            .build()
        } else {
            stmt.build()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Migrations(pub Vec<Statement>);

impl Display for Migrations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.0.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}
impl Migrations {
    pub fn add_migration(mut self, migration: Statement) {
        self.0.push(migration);
    }
}

#[async_trait]
impl MigrationRunner for Statement {
    async fn run(&self, client: &Client) {
        let statement = self.to_string();
        let row = client.execute(&statement.to_string(), &[]).await.unwrap();
    }
}
#[async_trait]
impl MigrationRunner for Migrations {
    async fn run(&self, client: &Client) {
        join_all(self.0.iter().map(|statement| statement.run(&client))).await;
    }
}
