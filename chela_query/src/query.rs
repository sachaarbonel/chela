use std::fmt::Display;

use crate::{display::display_comma_separated, values::Value, values::Values};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct TableWithJoins {
    pub relation: TableFactor,
    // pub joins: Vec<Join>,
}

impl Display for TableWithJoins {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.relation).unwrap();
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableFactor {
    Table {
        name: ObjectName,
        // alias: Option<TableAlias>,
        // Arguments of a table-valued function, as supported by Postgres
        // and MSSQL. Note that deprecated MSSQL `FROM foo (NOLOCK)` syntax
        // will also be parsed as `args`.
        // args: Vec<FunctionArg>,
        // MSSQL-specific `WITH (...)` hints such as NOLOCK.
        // with_hints: Vec<Expr>,
    },
}

impl Display for TableFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TableFactor::Table { name, .. } => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectName(pub Vec<Ident>);

impl Display for ObjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", display_comma_separated(&self.0))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    /// The value of the identifier without quotes.
    pub value: String,
    // The starting quote if any. Valid quote characters are the single quote,
    // double quote, backtick, and opening square bracket.
    // pub quote_style: Option<char>,
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value).unwrap();
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Select {
    // pub distinct: bool,
    /// projection expressions
    pub projection: Vec<SelectItem>,
    /// FROM
    pub from: Vec<TableWithJoins>, //
    /// WHERE
    pub selection: Option<Expr>,
    /// GROUP BY
    pub group_by: Option<String>, //Vec<Expr>,
    /// SORT BY (Hive)
    pub sort_by: Vec<Expr>,
    /// HAVING
    pub having: Option<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Value(Value),
    Identifier(Ident),
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
            display_comma_separated(&self.from)
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
            Expr::Identifier(value) => write!(f, "{}", value),
        }
    }
}
