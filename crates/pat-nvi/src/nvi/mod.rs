//! The NVI interface is made of the public wrapper struct and the public trait
//! that is implemented by the (private) concrete types. This is a better
//! alternative to just exposing the trait everywhere.

/// Module / client implementing this NVI interface.
pub mod nvi_impl;

pub type Error = Box<dyn std::error::Error>;

/// This is `pub`, and it is the actual `interface` used around the program.
/// Clients used the trait methods through this struct.
pub struct Database(Box<dyn DatabaseImpl>);

impl Database {
    pub fn insert(&mut self, id: usize, item: String) -> Result<(), Error> {
        self.0.insert(id, item)
    }
    pub fn load(&self, id: usize) -> Option<String> {
        self.0.load(id)
    }
}

/// The trait implemented by the concrete types. The NVI dynamically dispatches
/// to it.
pub trait DatabaseImpl {
    /// Inserts an item into database.
    fn insert(&mut self, id: usize, item: String) -> Result<(), Error>;
    /// Loads an item from database.
    fn load(&self, id: usize) -> Option<String>;
}
