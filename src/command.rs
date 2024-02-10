
use crate::column_types::ColumnType;

/// Represents a command that can be executed on a table.
pub enum Command {
    AddColumn(&'static str, ColumnType),
    //RemoveColumn(String),
    AddRow(Vec<(&'static str, &'static str)>),
    //RemoveRow(usize),
    //UpdateRow(usize, Vec<ColumnValue>),
}