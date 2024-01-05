// This file contains the Storage trait and all implementations of it. 

use std::fs::File;

use crate::table::Table;

pub trait Storage<'a> {
    fn save(&mut self, tables: &'a Vec<Table>) -> Result<(), std::io::Error>;
    fn load(&self) ->  Result<Vec<Table>, std::io::Error>;
    fn get_number_of_tables(&self) -> usize;
}

pub struct RAMStorage<'a> {
    tables: Vec<Table<'a>>,
}

impl<'a> RAMStorage<'a> {
    pub fn new() -> RAMStorage<'a> {
        RAMStorage {
            tables: vec![],
        }
    }
}

impl<'a> Storage<'a> for RAMStorage<'a> {
    fn save(&mut self, tables: &'a Vec<Table>) -> Result<(), std::io::Error> {
        self.tables = tables.clone();
        Ok(())
    }

    fn load(&self) -> Result<Vec<Table<'a>>, std::io::Error> {
        Ok(self.tables.clone())
    }

    fn get_number_of_tables(&self) -> usize {
        self.tables.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::storables::RAMStorage;

    #[test]
    fn save_ram() {
        let mut ram_storage = RAMStorage::new();
        let table = Table::new("test".to_string());
        let tables = vec![table];
        let _ = ram_storage.save(&tables);
        assert_eq!(ram_storage.tables.len(), 1);
    }

    #[test]
    fn load_ram() {
        let mut ram_storage = RAMStorage::new();
        let table = Table::new("test".to_string());
        let tables = vec![table];
        ram_storage.tables = tables.clone();
        let loaded_tables = ram_storage.load();
        assert_eq!(loaded_tables.unwrap().len(), 1);
    }
}