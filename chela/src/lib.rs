pub mod migrator;
pub use chela_derive::*;
use chela_query::create::Column;
use chela_query::statement::Statement;
use futures::future::join_all;
use migrator::{Migrations, Migrator};
use std::any::Any;
use std::rc::Rc;
use std::{any::TypeId, collections::HashMap};

use async_trait::async_trait;
use tokio_postgres::Client;
#[async_trait]
pub trait QueryRunner {
    type Output;
    async fn list(self, client: &Client) -> Vec<Self::Output>;
}

pub trait Repository {
    fn entity(self) -> Entity;
    fn as_any(&self) -> &dyn Any;
}

pub trait ToEntity {
    fn to_entity() -> Entity;
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub table_name: String,
    pub struct_name: String, // the struct to be parse, like the User struct above.
    pub columns: Vec<Column>, // the struct's fields
    pub has_many: Vec<HasMany>,
}

#[derive(Debug, Clone)]
pub struct HasMany {
    pub foreign_key: String,
    pub struct_name: String,
    pub table_name: String,
}
#[derive(Clone)]
pub struct Schema {
    entities: Vec<Entity>,
}

impl Schema {
    pub fn new(entities: Vec<Entity>) -> Self {
        // let concrete_entities: Vec<Entity> = entities
        //     .into_iter()
        //     .map(|entity| entity.to_entity())
        //     .collect();

        Schema {
            entities, //: concrete_entities,
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
    pub fn new(entities: Vec<Entity>) -> Self {
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
