use rust_database::column_types::ColumnType;
use rust_database::command::Command;
use rust_database::database::Database;
use rust_database::storables::RAMStorage;

fn main() {
    // Create a RamStorage
    let storage = RAMStorage::new();

    // Create a Database
    let mut database = Database::new(storage);

    database.make_table("test".to_string());
    database.update_table(
        "test",
        Command::AddColumn("test column".to_string(), ColumnType::String),
    );
}
