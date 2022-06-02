pub mod builder;
pub mod create;
pub mod display;
pub mod insert;
pub mod query;
pub mod statement;
pub mod values;

#[cfg(test)]
mod tests {
    use crate::{
        builder::QueryBuilder,
        insert::InsertStmt,
        query::QueryStmt,
        query::{Expr, Select},
        query::{Ident, ObjectName, SelectItem, TableWithJoins},
        query::{SetExpr, TableFactor},
        values::{Value, Values},
    };

    #[test]
    fn insert_test() {
        let i = InsertStmt {
            into: true,
            table_name: "test".to_string(),
            columns: vec![],
            source: QueryStmt {
                body: SetExpr::Values(Values(vec![vec![Expr::Value(Value::SingleQuotedString(
                    "test".to_string(),
                ))]])),
                order_by: None,
                limit: None,
            },
        };
        assert_eq!(
            i.to_string(),
            "INSERT INTO test VALUES ('test');".to_string()
        );
    }

    #[test]
    fn query_in_list_test() {
        let orders_query = find_orders_in_list();
        let orders_query_builder = QueryBuilder::new()
            .select()
            .from("orders".to_string())
            .where_("user_id".to_string())
            .in_list(vec![1, 2, 3, 4]);
        assert_eq!(orders_query_builder.build(), orders_query);
        assert_eq!(
            orders_query.to_string(),
            "SELECT * FROM orders WHERE user_id IN (1, 2, 3, 4)"
        )
    }
    #[test]
    fn query_test() {
        //SELECT * FROM users ORDER BY id LIMIT 1;
        let q = find_first_user();

        assert_eq!(
            q.to_string(),
            "SELECT * FROM users ORDER BY id LIMIT 1".to_string()
        );
    }

    fn find_orders_in_list() -> QueryStmt {
        QueryStmt {
            body: SetExpr::Select(Box::new(Select {
                projection: vec![SelectItem::Wildcard],
                from: vec![TableWithJoins {
                    relation: TableFactor::Table {
                        name: ObjectName(vec![Ident {
                            value: "orders".to_string(),
                        }]),
                    },
                }],
                selection: Some(Expr::InList {
                    expr: Box::new(Expr::Identifier(Ident {
                        value: "user_id".to_string(),
                    })),
                    list: vec![
                        Expr::Value(Value::Number("1".to_string(), false)),
                        Expr::Value(Value::Number("2".to_string(), false)),
                        Expr::Value(Value::Number("3".to_string(), false)),
                        Expr::Value(Value::Number("4".to_string(), false)),
                    ],
                    negated: false,
                }),
                group_by: None,
                sort_by: vec![],
                having: None,
            })),
            order_by: None,
            limit: None,
        }
    }

    fn find_first_user() -> QueryStmt {
        QueryStmt {
            body: SetExpr::Select(Box::new(Select {
                projection: vec![SelectItem::Wildcard],
                from: vec![TableWithJoins {
                    relation: TableFactor::Table {
                        name: ObjectName(vec![Ident {
                            value: "users".to_string(),
                        }]),
                    },
                }],
                selection: None,
                group_by: None,
                sort_by: vec![],
                having: None,
            })),
            order_by: Some("id".to_string()),
            limit: Some(1),
        }
    }

    #[test]
    fn select_builder_test() {
        let built_query = QueryBuilder::new()
            .select()
            .from("users".to_string())
            .order_by(Some("id".to_string()))
            .limit(Some(1))
            .build();
        assert_eq!(find_first_user(), built_query);
    }
}
