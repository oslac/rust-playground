use super::Error;
use crate::nvi::{self, Database};
use std::collections::HashMap;

/// Constructs the NVI interface for **this** concrete type.
pub fn new() -> Database {
    Database(Box::new(MemoryDb::default()))
}

/// Type implementing the NVI interface; no `new` method, this type is *not*
/// `pub`.
#[derive(Debug, Default)]
struct MemoryDb(HashMap<usize, String>);

impl nvi::DatabaseImpl for MemoryDb {
    fn insert(&mut self, id: usize, item: String) -> Result<(), Error> {
        self.0.insert(id, item);
        Ok(())
    }

    fn load(&self, id: usize) -> Option<String> {
        self.0.get(&id).cloned()
    }
}
