use std::collections::HashMap;
use std::vec;

use uuid::Uuid;
use crate::column::Column;
use crate::command::Command;
use crate::row::Row;

/// Represents a table in a database and it's metadata.
#[derive(Clone)]
pub struct Table {
    id: Uuid,
    pub name: String,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            id: Uuid::new_v4(),
            name: name,
            columns: vec![],
            rows: vec![],
        }
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::AddColumn(name, column_type) => {
                let column = Column::new(None, name, column_type);
                self.columns.push(column);

            },
            Command::AddRow(values) => {
                // Create a HashMap and gather the values from the vector
                let mut column_values = HashMap::new();
                for (id, column_value) in values {
                    column_values.insert(id, column_value);
                }
                let new_row = Row::new(None, column_values);
                self.rows.push(new_row);
            },
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column_types::{ColumnType, ColumnValue};
    #[test]
    fn add_column() {
        // Given: a database table
        let mut table = Table::new("test".to_string());

        // When: we add a column to the table
        let add_column_command = Command::AddColumn("test".to_string(), ColumnType::Integer);
        table.execute(add_column_command);
        
        // Then: the table has one column
        assert_eq!(table.columns.len(), 1);
    }

    #[test]
    fn add_row() {
        // Given: a table and a column and a row
        let mut table = Table::new("test".to_string());
        let column = Column::new(None, "test".to_string(), ColumnType::Integer);
        let mut columns = &mut table.columns;
        columns.push(column.clone());

        // When: we row to the table
        let id = column.id;
        let column_value = ColumnValue::IntValue(5);
        let command = Command::AddRow(vec![(id, column_value)]);
        table.execute(command);

        // Then: the table has one row
        assert_eq!(table.rows.len(), 1); 
    }
}