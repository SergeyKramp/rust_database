///! Contains the data structure for a database instance.
use std::cell::RefCell;
use crate::storables::Storage;
use crate::table::Table;



///! Represents a database instance.
pub struct Database<'a> {
    storage: Box<dyn Storage<'a>>,
    tables: RefCell<Vec<Table<'a>>>,
}

impl<'a> Database<'a> {
    fn new(storage: Box<dyn Storage<'a>>) -> Database<'a> {
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
    /// Get the number of tables in the database.
    pub fn get_number_of_tables(&self) -> usize {
        self.storage.get_number_of_tables()
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
        let new_tables: Result<Vec<Table<'a>>, std::io::Error> = self.storage.load();
        match new_tables {
            Ok(tables) => {
                self.tables.replace(tables);
                Ok(())
            },
            Err(e) => Err(e),
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
        let ram_storage = Box::new(RAMStorage::new());
        let mut database = Database::new(ram_storage);

        // When: we make a table
        database.make_table("test".to_string());

        // Then: the table is in the database
        assert_eq!(database.get_number_of_tables(), 1);
    }

    #[test]
    fn flush() {
        // Given: a database stored with a table
        let ram_storage: Box<RAMStorage<'_>> = Box::new(RAMStorage::new());
        let mut database = Database::new(ram_storage);
        database.tables = RefCell::new(vec![Table::new("test".to_string())]);

        // When: we flush the database
        let _ = database.flush();

        // Check the number of tables in the storage
        assert_eq!(database.storage.get_number_of_tables(), 1);
    }

    #[test]
    fn load_data() {
        // Given: a storage device with a table
        let ram_storage: Box<RAMStorage<'_>> = Box::new(RAMStorage::new());
        ram_storage.save(&vec![Table::new("test".to_string())]).unwrap();
        let mut database = Database::new(ram_storage);

        // When: we load the data
        database.load_data();

        // Then: the table is in the database
        assert_eq!(database.get_number_of_tables(), 1);
    }
}
