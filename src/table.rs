use std::cell::RefCell;
use std::collections::HashMap;
use std::vec;

use uuid::Uuid;
use crate::column::{self, Column};
use crate::command::Command;
use crate::row::Row;

#[derive(Clone)]
pub struct Table<'a> {
    id: Uuid,
    pub name: String,
    columns: RefCell<Vec<Column>>,
    rows: RefCell<Vec<Row<'a>>>,
}

impl<'a> Table<'a> {
    pub fn new(name: String) -> Table<'a> {
        Table {
            id: Uuid::new_v4(),
            name: name,
            columns: RefCell::new(vec![]),
            rows: RefCell::new(vec![]),
        }
    }

    pub fn execute(&self, command: Command) {
        match command {
            Command::AddColumn(name, column_type) => {
                let column = Column::new(None, name.to_string(), column_type);
                self.add_column(column);
            },
            Command::AddRow(values) => {
                let mut column_values = HashMap::new();
                for (column_name, value) in values {
                    let column = self.columns.borrow().iter().find(|c| c.name == column_name);
                    match column {
                        Some(c) => {
                            match column::parse_column_value(&c.column_type, value) {
                                Ok(value) => { column_values.insert(c, value); },
                                Err(e) => println!("{}", e),
                            }
                        },
                        None => println!("Column {column_name} not found."),
                    }
                }
                let row = Row::new(None, column_values);
                self.add_row(row);
            },
        }
    }

    fn get_reference_to_column(&'a self, column_name: &str) -> Option<&Column> {
        self.columns.borrow().iter().find(|c| c.name == column_name)
    } 

    fn add_column(&self, column: Column) {
        self.columns.borrow_mut().push(column);
    }

    fn add_row(&mut self, row: Row<'a>) {
        self.rows.borrow_mut().push(row);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::column_types::{ColumnType, ColumnValue};
    #[test]
    fn add_column() {
        let mut table = Table::new("test".to_string());
        let column = Column::new(None, "test".to_string(),ColumnType::Integer);
        table.add_column(column);
        assert_eq!(table.columns.borrow().len(), 1);
    }

    #[test]
    fn add_row() {
        // Given: a table and a column and a row
        let mut table = Table::new("test".to_string());
        let column = Column::new(None, "test".to_string(), ColumnType::Integer);

        let mut column_values = HashMap::new();
        column_values.insert(&column, ColumnValue::IntValue(1));
        let row = Row::new(None, column_values);

        // When: we add the column and row to the table
        table.add_row(row);

        // Then: the table has one row
        assert_eq!(table.rows.borrow().len(), 1); 
    }
}