use std::fmt::Display;

use crate::query::Expr;

#[derive(Debug, PartialEq)]
pub enum Value {
    SingleQuotedString(String),
    Number(String, bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::SingleQuotedString(s) => write!(f, "'{}'", s),
            Value::Number(s, negated) => {
                if *negated {
                    write!(f, "-{}", s)
                } else {
                    write!(f, "{}", s)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Values(pub Vec<Vec<Expr>>);

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VALUES ")?;
        for row in &self.0 {
            write!(f, "(")?;
            for (i, expr) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", expr)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}
