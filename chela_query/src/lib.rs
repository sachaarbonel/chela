pub mod builder;
pub mod create;
pub mod display;
pub mod insert;
pub mod query;
pub mod statement;
pub mod values;

#[cfg(test)]
mod tests {
    use crate::builder::int;
    use crate::create::TableConstraint::ForeignKey;
    use crate::{
        builder::{
            create_table, not_null, primary_key_unique, select_table, serial, varchar,
            ColumnOptionDefBuilder, CreateBuilder, DataTypeBuilder, QueryBuilder,
        },
        create::{ColumnDef, ColumnOption, ColumnOptionDef, CreateStmt, DataType},
        insert::InsertStmt,
        query::QueryStmt,
        query::{Expr, Select},
        query::{Ident, ObjectName, SelectItem, TableWithJoins},
        query::{SetExpr, TableFactor},
        values::{Value, Values},
    };

    #[test]
    fn create_table_fkey_test() {
        let query = create_fkey_stmt();
        assert_eq!(
            query,
            create_table("article".to_string(), vec![])
                .column("id".to_string(), serial(), primary_key_unique())
                .column("author_id".to_string(), int(None), not_null())
                .foreign_key_constraint(
                    "fk_author".to_string(),
                    "author_id".to_string(),
                    "author".to_string(),
                    "id".to_string(),
                )
                .build()
        );
        assert_eq!(query.to_string(), "CREATE TABLE article (id SERIAL PRIMARY KEY, author_id INT NOT NULL, CONSTRAINT fk_author FOREIGN KEY (author_id) REFERENCES author (id))")
    }

    fn create_fkey_stmt() -> CreateStmt {
        CreateStmt {
            name: ObjectName(vec![Ident {
                value: "article".to_string(),
            }]),
            columns: vec![
                ColumnDef {
                    name: Ident {
                        value: "id".to_string(),
                    },
                    data_type: DataType::Custom(ObjectName(vec![Ident {
                        value: "SERIAL".to_string(),
                    }])),
                    options: vec![ColumnOptionDef {
                        name: None,
                        option: ColumnOption::Unique { is_primary: true },
                    }],
                },
                ColumnDef {
                    name: Ident {
                        value: "author_id".to_string(),
                    },
                    data_type: DataType::Int(None),
                    options: vec![ColumnOptionDef {
                        name: None,
                        option: ColumnOption::NotNull,
                    }],
                },
            ],
            constraints: vec![ForeignKey {
                name: Some(Ident {
                    value: "fk_author".to_string(),
                }),
                columns: vec![Ident {
                    value: "author_id".to_string(),
                }],
                foreign_table: ObjectName(vec![Ident {
                    value: "author".to_string(),
                }]),
                referred_columns: vec![Ident {
                    value: "id".to_string(),
                }],
                // on_delete: None,
                // on_update: None,
            }],
            // primary_key: Some(vec![Ident{value: "id".to_string()}]),
            // ..Default::default()
        }
    }

    #[test]
    fn create_table_test() {
        let query = create_stmt();
        let builder = create_table("alphabet".to_string(), vec![])
            .column("id".to_string(), serial(), primary_key_unique())
            .column("letter".to_string(), varchar(None), not_null());

        assert_eq!(builder.build(), query);

        assert_eq!(
            query.to_string(),
            "CREATE TABLE alphabet (id SERIAL PRIMARY KEY, letter VARCHAR NOT NULL)"
        );
    }

    fn create_stmt() -> CreateStmt {
        CreateStmt {
            name: ObjectName(vec![Ident {
                value: "alphabet".to_string(),
            }]),
            columns: vec![
                ColumnDef {
                    name: Ident {
                        value: "id".to_string(),
                    },
                    data_type: DataType::Custom(ObjectName(vec![Ident {
                        value: "SERIAL".to_string(),
                    }])),
                    options: vec![ColumnOptionDef {
                        name: None,
                        option: ColumnOption::Unique { is_primary: true },
                    }],
                },
                ColumnDef {
                    name: Ident {
                        value: "letter".to_string(),
                    },
                    data_type: DataType::Varchar(None),
                    options: vec![ColumnOptionDef {
                        name: None,
                        option: ColumnOption::NotNull,
                    }],
                },
            ],
            constraints: vec![],
            // primary_key: Some(vec![Ident{value: "id".to_string()}]),
            // ..Default::default()
        }
    }

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
        let orders_query_builder = select_table("orders".to_string())
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
        let built_query = select_table("users".to_string())
            .order_by(Some("id".to_string()))
            .limit(Some(1))
            .build();
        assert_eq!(find_first_user(), built_query);
    }
}
