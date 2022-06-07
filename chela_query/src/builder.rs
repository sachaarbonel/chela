use crate::{
    create::{ColumnDef, ColumnOption, ColumnOptionDef, CreateStmt, DataType, TableConstraint},
    insert::InsertStmt,
    query::Expr,
    query::QueryStmt,
    query::SelectItem,
    query::SetExpr,
    query::{Ident, ObjectName, Select, TableFactor, TableWithJoins},
    values::{Value, Values},
};

#[derive(Debug, Clone)]
pub struct CreateBuilder {
    // or_replace: bool,
    // temporary: bool,
    // external: bool,
    // global: Option<bool>,
    // if_not_exists: bool,
    /// Table name
    pub name: ObjectName,
    /// Optional schema
    pub columns: Vec<ColumnDef>,

    pub constraints: Vec<TableConstraint>,
}

///Short hand for CreateBuilder::new().name(name)
pub fn create_table(name: String, columns: Vec<ColumnDef>) -> CreateBuilder {
    CreateBuilder::new(columns).name(name)
}

pub fn select_table(name: String) -> QueryBuilder {
    QueryBuilder::new().select().from(name)
}

impl CreateBuilder {
    pub fn new(columns: Vec<ColumnDef>) -> Self {
        CreateBuilder {
            // or_replace: false,
            // temporary: false,
            // external: false,
            // global: None,
            // if_not_exists: false,
            name: ObjectName(vec![]),
            columns: columns,
            constraints: vec![],
        }
    }

    pub fn foreign_key_constraint(
        mut self,
        constraint_name: String,
        column_name: String,
        foreign_table_name: String,
        referred_column_name: String,
    ) -> CreateBuilder {
        self.constraints.push(TableConstraint::ForeignKey {
            name: Some(Ident {
                value: constraint_name,
            }),
            columns: vec![Ident { value: column_name }],
            foreign_table: ObjectName(vec![Ident {
                value: foreign_table_name,
            }]),
            referred_columns: vec![Ident {
                value: referred_column_name,
            }],
            // on_delete: None,
            // on_update: None,
        });
        self
    }
    pub fn name(mut self, name: String) -> CreateBuilder {
        self.name = ObjectName(vec![Ident { value: name }]);
        self
    }

    pub fn column(
        mut self,
        name: String,
        data_type: DataType,
        options_builder: Vec<ColumnOptionDef>,
    ) -> CreateBuilder {
        self.columns.push(ColumnDef {
            name: Ident { value: name },
            data_type: data_type,
            options: options_builder,
        });
        self
    }

    pub fn columns(self, tuple: Vec<(String, DataType, Vec<ColumnOptionDef>)>) -> CreateBuilder {
        tuple
            .into_iter()
            .for_each(|(name, data_type, options_builder)| {
                self.clone().column(name, data_type, options_builder);
            });
        self
    }

    pub fn build(self) -> CreateStmt {
        CreateStmt {
            name: self.name,
            columns: self.columns,
            constraints: self.constraints,
        }
    }
}

pub struct DataTypeBuilder {
    data_type: DataType,
}

impl DataTypeBuilder {
    pub fn new() -> Self {
        DataTypeBuilder {
            data_type: DataType::Custom(ObjectName(vec![])),
        }
    }

    pub fn varchar(mut self, length: Option<u64>) -> DataTypeBuilder {
        self.data_type = DataType::Varchar(length);
        self
    }

    pub fn int(mut self, length: Option<u64>) -> DataTypeBuilder {
        self.data_type = DataType::Int(length);
        self
    }

    pub fn serial(mut self) -> DataTypeBuilder {
        self.data_type = DataType::Custom(ObjectName(vec![Ident {
            value: "SERIAL".to_string(),
        }]));
        self
    }

    pub fn build(self) -> DataType {
        self.data_type
    }
}

///Short hand for DataTypeBuilder::new().varchar(length)
pub fn varchar(length: Option<u64>) -> DataType {
    DataTypeBuilder::new().varchar(length).build()
}
pub fn int(length: Option<u64>) -> DataType {
    DataTypeBuilder::new().int(length).build()
}

pub fn serial() -> DataType {
    DataTypeBuilder::new().serial().build()
}
pub struct ColumnOptionDefBuilder {
    options: Vec<ColumnOptionDef>,
}

impl ColumnOptionDefBuilder {
    pub fn new() -> Self {
        ColumnOptionDefBuilder { options: vec![] }
    }

    pub fn option(mut self, name: String, option: ColumnOption) -> ColumnOptionDefBuilder {
        self.options.push(ColumnOptionDef {
            name: Some(Ident { value: name }),
            option: option,
        });

        self
    }

    pub fn primary_key_unique(mut self) -> ColumnOptionDefBuilder {
        self.options.push(ColumnOptionDef {
            name: None,
            option: ColumnOption::Unique { is_primary: true },
        });

        self
    }

    pub fn not_null(mut self) -> ColumnOptionDefBuilder {
        self.options.push(ColumnOptionDef {
            name: None,
            option: ColumnOption::NotNull,
        });

        self
    }

    pub fn build(self) -> Vec<ColumnOptionDef> {
        self.options
    }
}

