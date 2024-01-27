///! This module contains the Storage trait and all implementations of it. 
use std::cell::RefCell;
use crate::table::Table;

///! Represents a generic storage device.
pub trait Storage<'a> {
    /// Save the tables to the storage device.
    /// 
    /// # Arguments
    /// 
    /// * `tables` - A vector of tables to be saved.
    fn save(&self, tables: &[Table<'a>]) -> Result<(), std::io::Error>;
    /// Load tables from the storage device.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a vector of tables or an `std::io::Error`.
    fn load(&self) ->  Result<Vec<Table>, std::io::Error>;
    /// Get the number of tables stored.
    /// 
    /// # Returns
    /// 
    /// The number of tables stored.
    fn get_number_of_tables(&self) -> usize;
}

///! An in memory storage container.
pub struct RAMStorage<'a> {
    tables: RefCell<Vec<Table<'a>>>,
}

impl<'a> RAMStorage<'a> {
    pub fn new() -> RAMStorage<'a> {
        RAMStorage {
            tables: RefCell::new(vec![]),
        }
    }
}

impl<'a> Storage<'a> for RAMStorage<'a> {
    fn save(&self, tables: &[Table<'a>]) -> Result<(), std::io::Error> {
        self.tables.replace(tables.to_vec());
        Ok(())
    }

    fn load(&self) -> Result<Vec<Table<'a>>, std::io::Error> {
        Ok(self.tables.borrow().to_vec())
    }

    fn get_number_of_tables(&self) -> usize {
        self.tables.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storables::RAMStorage;

    #[test]
    fn save_ram() {
        // Given: a RAMStorage  
        let mut ram_storage = RAMStorage::new();
        let table = Table::new("test".to_string());
        let tables = vec![table];

        // When: we save a table
        let _ = ram_storage.save(&tables);
        
        // Then: the table is saved
        assert_eq!(ram_storage.tables.len(), 1);
    }

    #[test]
    fn load_ram() {
        // Given: a RAMStorage with a table
        let mut ram_storage = RAMStorage::new();
        let table = Table::new("test".to_string());
        let tables = vec![table];
        ram_storage.tables = tables.clone();

        // When: we call load
        let loaded_tables = ram_storage.load();

        // Then: the table is returned
        assert_eq!(loaded_tables.unwrap().len(), 1);
    }

    #[test]
    fn test_get_number_of_tables() {
        // Given: a RAMStorage instance with some tables
        let mut storage: RAMStorage<'_> = RAMStorage::new();
        let tables = vec![Table::new("table1".to_string()), Table::new("table2".to_string())];
        storage.save(&tables).unwrap();

        // When: we call get_number_of_tables
        let num_tables = storage.get_number_of_tables();

        // Then: it should return the correct count
        assert_eq!(num_tables, 2);
    }
}