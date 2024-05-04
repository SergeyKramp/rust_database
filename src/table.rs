use std::collections::HashMap;
use std::vec;

use crate::column::Column;
use crate::command::Command;
use crate::row::Row;
use uuid::Uuid;

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
            name,
            columns: vec![],
            rows: vec![],
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::AddColumn(name, column_type) => {
                // Check the column doesn't already exist
                for column in &self.columns {
                    if column.name == name {
                        return Result::Err(format!("Column {} already exists", name).to_string());
                    }
                }
                let column = Column::new(None, name, column_type);
                self.columns.push(column);
                Result::Ok(())
            }
            Command::AddRow(values) => {
                // Check that all columns exist using the column name
                for (column_name, _) in &values {
                    let column_does_not_exist = self
                        .columns
                        .iter()
                        .any(|column| column.name == *column_name);
                    if column_does_not_exist {
                        return Result::Err(
                            format!("Column {} does not exist", column_name).to_string(),
                        );
                    }
                }

                // Create a HashMap and gather the values from the vector
                let mut column_values = HashMap::new();
                for (column_name, column_value) in values {
                    // User unwrap because we know the column exists
                    let id = self
                        .columns
                        .iter()
                        .find(|column| column.name == column_name)
                        .unwrap()
                        .id;
                    column_values.insert(id, column_value);
                }
                let new_row = Row::new(None, column_values);
                self.rows.push(new_row);
                Result::Ok(())
            }
            Command::RemoveColumn(column_name) => {
                // Check if the column exists
                let mut column_exists = false;
                for column in &self.columns {
                    if column.name == column_name {
                        column_exists = true;
                    }
                }
                if !column_exists {
                    Result::Err(format!("Column {} does not exist", column_name).to_string())
                } else {
                    self.columns.retain(|column| column.name != column_name);
                    Result::Ok(())
                }
            }
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
        let _ = table.execute(add_column_command);

        // Then: the table has one column
        assert_eq!(table.columns.len(), 1);
    }

    #[test]
    fn add_row() {
        // Given: a table and a column and a row
        let mut table = Table::new("test".to_string());
        let column = Column::new(None, "test".to_string(), ColumnType::Integer);
        let columns = &mut table.columns;
        columns.push(column.clone());

        // When: we row to the table
        let column_name = column.name;
        let column_value = ColumnValue::IntValue(5);
        let command = Command::AddRow(vec![(column_name, column_value)]);
        let _ = table.execute(command);

        // Then: the table has one row
        assert_eq!(table.rows.len(), 1);
    }
}