pub fn not_null() -> Vec<ColumnOptionDef> {
    ColumnOptionDefBuilder::new().not_null().build()
}
pub fn primary_key_unique() -> Vec<ColumnOptionDef> {
    ColumnOptionDefBuilder::new().primary_key_unique().build()
}
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    pub order_by: Option<String>,
    pub r#where: Box<Expr>,
    pub limit: Option<i64>,
    // pub distinct: bool,
    /// projection expressions
    pub projection: Vec<SelectItem>,
    /// FROM
    pub from: Vec<TableWithJoins>, //Vec<TableWithJoins>,
    /// WHERE
    pub selection: Option<Expr>,
    /// GROUP BY
    pub group_by: Option<String>, //Vec<Expr>,
    /// SORT BY (Hive)
    pub sort_by: Vec<Expr>,
    /// HAVING
    pub having: Option<Expr>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            order_by: None,
            limit: None,
            r#where: Box::new(Expr::Identifier(Ident {
                value: "".to_string(),
            })),
            projection: Vec::new(),
            from: Vec::new(),
            selection: None,
            group_by: None,
            sort_by: Vec::new(),
            having: None,
        }
    }
    pub fn select(mut self) -> QueryBuilder {
        //projection: Vec<SelectItem>
        self.projection = vec![SelectItem::Wildcard];
        self
    }

    pub fn from(mut self, from: String) -> QueryBuilder {
        self.from = vec![TableWithJoins {
            relation: TableFactor::Table {
                name: ObjectName(vec![Ident { value: from }]),
            },
        }];
        self
    }

    pub fn in_list(mut self, list_of_ids: Vec<i32>) -> QueryBuilder {
        let expr_value = list_of_ids
            .into_iter()
            .map(|value| Expr::Value(Value::Number(value.to_string(), false)))
            .collect::<Vec<Expr>>();
        self.selection = Some(Expr::InList {
            expr: self.r#where.clone(),
            list: expr_value,

            negated: false,
        });
        self
    }

    pub fn where_(mut self, id: String) -> QueryBuilder {
        self.r#where = Box::new(Expr::Identifier(Ident { value: id }));
        self
    }

    pub fn group_by(mut self, group_by: Option<String>) -> QueryBuilder {
        self.group_by = group_by;
        self
    }

    pub fn order_by(mut self, order_by: Option<String>) -> QueryBuilder {
        self.order_by = order_by;
        self
    }

    pub fn limit(mut self, limit: Option<i64>) -> QueryBuilder {
        self.limit = limit;
        self
    }

    pub fn sort_by(mut self) -> QueryBuilder {
        self.sort_by = vec![];
        self
    }

    pub fn build(self) -> QueryStmt {
        QueryStmt {
            body: SetExpr::Select(Box::new(Select {
                projection: self.projection,
                from: self.from,
                selection: self.selection,
                group_by: self.group_by,
                sort_by: self.sort_by,
                having: self.having,
            })),
            order_by: self.order_by,
            limit: self.limit,
        }
    }
}

pub struct InsertBuilder {
    /// Only for Sqlite
    // or: Option<SqliteOnConflict>,
    /// INTO - optional keyword
    pub into: bool,
    /// TABLE
    pub table_name: String,
    /// COLUMNS
    pub columns: Vec<Ident>,
    /// Overwrite (Hive)
    // pub overwrite: bool,
    // A SQL query that specifies what to insert
    pub source: QueryStmt,
    // partitioned insert (Hive)
    // partitioned: Option<Vec<Expr>>,
    // Columns defined after PARTITION
    // after_columns: Vec<Ident>,
    // whether the insert has the table keyword (Hive)
    // table: bool,
    // on: Option<OnInsert>,
}

impl InsertBuilder {
    pub fn new() -> Self {
        Self {
            into: true,
            table_name: "".to_string(),
            columns: Vec::new(),
            source: QueryStmt {
                body: SetExpr::Values(Values(vec![vec![]])),
                order_by: None,
                limit: None,
            },
        }
    }

    pub fn columns(mut self, columns: Vec<String>) -> InsertBuilder {
        columns
            .into_iter()
            .for_each(|column| self.columns.push(Ident { value: column }));
        self
    }
    pub fn column(mut self, column: String) -> InsertBuilder {
        self.columns.push(Ident { value: column });
        self
    }

    pub fn into(mut self, table: String) -> InsertBuilder {
        self.table_name = table;
        self
    }

    pub fn values(mut self, values: Vec<String>) -> InsertBuilder {
        let expr_value = values
            .into_iter()
            .map(|value| Expr::Value(Value::SingleQuotedString(value)))
            .collect::<Vec<Expr>>();
        self.source.body = SetExpr::Values(Values(vec![expr_value]));
        self
    }

    pub fn build(self) -> InsertStmt {
        InsertStmt {
            into: true,
            table_name: self.table_name,
            columns: self.columns,
            source: self.source,
        }
    }
}

pub fn insert_into(table: String) -> InsertBuilder {
    InsertBuilder::new().into(table)
}

#[cfg(test)]
mod tests {
    use crate::builder::{not_null, primary_key_unique};

    #[test]
    fn it_works() {
        let options = primary_key_unique();
        let not_null_option = not_null();
        let option_1: Vec<bool> = options
            .into_iter()
            .filter_map(|option_def| option_def.option.is_primary())
            .collect();
        println!("{:#?}", option_1);
        let option_2: Vec<bool> = not_null_option
            .into_iter()
            .filter_map(|option_def| option_def.option.is_primary())
            .collect();
        println!("{:#?}", option_2);
        let option_1_is_unique = option_1.into_iter().next().unwrap();
        let option_2_is_unique = option_2.into_iter().next().unwrap();

        assert_eq!(option_1_is_unique, true);
        assert_eq!(option_2_is_unique, false);
    }
}
