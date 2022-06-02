use async_trait::async_trait;
// use chela::Column;
// use chela::Entity;
// use chela::Schema;
use chela::{migrator::MigrationRunner, *};
use chela_query::builder::QueryBuilder;
use chela_query::create::Column;
use chela_query::create::ColumnType;
use itertools::Itertools;
use std::fmt::Display;
use std::{any::Any, rc::Rc}; //TODO: in query create an intermediate ColumnDef
                             // use chela_query::runner::QueryRunner;
use tokio_postgres::Client;

#[derive(ToEntity, PartialEq, Debug)]
struct User {
    id: i32,
    username: String,
    #[has_many(foreign_key = "user_id",table_name = "orders")]
    orders: Vec<Order>,
}

#[derive(PartialEq, Clone)]
struct UserOuter {
    id: i32,
    username: String,
}

#[derive(ToEntity, Clone, Copy, PartialEq, Debug)]
struct Order {
    id: i32,
    user_id: i32,
    price: f64,
}

fn main() {
    let user = User {
        id: 1,
        username: "origin".to_string(),
        orders: todo!(),
    };
    let order = Order {
        id: 1,
        user_id: 1,
        price: 100.0,
    };
    let repository = UserRepository::new();
    

    // let pointRepo = PointRepository::new();
    // let repos = vec![Box::new(pointRepo)];
    // let repositories = vec![Box::new(repository)];
    let mut chela = Chela::new(vec![User::to_entity(),Order::to_entity()]); //Schema::new(vec![Box::new(point)]);
    println!("{}", chela.migrations());
    // chela.add_repo(Rc::new(repository));
}

// async fn first_point(chela: Chela, pointRepository: ProductRepository, client: &Client) -> Product {
//     let migrations = chela.migrations();
//     let first_migration = migrations.run(client);
//     // pointRepository.first(client).await
// }

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

struct Ids(Vec<i32>);

impl Display for Ids {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //comma separated list of ids
        let ids = self
            .0
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>();

        write!(f, "{}", ids.join(","))
    }
}

#[async_trait]
impl QueryRunner for UserRepository
where
    User: ToEntity,
{
    type Output = User;
    async fn list(self, client: &Client) -> Vec<Self::Output> {
        let entity = self.entity();
        let parent_query = QueryBuilder::new()
            .select()
            .from(entity.table_name.to_string())
            // .order_by(Some("id".to_string()))
            // .limit(Some(1))
            .build();

        let parent_rows = client.query(&parent_query.to_string(), &[]).await.unwrap();
        let users_outer: Vec<UserOuter> = parent_rows
            .into_iter()
            .map(|row| UserOuter::from(row))
            .collect();
        let ids = users_outer.iter().map(|user| user.id).collect::<Vec<_>>();
        let parent_and_ids = users_outer
            .iter()
            .map(|user| (user, user.id))
            .collect::<Vec<_>>();

        let many_row0_query = format!(
            r#"SELECT * FROM {} WHERE {} IN ({});"#,
            entity.has_many[0].table_name.to_string(),
            entity.has_many[0].foreign_key.to_string(),
            Ids(ids).to_string()
        );
        // let many_row0_query = QueryBuilder::new()
        //     .select()
        //     .from(entity.has_many[0].table_name.to_string())
        //     .order_by(Some("id".to_string()))
        //     .limit(Some(1))
        //     .build();

        let many_row0 = client
            .query(&many_row0_query.to_string(), &[])
            .await
            .unwrap();
        let many_row0result: Vec<Order> =
            many_row0.into_iter().map(|row| Order::from(row)).collect();

        let lookup_many0result = many_row0result
            .into_iter()
            .into_group_map_by(|order| order.id);

        let result = parent_and_ids
            .into_iter()
            .map(|(user, id)| User {
                orders: lookup_many0result[&id].to_owned(),
                id: user.id,
                username: user.username.to_owned(),
            })
            .collect::<Vec<_>>();
        result
    }
}

impl From<tokio_postgres::row::Row> for UserOuter {
    fn from(row: tokio_postgres::row::Row) -> Self {
        UserOuter {
            id: row.get(0),
            username: row.get(1),
        }
    }
}

impl From<tokio_postgres::row::Row> for Order {
    fn from(row: tokio_postgres::row::Row) -> Self {
        Order {
            id: row.get(0),
            user_id: row.get(1),
            price: row.get(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Order, User, UserOuter};
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let expected_users = vec![User {
            id: 1,
            username: "origin".to_string(),
            orders: vec![Order {
                id: 1,
                user_id: 1,
                price: 200.0,
            }],
        }];
        let users_outer = vec![UserOuter {
            id: 1,
            username: "origin".to_string(),
        }];

        let orders = vec![Order {
            id: 1,
            user_id: 1,
            price: 200.0,
        }];
        let parent_and_ids = users_outer
            .iter()
            .map(|user| (user, user.id))
            .collect::<Vec<_>>();
        let lookup_orders = orders.into_iter().into_group_map_by(|order| order.id);

        let result = parent_and_ids
            .into_iter()
            .map(|(user, id)| User {
                orders: lookup_orders[&id].to_owned(),
                id: user.id,
                username: user.username.to_owned(),
            })
            .collect::<Vec<_>>();
        assert_eq!(result, expected_users);
    }
}
