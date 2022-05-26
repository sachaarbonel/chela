use core::fmt;
use std::fmt::Display;
pub mod builder;
mod display;
#[derive(Debug, PartialEq)]
pub enum SelectItem {
    Wildcard,
}

#[derive(Debug, PartialEq)]
pub struct Query {
    // SELECT or UNION / EXCEPT / INTERSECT
    pub body: SetExpr,
    // ORDER BY
    pub order_by: Option<String>, //Vec<OrderByExpr>,
    /// `LIMIT { <N> | ALL }`
    pub limit: Option<i64>, //Option<Expr>,
                                  // `OFFSET <N> [ { ROW | ROWS } ]`
                                  // pub offset: Option<Offset>
}

#[derive(Debug, PartialEq)]
pub enum SetExpr {
    /// Restricted SELECT .. FROM .. HAVING (no ORDER BY or set operations)
    Select(Box<Select>),
}

#[derive(Debug, PartialEq)]
pub struct Select {
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

#[derive(Debug, PartialEq)]
pub enum Expr {
    InList {
        expr: Box<Expr>,
        list: Vec<Expr>,
        negated: bool,
    },
}

#[cfg(test)]
mod tests {
    use crate::{builder::QueryBuilder, Query, Select, SelectItem, SetExpr};

    #[test]
    fn query_test() {
        //SELECT * FROM users ORDER BY id LIMIT 1;
        let q = find_first_user();

        assert_eq!(
            q.to_string(),
            "SELECT * FROM users ORDER BY id LIMIT 1".to_string()
        );
    }

    fn find_first_user() -> Query {
        Query {
            body: SetExpr::Select(Box::new(Select {
                projection: vec![SelectItem::Wildcard],
                from: "users".to_string(),
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
