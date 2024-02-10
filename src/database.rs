///! Contains the data structure for a database instance.
use std::cell::RefCell;
use crate::command::Command;
use crate::storables::Storage;
use crate::table::Table;



///! Represents a database instance.
pub struct Database<'a, T: Storage<'a>> {
    storage: T,
    tables: RefCell<Vec<Table<'a>>>,
}

impl<'a, T: Storage<'a>> Database<'a, T> {
    pub fn new(storage: T) -> Database<'a, T> {
        Database {
            storage: storage,
            tables: RefCell::new(vec![]),
        }
    }
    /// Creates a new table in the database.
    /// 
    /// # Arguments
    /// 
    /// * `table_name` - The name of the table to be created.
    pub fn make_table(&self, table_name: String) {
        let table = Table::new(table_name);
        self.tables.borrow_mut().push(table);
    }
    /// Stores the data to the storage device.
    pub fn flush(&self) -> std::io::Result<()> {
        match self.storage.save(&self.tables.borrow()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    /// Loads the data from the storage device.
    pub fn load_data(&'a self) -> std::io::Result<()> {
        match self.storage.load() {
            Ok(tables) => {
                self.tables.replace(tables);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
    /// Updates a table in the database.
    /// 
    /// # Arguments
    /// 
    /// * `table_name` - The name of the table to be updated.
    /// * `command` - The command to be executed on the table.
    pub fn update_table(&self, table_name: &str, command: Command) {
        let mut binding = self.tables.borrow_mut();
        let table = binding.iter_mut().find(|t| t.name == table_name);
        match table {
            Some(t) => t.execute(command),
            None => println!("Table {table_name} not found."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storables::RAMStorage;

    #[test]
    fn make_table() {
        // Given: a database
        let ram_storage = RAMStorage::new();
        let database = Database::new(ram_storage);

        // When: we make a table
        database.make_table("test".to_string());

        // Then: the table is in the database
        assert_eq!(database.tables.borrow().len(), 1);
    }

    #[test]
    fn flush() {
        // Given: a database stored with a table
        let ram_storage = RAMStorage::new();
        let mut database = Database::new(ram_storage);
        database.tables = RefCell::new(vec![Table::new("test".to_string())]);

        // When: we flush the database
        let _ = database.flush();

        // Check the number of tables in the storage
        assert_eq!(database.storage.get_number_of_tables(), 1);
    }

    #[test]
    fn  load_data() {
        // Given: a storage device with a table
        let ram_storage = RAMStorage::new();
        ram_storage.save(&vec![Table::new("test".to_string())]).unwrap();
        let database = Database::new(ram_storage);

        // When: we load the data
        let _ = database.load_data();

        // Then: the table is in the database
        assert_eq!(database.tables.borrow().len(), 1);
    }
}
