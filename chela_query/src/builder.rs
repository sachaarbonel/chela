use crate::{
    query::Expr,
    query::QueryStmt,
    query::SelectItem,
    query::SetExpr,
    query::{Ident, ObjectName, Select, TableFactor, TableWithJoins},
    values::Value,
};

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
