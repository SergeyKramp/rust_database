///! Contains the Row struct and it's methods
use std::collections::HashMap;
use uuid::Uuid;
use crate::column_types::ColumnValue;

/// Represents a row in the database and it's metadata.
#[derive(Clone)]
pub struct Row {
    id: Uuid,
    column_values: HashMap<Uuid, ColumnValue>,
}

impl Row {
    pub fn new(id: Option<Uuid>, column_values: HashMap<Uuid, ColumnValue>) -> Row {
        Row {
            id: id.unwrap_or(Uuid::new_v4()),
            column_values: column_values,
        }
    }
}