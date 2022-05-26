use async_trait::async_trait;
use futures::future::join_all;
use tokio_postgres::Client;

use crate::{CreateStmt, Entity, Statement};

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

pub struct Migrations(pub Vec<Statement>);

impl Migrations {
    pub fn new(statements: &'static [Statement]) -> Migrations {
        Migrations(statements.to_vec())
    }

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
