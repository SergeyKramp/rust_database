use std::vec;

use uuid::Uuid;
use crate::column::Column;
use crate::row::Row;

#[derive(Clone)]
pub struct Table<'a> {
    id: Uuid,
    name: String,
    columns: Vec<Column>,
    rows: Vec<Row<'a>>,
}

impl<'a> Table<'a> {
    pub fn new(name: String) -> Table<'a> {
        Table {
            id: Uuid::new_v4(),
            name: name,
            columns: vec![],
            rows: vec![],
        }
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn add_row(&mut self, row: Row<'a>) {
        self.rows.push(row);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::column::{ColumnType, ColumnValue};
    #[test]
    fn add_column() {
        let mut table = Table::new("test".to_string());
        let column = Column::new(None, "test".to_string(),ColumnType::Integer);
        table.add_column(column);
        assert_eq!(table.columns.len(), 1);
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
        assert_eq!(table.rows.len(), 1); 
    }
}