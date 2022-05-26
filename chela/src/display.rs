use std::fmt::{self, Display};

use crate::{Migrations, Column, CreateStmt, Statement};


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

impl Display for CreateStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CREATE TABLE {} ({})",
            self.table_name,
            display_comma_separated(&self.columns)
        )
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.column_name, self.column_type)
    }
}


impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::CreateStmt(create_stmt) => write!(f, "{};", create_stmt),
        }
    }
}


impl fmt::Display for Migrations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

