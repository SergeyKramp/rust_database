pub struct Query {
    pub column_names: Vec<String>,
    pub table_name: String,
    pub conditions: Vec<Condition>,
}
