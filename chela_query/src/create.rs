use core::fmt::{self, Formatter};
use std::fmt::Display;

use crate::{
    display::{display_comma_separated, escape_single_quote_string},
    query::{Expr, Ident, ObjectName},
};

// #[derive(Debug, PartialEq, Clone)]
// pub struct CreateStmt {
//     pub table_name: String,
//     pub columns: Vec<Column>,
// }

#[derive(Debug, PartialEq, Clone)]
pub struct CreateStmt {
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
    // hive_distribution: HiveDistributionStyle,
    // hive_formats: Option<HiveFormat>,
    // table_properties: Vec<SqlOption>,
    // with_options: Vec<SqlOption>,
    // file_format: Option<FileFormat>,
    // location: Option<String>,
    // query: Option<Box<Query>>,
    // without_rowid: bool,
    // like: Option<ObjectName>,
    // engine: Option<String>,
    // default_charset: Option<String>,
    // collation: Option<String>,
    // on_commit: Option<OnCommit>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableConstraint {
    // `[ CONSTRAINT <name> ] { PRIMARY KEY | UNIQUE } (<columns>)`
    // Unique {
    //     name: Option<Ident>,
    //     columns: Vec<Ident>,
    //     /// Whether this is a `PRIMARY KEY` or just a `UNIQUE` constraint
    //     is_primary: bool,
    // },
    /// A referential integrity constraint (`[ CONSTRAINT <name> ] FOREIGN KEY (<columns>)
    /// REFERENCES <foreign_table> (<referred_columns>)
    /// { [ON DELETE <referential_action>] [ON UPDATE <referential_action>] |
    ///   [ON UPDATE <referential_action>] [ON DELETE <referential_action>]
    /// }`).
    ForeignKey {
        name: Option<Ident>,
        columns: Vec<Ident>,
        foreign_table: ObjectName,
        referred_columns: Vec<Ident>,
        // on_delete: Option<ReferentialAction>,
        // on_update: Option<ReferentialAction>,
    },
}


impl Display for TableConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TableConstraint::ForeignKey {
                name,
                columns,
                foreign_table,
                referred_columns,
            } => write!(
                f,
                ", CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({})",
                name.clone().unwrap().to_string(),
                display_comma_separated(&columns),
                foreign_table,
                display_comma_separated(&referred_columns),
            )?,
        }
        Ok(())
    }
}


impl Display for CreateStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "CREATE TABLE {} (", self.name)?;
        for (i, column) in self.columns.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", column)?;
        }
        write!(f, "{}", display_comma_separated(&self.constraints))?;
        write!(f, ")")?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnDef {
    pub name: Ident,
    pub data_type: DataType,
    // pub collation: Option<ObjectName>,
    pub options: Vec<ColumnOptionDef>,
}

