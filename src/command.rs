use uuid::Uuid;
use crate::column_types::{ColumnType, ColumnValue};

/// Represents a command that can be executed on a table.
pub enum Command {
    AddColumn(String, ColumnType),
    //RemoveColumn(String),
    AddRow(Vec<(Uuid, ColumnValue)>),
    //RemoveRow(usize),
    //UpdateRow(usize, Vec<ColumnValue>),
}