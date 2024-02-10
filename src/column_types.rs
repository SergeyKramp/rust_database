/// Represents a potential type of a column.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ColumnType {
    Integer,
    Float,
    String,
}

/// Represents a potential value of a column.
#[derive(Clone, PartialEq, Debug)]
pub enum ColumnValue {
    IntValue(i32),
    FloatValue(f32),
    StringValue(String),
}