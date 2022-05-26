mod display;

pub use chela_derive::*;
use futures::future::join_all;
use std::any::Any;
use std::rc::Rc;
use std::{any::TypeId, collections::HashMap};

use async_trait::async_trait;
use tokio_postgres::Client;
#[async_trait]
pub trait QueryRunner {
    type Output;
    async fn first(&self, client: &Client) -> Self::Output;
}

pub trait Repository: Any + Send + Sync {
    fn table_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}

pub trait ToEntity {
    fn to_entity(&self) -> Entity;
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub table_name: String,
    pub struct_name: String, // the struct to be parse, like the User struct above.
    pub columns: Vec<Column>, // the struct's fields
}

#[derive(Debug, PartialEq, Clone)]
pub struct Column {
    pub column_name: String,
    pub column_type: String,
}

#[derive(PartialEq, Clone)]
pub struct CreateStmt {
    table_name: String,
    columns: Vec<Column>,
}

trait Migrator {
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
#[derive(Clone)]
pub struct Schema {
    entities: Vec<Entity>,
}

#[derive(Clone)]
pub enum Statement {
    CreateStmt(CreateStmt),
}

pub struct Migrations(Vec<Statement>);

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

impl Schema {
    pub fn new(entities: Vec<Box<dyn ToEntity>>) -> Self {
        let concrete_entities: Vec<Entity> = entities
            .into_iter()
            .map(|entity| entity.to_entity())
            .collect();

        Schema {
            entities: concrete_entities,
        }
    }

    pub fn entities(self) -> Vec<Entity> {
        self.entities
    }
}

pub struct Chela {
    schema: Schema,
    migrations: Migrations,
    repositories: Vec<Rc<dyn Repository>>, //HashMap<TypeId, &'a dyn Repository<'a>>,
}

impl Chela {
    pub fn new(entities: Vec<Box<dyn ToEntity>>) -> Self {
        let repositories = Vec::new(); //HashMap::new();
        let schema = Schema::new(entities);

        let statements: Vec<Statement> = schema
            .clone()
            .entities
            .into_iter()
            .map(|entity| Statement::CreateStmt(entity.create_table()))
            .collect();
        let migrations = Migrations(statements);
        Chela {
            schema,
            repositories,
            migrations,
        }
    }

    pub fn schema(self) -> Schema {
        self.schema
    }

    pub fn migrations(self) -> Migrations {
        self.migrations
    }

    pub fn add_repo(&mut self, repo: Rc<dyn Repository>) {
        self.repositories.push(repo) //.insert(repo.type_id(), repo);
    }

    pub fn get_repo<R: 'static>(&self) -> Rc<(dyn Repository + 'static)> {
        let found = self
            .repositories
            .clone()
            .into_iter()
            .find(|repo| repo.as_any().type_id() == TypeId::of::<R>());

        found.unwrap()
    }
}
