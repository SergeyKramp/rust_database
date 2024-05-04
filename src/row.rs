//! Contains the Row struct and it's methods
use crate::column_types::ColumnValue;
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a row in the database and it's metadata.
#[derive(Clone, Debug)]
pub struct Row {
    id: Uuid,
    column_values: HashMap<Uuid, ColumnValue>,
}

impl Row {
    pub fn new(id: Option<Uuid>, column_values: HashMap<Uuid, ColumnValue>) -> Row {
        Row {
            id: id.unwrap_or(Uuid::new_v4()),
            column_values,
        }
    }
}
