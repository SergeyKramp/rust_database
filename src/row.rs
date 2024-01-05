use std::collections::HashMap;
use uuid::Uuid;
use crate::column::{Column, ColumnValue};

#[derive(Clone)]
pub struct Row<'a> {
    id: Uuid,
    column_values: HashMap<&'a Column, ColumnValue>,
}

impl<'a> Row<'a> {
    pub fn new(id: Option<Uuid>, column_values: HashMap<&'a Column, ColumnValue>) -> Row {
        Row {
            id: id.unwrap_or(Uuid::new_v4()),
            column_values: column_values,
        }
    }
}