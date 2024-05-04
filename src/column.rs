//! Contains the Column struct
use crate::column_types::{ColumnType, ColumnValue};
use uuid::Uuid;

/// The Column struct represents a column in a database table.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Column {
    pub id: Uuid,
    pub name: String,
    pub column_type: ColumnType,
}

impl Column {
    pub fn new(id: Option<Uuid>, name: String, column_type: ColumnType) -> Column {
        Column {
            id: id.unwrap_or(Uuid::new_v4()),
            name,
            column_type,
        }
    }
}
pub fn parse_column_value(column_type: &ColumnType, value: &str) -> Result<ColumnValue, String> {
    match column_type {
        ColumnType::Integer => match value.parse::<i32>() {
            Ok(value) => Ok(ColumnValue::IntValue(value)),
            Err(_) => Err("Invalid integer value.".to_string()),
        },
        ColumnType::Float => match value.parse::<f32>() {
            Ok(value) => Ok(ColumnValue::FloatValue(value)),
            Err(_) => Err("Invalid float value.".to_string()),
        },
        ColumnType::String => Ok(ColumnValue::StringValue(value.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_new() {
        let column = Column::new(None, "test".to_string(), ColumnType::Integer);
        assert_eq!(column.column_type, ColumnType::Integer);
    }

    #[test]
    fn test_parse_column_value_integer() {
        let value = "5";
        let column_type = ColumnType::Integer;
        let result = parse_column_value(&column_type, value);
        assert_eq!(result, Ok(ColumnValue::IntValue(5)));
    }

    #[test]
    fn test_parse_column_value_float_decimal() {
        let value = "5.5";
        let column_type = ColumnType::Float;
        let result = parse_column_value(&column_type, value);
        assert_eq!(result, Ok(ColumnValue::FloatValue(5.5)));
    }

    #[test]
    fn test_parse_column_value_float_integer() {
        let value = "5";
        let column_type = ColumnType::Float;
        let result = parse_column_value(&column_type, value);
        assert_eq!(result, Ok(ColumnValue::FloatValue(5.0)));
    }

    #[test]
    fn test_parse_column_value_string() {
        let value = "test";
        let column_type = ColumnType::String;
        let result = parse_column_value(&column_type, value);
        assert_eq!(result, Ok(ColumnValue::StringValue("test".to_string())));
    }
}
