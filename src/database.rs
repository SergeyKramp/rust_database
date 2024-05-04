///! Contains the data structure for a database instance.
use crate::command::Command;
use crate::storables::Storage;
use crate::table::Table;

///! Represents a database instance.
pub struct Database<T: Storage> {
    storage: T,
    tables: Vec<Table>,
}

impl<T: Storage> Database<T> {
    pub fn new(storage: T) -> Database<T> {
        Database {
            storage: storage,
            tables: vec![],
        }
    }
    /// Creates a new table in the database.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to be created.
    pub fn make_table(&mut self, table_name: String) {
        let table = Table::new(table_name);
        self.tables.push(table);
    }
    /// Stores the data to the storage device.
    pub fn flush(&mut self) -> std::io::Result<()> {
        match self.storage.save(&self.tables) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    /// Loads the data from the storage device.
    pub fn load_data(&mut self) -> std::io::Result<()> {
        match self.storage.load() {
            Ok(tables) => {
                self.tables = tables;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    /// Updates a table in the database.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to be updated.
    /// * `command` - The command to be executed on the table.
    pub fn update_table(&mut self, table_name: &str, command: Command) -> Result<(), String> {
        let result = self
            .tables
            .iter_mut()
            .find(|table| table.name == table_name);
        match result {
            Some(table) => {
                let execute_result = table.execute(command);
                match execute_result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            None => Result::Err(format!("Table {} does not exist", table_name).to_string()),
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
        let mut database = Database::new(ram_storage);

        // When: we make a table
        database.make_table("test".to_string());

        // Then: the table is in the database
        assert_eq!(database.tables.len(), 1);
    }

    #[test]
    fn flush() {
        // Given: a database stored with a table
        let ram_storage = RAMStorage::new();
        let mut database = Database::new(ram_storage);
        database.tables = vec![Table::new("test".to_string())];

        // When: we flush the database
        let _ = database.flush();

        // Check the number of tables in the storage
        assert_eq!(database.storage.get_number_of_tables(), 1);
    }

    #[test]
    fn load_data() {
        // Given: a storage device with a table
        let mut ram_storage = RAMStorage::new();
        ram_storage.save(&[Table::new("test".to_string())]).unwrap();
        let mut database = Database::new(ram_storage);

        // When: we load the data
        let _ = database.load_data();

        // Then: the table is in the database
        assert_eq!(database.tables.len(), 1);
    }
}
