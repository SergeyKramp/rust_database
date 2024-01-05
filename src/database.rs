use std::fmt::Result;
use std::fs::File;
use crate::storables::Storage;
use crate::table::Table;

pub struct Database<'a> {
    storage: Box<dyn Storage<'a>>,
    tables: Option<Vec<Table<'a>>>,
}


impl<'a> Database<'a> {
    pub fn new(storage: Box<dyn Storage<'a>>, tables: Option<Vec<Table<'a>>>) -> Database<'a> {
        Database {
            storage,
            tables
        }
    }

    pub fn make_table(&mut self, table_name: String) {
        let table = Table::new(table_name);
        self.tables.get_or_insert(vec![]).push(table);
    }

    pub fn flush(&self) -> Result<(), std::io::Error> {
        self.storage.save(&self.tables.as_ref().unwrap())
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

        // Then: the table is in the storage
        assert_eq!((*database.storage).get_number_of_tables(), 1);
    }
}
