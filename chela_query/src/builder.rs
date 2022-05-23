use crate::{SelectItem, Expr, Select, SetExpr, Query};



pub struct QueryBuilder {
    pub order_by: Option<String>,
    pub limit: Option<i64>,
    // pub distinct: bool,
    /// projection expressions
    pub projection: Vec<SelectItem>,
    /// FROM
    pub from: String, //Vec<TableWithJoins>,
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
            projection: Vec::new(),
            from: String::new(),
            selection: None,
            group_by: None,
            sort_by: Vec::new(),
            having: None,
        }
    }
    pub  fn select(mut self) -> QueryBuilder {
        //projection: Vec<SelectItem>
        self.projection = vec![SelectItem::Wildcard];
        self
    }

    pub fn from(mut self, from: String) -> QueryBuilder {
        self.from = from;
        self
    }

    pub fn r#where(mut self, selection: Option<Expr>) -> QueryBuilder {
        self.selection = selection;
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

    pub fn build(self) -> Query {
        Query {
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