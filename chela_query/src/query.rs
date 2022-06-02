use std::fmt::Display;

use crate::{display::display_comma_separated, values::Value, values::Values};

#[derive(Debug, PartialEq)]
pub enum SelectItem {
    Wildcard,
}

#[derive(Debug, PartialEq)]
pub struct QueryStmt {
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
    Values(Values),
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
    Value(Value),
    InList {
        expr: Box<Expr>,
        list: Vec<Expr>,
        negated: bool,
    },
}

impl Display for SelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectItem::Wildcard => write!(f, "*"),
        }
    }
}

impl Display for QueryStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)?;
        if let Some(order_by) = &self.order_by {
            write!(f, " ORDER BY {}", order_by)?;
        }
        if let Some(limit) = &self.limit {
            write!(f, " LIMIT {}", limit)?;
        }
        Ok(())
    }
}

impl Display for SetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetExpr::Select(select) => write!(f, "{}", select),
            SetExpr::Values(values) => write!(f, "{}", values),
        }
    }
}

impl Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SELECT {} FROM {}",
            display_comma_separated(&self.projection),
            self.from
        )?;
        if let Some(selection) = &self.selection {
            write!(f, " WHERE {}", selection)?;
        }
        if let Some(group_by) = &self.group_by {
            write!(f, " GROUP BY {}", group_by)?;
        }
        if let Some(having) = &self.having {
            write!(f, " HAVING {}", having)?;
        }
        Ok(())
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::InList {
                expr,
                list,
                negated,
            } => {
                if *negated {
                    write!(f, "NOT ")?;
                }
                write!(f, "{} IN ({})", expr, display_comma_separated(list))
            }
            Expr::Value(value) => write!(f, "{}", value),
        }
    }
}