impl Display for ColumnDef {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.name,
            self.data_type,
            display_comma_separated(&self.options)
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnOptionDef {
    pub name: Option<Ident>,
    pub option: ColumnOption,
}

impl Display for ColumnOptionDef {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.option)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ColumnOption {
    /// `NULL`
    Null,
    /// `NOT NULL`
    NotNull,
    /// `DEFAULT <restricted-expr>`
    Default(Expr),
    /// `{ PRIMARY KEY | UNIQUE }`
    Unique { is_primary: bool },
    /// A referential integrity constraint (`[FOREIGN KEY REFERENCES
    /// <foreign_table> (<referred_columns>)
    /// { [ON DELETE <referential_action>] [ON UPDATE <referential_action>] |
    ///   [ON UPDATE <referential_action>] [ON DELETE <referential_action>]
    /// }`).
    ForeignKey {
        foreign_table: ObjectName,
        referred_columns: Vec<Ident>,
        // on_delete: Option<ReferentialAction>,
        // on_update: Option<ReferentialAction>,
    },
    /// `CHECK (<expr>)`
    Check(Expr),
    // Dialect-specific options, such as:
    // - MySQL's `AUTO_INCREMENT` or SQLite's `AUTOINCREMENT`
    // - ...
    // DialectSpecific(Vec<Token>),
    // CharacterSet(ObjectName),
    // Comment(String),
}

impl Display for ColumnOption {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ColumnOption::Null => write!(f, "NULL"),
            ColumnOption::NotNull => write!(f, "NOT NULL"),
            ColumnOption::Default(expr) => write!(f, "DEFAULT {}", expr),
            ColumnOption::Unique { is_primary } => {
                if *is_primary {
                    write!(f, "PRIMARY KEY")
                } else {
                    write!(f, "UNIQUE")
                }
            }
            ColumnOption::ForeignKey {
                foreign_table,
                referred_columns,
                // on_delete,
                // on_update,
            } => write!(
                f,
                "FOREIGN KEY REFERENCES {} ({})",
                foreign_table,
                display_comma_separated(&referred_columns)
            ),
            ColumnOption::Check(expr) => write!(f, "CHECK ({})", expr),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    /// Fixed-length character type e.g. CHAR(10)
    Char(Option<u64>),
    /// Variable-length character type e.g. VARCHAR(10)
    Varchar(Option<u64>),
    /// Variable-length character type e.g. NVARCHAR(10)
    Nvarchar(Option<u64>),
    /// Uuid type
    Uuid,
    /// Large character object e.g. CLOB(1000)
    Clob(u64),
    /// Fixed-length binary type e.g. BINARY(10)
    Binary(u64),
    /// Variable-length binary type e.g. VARBINARY(10)
    Varbinary(u64),
    /// Large binary object e.g. BLOB(1000)
    Blob(u64),
    /// Decimal type with optional precision and scale e.g. DECIMAL(10,2)
    Decimal(Option<u64>, Option<u64>),
    /// Floating point with optional precision e.g. FLOAT(8)
    Float(Option<u64>),
    /// Tiny integer with optional display width e.g. TINYINT or TINYINT(3)
    TinyInt(Option<u64>),
    /// Unsigned tiny integer with optional display width e.g. TINYINT UNSIGNED or TINYINT(3) UNSIGNED
    UnsignedTinyInt(Option<u64>),
    /// Small integer with optional display width e.g. SMALLINT or SMALLINT(5)
    SmallInt(Option<u64>),
    /// Unsigned small integer with optional display width e.g. SMALLINT UNSIGNED or SMALLINT(5) UNSIGNED
    UnsignedSmallInt(Option<u64>),
    /// Integer with optional display width e.g. INT or INT(11)
    Int(Option<u64>),
    /// Unsigned integer with optional display width e.g. INT UNSIGNED or INT(11) UNSIGNED
    UnsignedInt(Option<u64>),
    /// Big integer with optional display width e.g. BIGINT or BIGINT(20)
    BigInt(Option<u64>),
    /// Unsigned big integer with optional display width e.g. BIGINT UNSIGNED or BIGINT(20) UNSIGNED
    UnsignedBigInt(Option<u64>),
    /// Floating point e.g. REAL
    Real,
    /// Double e.g. DOUBLE PRECISION
    Double,
    /// Boolean
    Boolean,
    /// Date
    Date,
    /// Time
    Time,
    /// Timestamp
    Timestamp,
    /// Interval
    Interval,
    /// Regclass used in postgresql serial
    Regclass,
    /// Text
    Text,
    /// String
    String,
    /// Bytea
    Bytea,
    /// Custom type such as enums
    Custom(ObjectName),
    /// Arrays
    Array(Box<DataType>),
    /// Enums
    Enum(Vec<String>),
    /// Set
    Set(Vec<String>),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Char(size) => format_type_with_optional_length(f, "CHAR", size, false),
            DataType::Varchar(size) => {
                format_type_with_optional_length(f, "VARCHAR", size, false) //CHARACTER VARYING
            }
            DataType::Nvarchar(size) => {
                format_type_with_optional_length(f, "NVARCHAR", size, false)
            }
            DataType::Uuid => write!(f, "UUID"),
            DataType::Clob(size) => write!(f, "CLOB({})", size),
            DataType::Binary(size) => write!(f, "BINARY({})", size),
            DataType::Varbinary(size) => write!(f, "VARBINARY({})", size),
            DataType::Blob(size) => write!(f, "BLOB({})", size),
            DataType::Decimal(precision, scale) => {
                if let Some(scale) = scale {
                    write!(f, "NUMERIC({},{})", precision.unwrap(), scale)
                } else {
                    format_type_with_optional_length(f, "NUMERIC", precision, false)
                }
            }
            DataType::Float(size) => format_type_with_optional_length(f, "FLOAT", size, false),
            DataType::TinyInt(zerofill) => {
                format_type_with_optional_length(f, "TINYINT", zerofill, false)
            }
            DataType::UnsignedTinyInt(zerofill) => {
                format_type_with_optional_length(f, "TINYINT", zerofill, true)
            }
            DataType::SmallInt(zerofill) => {
                format_type_with_optional_length(f, "SMALLINT", zerofill, false)
            }
            DataType::UnsignedSmallInt(zerofill) => {
                format_type_with_optional_length(f, "SMALLINT", zerofill, true)
            }
            DataType::Int(zerofill) => format_type_with_optional_length(f, "INT", zerofill, false),
            DataType::UnsignedInt(zerofill) => {
                format_type_with_optional_length(f, "INT", zerofill, true)
            }
            DataType::BigInt(zerofill) => {
                format_type_with_optional_length(f, "BIGINT", zerofill, false)
            }
            DataType::UnsignedBigInt(zerofill) => {
                format_type_with_optional_length(f, "BIGINT", zerofill, true)
            }
            DataType::Real => write!(f, "REAL"),
            DataType::Double => write!(f, "DOUBLE"),
            DataType::Boolean => write!(f, "BOOLEAN"),
            DataType::Date => write!(f, "DATE"),
            DataType::Time => write!(f, "TIME"),
            DataType::Timestamp => write!(f, "TIMESTAMP"),
            DataType::Interval => write!(f, "INTERVAL"),
            DataType::Regclass => write!(f, "REGCLASS"),
            DataType::Text => write!(f, "TEXT"),
            DataType::String => write!(f, "STRING"),
            DataType::Bytea => write!(f, "BYTEA"),
            DataType::Array(ty) => write!(f, "{}[]", ty),
            DataType::Custom(ty) => write!(f, "{}", ty),
            DataType::Enum(vals) => {
                write!(f, "ENUM(")?;
                for (i, v) in vals.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "'{}'", escape_single_quote_string(v))?;
                }
                write!(f, ")")
            }
            DataType::Set(vals) => {
                write!(f, "SET(")?;
                for (i, v) in vals.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "'{}'", escape_single_quote_string(v))?;
                }
                write!(f, ")")
            }
        }
    }
}

