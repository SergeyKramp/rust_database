use crate::column_types::{ColumnType, ColumnValue};

type ColumnName = String;

/// Represents a command that can be executed on a table.
pub enum Command {
    AddColumn(ColumnName, ColumnType),
    RemoveColumn(ColumnName),
    AddRow(Vec<(ColumnName, ColumnValue)>),
    //RemoveRow(usize),
    //UpdateRow(usize, Vec<ColumnValue>),
}
