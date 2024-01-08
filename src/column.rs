///! Contains the Column struct and its associated types.
use uuid::Uuid;

/// The Column struct represents a column in a database table.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Column {
    id: Uuid,
    name: String,
    column_type: ColumnType,
}

impl Column {
    pub fn new(id: Option<Uuid>, name: String, column_type: ColumnType) -> Column {
        Column {
            id: id.unwrap_or(Uuid::new_v4()),
            name: name,
            column_type: column_type,
        }
    }
}

/// Represents the type of a column.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ColumnType {
    Integer,
    Float,
    String,
}

/// Represents a potential value of a column.
#[derive(Clone)]
pub enum ColumnValue {
    IntValue(i32),
    FloatValue(f32),
    StringValue(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_new() {
        let column = Column::new(None, "test".to_string(), ColumnType::Integer);
        assert_eq!(column.column_type, ColumnType::Integer);
    }
}
