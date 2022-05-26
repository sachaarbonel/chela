use async_trait::async_trait;
// use chela::Column;
// use chela::Entity;
// use chela::Schema;
use chela::{*, migrator::MigrationRunner};
use chela_query::builder::QueryBuilder;
use std::{any::Any, rc::Rc};
// use chela_query::runner::QueryRunner;
use tokio_postgres::Client;

#[derive(ToEntity)]
struct Point {
    name: String,
    x: i32,
    y: i32,
}

fn main() {
    let point = Point {
        name: "origin".to_string(),
        x: 2,
        y: 3,
    };
    let repository = PointRepository::new();

    // let pointRepo = PointRepository::new();
    // let repos = vec![Box::new(pointRepo)];
    // let repositories = vec![Box::new(repository)];
    let mut chela = Chela::new(vec![Box::new(point)]); //Schema::new(vec![Box::new(point)]);
    chela.add_repo(Rc::new(repository));
}

async fn first_point(chela: Chela, pointRepository: PointRepository, client: &Client) -> Point {
    let migrations = chela.migrations();
    let first_migration = migrations.run(client);
    pointRepository.first(client).await
}

// impl Point {
//     pub fn repo(chela: Chela) -> &'static PointRepository {
//         // -> &'static PointRepository
//         let repo_trait = chela.get_repo::<PointRepository>();
//         let repo: &PointRepository = repo_trait
//             .clone()
//             .as_any()
//             .downcast_ref::<PointRepository>()
//             .unwrap();
//         repo
//         // repo
//     }
// }
