pub use chela_derive::*;
pub trait ToEntity {
    fn to_entity(&self) -> Entity;
}

#[derive(Debug)]
pub struct Entity {
    pub table_name: String,
    pub struct_name: String, // the struct to be parse, like the User struct above.
    pub columns: Vec<Column>,  // the struct's fields
}

#[derive(Debug)]
pub struct Column {
    pub column_name: String,
    pub column_type: String,
}
