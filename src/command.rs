use crate::column_types::{ColumnType, ColumnValue};
use uuid::Uuid;

/// Represents a command that can be executed on a table.
pub enum Command {
    AddColumn(String, ColumnType),
    RemoveColumn(String),
    AddRow(Vec<(String, ColumnValue)>),
    //RemoveRow(usize),
    //UpdateRow(usize, Vec<ColumnValue>),
}