fn format_type_with_optional_length(
    f: &mut fmt::Formatter,
    sql_type: &'static str,
    len: &Option<u64>,
    unsigned: bool,
) -> fmt::Result {
    write!(f, "{}", sql_type)?;
    if let Some(len) = len {
        write!(f, "({})", len)?;
    }
    if unsigned {
        write!(f, " UNSIGNED")?;
    }
    Ok(())
}

// #[derive(Debug, PartialEq, Clone)]
// pub struct Column {
//     pub column_name: String,
//     pub column_type: ColumnType,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum ColumnType {
//     Integer,
//     Text,
//     Date,
//     Boolean,
//     VARCHAR(i64),
//     BigInt,
//     SmallInt,
//     Real,
//     TimestampWithTz,
//     TimestampNoTz,
//     UUID,
//     UserDefined,
//     DoublePrecision
// }

// impl From<String> for ColumnType {
//     fn from(s: String) -> Self {
//         match s.as_str() {
//             "i64" => ColumnType::Integer,
//             "f64"=> ColumnType::DoublePrecision,
//             "bool" => ColumnType::Boolean,
//             "String" => ColumnType::VARCHAR(150),
//             "NaiveDate" => ColumnType::Date,
//             "i32" => ColumnType::Integer,
//             "f32" => ColumnType::Real,
//             "i16" => ColumnType::SmallInt,
//             "DateTime<chrono::Local>" => ColumnType::TimestampWithTz,
//             "NaiveDateTime" => ColumnType::TimestampNoTz,
//             "Uuid" => ColumnType::UUID,
//             _ => panic!("not supported type: {}", s),
//         }
//     }
// }

// impl Display for CreateStmt {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "CREATE TABLE {} ({})",
//             self.table_name,
//             display_comma_separated(&self.columns)
//         )
//     }
// }

// impl Display for Column {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} {}", self.column_name, self.column_type)
//     }
// }

// impl Display for ColumnType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             ColumnType::Integer => write!(f, "INTEGER"),
//             ColumnType::Text => write!(f, "TEXT"),
//             ColumnType::Date => write!(f, "DATE"),
//             ColumnType::VARCHAR(n) => write!(f, "VARCHAR({})", n),
//             ColumnType::BigInt => write!(f, "BIGINT"),
//             ColumnType::SmallInt => write!(f, "SMALLINT"),
//             ColumnType::Real => write!(f, "REAL"),
//             ColumnType::TimestampWithTz => write!(f, "TIMESTAMP WITH TIME ZONE"),
//             ColumnType::TimestampNoTz => write!(f, "TIMESTAMP WITHOUT TIME ZONE"),
//             ColumnType::UUID => write!(f, "UUID"),
//             ColumnType::UserDefined => write!(f, "USER-DEFINED"),
//             ColumnType::Boolean => write!(f, "boolean"),
//             ColumnType::DoublePrecision => write!(f, "DOUBLE PRECISION"),
//         }
//     }
// }

impl From<String> for DataType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "i8" => DataType::Char(None),
            "i64" => DataType::BigInt(None),
            "f64" => DataType::Double,
            "bool" => DataType::Boolean,
            "String" => DataType::Varchar(Some(150)),
            "NaiveDate" => DataType::Date,
            "i32" => DataType::Int(None), //Serial
            "f32" => DataType::Real,
            "i16" => DataType::SmallInt(None),
            "DateTime<chrono::Local>" => DataType::Time,
            "NaiveDateTime" => DataType::Time,
            "&[u8]" => DataType::Bytea,
            "Uuid" => DataType::Uuid,
            //PgInternal
            //PgRange
            //PgMoney
            _ => panic!("not supported type: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::ColumnType;

    // #[test]
    // fn it_works() {
    //     let expected_ty = ColumnType::Integer;
    //     let ty = ColumnType::from("i64".to_string());
    //     assert_eq!(ty, expected_ty);
    // }
}
