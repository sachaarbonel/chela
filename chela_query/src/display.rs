use core::fmt;
use std::fmt::Display;

use crate::{Query, SetExpr, Select, Expr, SelectItem};

impl Display for SelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectItem::Wildcard => write!(f, "*"),
        }
    }
}

impl Display for Query {
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
        }
    }
}

struct DisplaySeparated<'a, T>
where
    T: fmt::Display,
{
    slice: &'a [T],
    sep: &'static str,
}

impl<'a, T> fmt::Display for DisplaySeparated<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut delim = "";
        for t in self.slice {
            write!(f, "{}", delim)?;
            delim = self.sep;
            write!(f, "{}", t)?;
        }
        Ok(())
    }
}

fn display_separated<'a, T>(slice: &'a [T], sep: &'static str) -> DisplaySeparated<'a, T>
where
    T: fmt::Display,
{
    DisplaySeparated { slice, sep }
}

fn display_comma_separated<T>(slice: &[T]) -> DisplaySeparated<'_, T>
where
    T: fmt::Display,
{
    DisplaySeparated { slice, sep: ", " }
}