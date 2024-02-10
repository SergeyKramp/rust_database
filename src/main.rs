use rust_database::database::Database;
use rust_database::storables::RAMStorage;
use rust_database::command::Command;
use rust_database::column::ColumnType;

fn main() {
    // 
    println!("RustDB \nWelcome to RustDB! \nThis is a simple database written in Rust. \nIt is a work in progress.");
    let storage_device = RAMStorage::new();
    let database = Database::new(storage_device);
    database.make_table("test".to_string());
    database.update_table("test", Command::AddColumn("test_column", ColumnType::Integer));
    database.update_table("test", Command::AddRow(vec![("test_column", "1")]));
}
