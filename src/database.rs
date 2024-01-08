///! Contains the data structure for a database instance.
use crate::storables::Storage;
use crate::table::Table;


///! Represents a database instance.
pub struct Database<'a> {
    storage: Box<dyn Storage<'a>>,
    tables: Option<Vec<Table<'a>>>,
}

impl<'a> Database<'a> {
    pub fn new(storage: Box<dyn Storage<'a>>, tables: Option<Vec<Table<'a>>>) -> Database<'a> {
        Database { storage, tables }
    }
    /// Creates a new database in the database.
    /// 
    /// # Arguments
    /// 
    /// * `table_name` - The name of the table to be created.
    pub fn make_table(&mut self, table_name: String) {
        let table = Table::new(table_name);
        self.tables.get_or_insert(vec![]).push(table);
    }
    /// Stores the data to the storage device.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.storage.save(&self.tables.as_ref().unwrap())
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
        let mut database = Database::new(ram_storage, None);

        // When: we make a table
        database.make_table("test".to_string());

        // Then: the table is in the database
        assert_eq!(database.tables.unwrap().len(), 1);
    }

    #[test]
    fn flush() {
        // Given: a database stored with a table
        let ram_storage: Box<RAMStorage<'_>> = Box::new(RAMStorage::new());
        let mut database = Database::new(ram_storage, None);
        database.tables = Some(vec![Table::new("test".to_string())]);

        // When: we flush the database
        database.flush();

        // Check the number of tables in the storage
        //assert_eq!(database.storage.get_number_of_tables(), 1);
    }
}
