use std::fmt::Display;

use async_trait::async_trait;
use chela_query::create::CreateStmt;
use futures::future::join_all;
use tokio_postgres::Client;

use crate::{Entity, Statement};

pub trait Migrator {
    fn create_table(self) -> CreateStmt;
}

#[async_trait]
pub trait MigrationRunner {
    async fn run(&self, client: &Client);
}

impl Migrator for Entity {
    fn create_table(self) -> CreateStmt {
        let mut create_stmt = CreateStmt {
            table_name: self.table_name.to_string(),
            columns: Vec::new(),
        };
        for column in self.columns.into_iter() {
            create_stmt.columns.push(column);
        }
        create_stmt
    }
}

#[derive(Debug, PartialEq)]
pub struct Migrations(pub Vec<Statement>);

impl Display for Migrations{
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
