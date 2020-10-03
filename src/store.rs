use std::collections::HashMap;
use crate::RucheResult;

/// The `RucheStore` stores string key/value pairs.
pub struct RucheStore {
    database: HashMap<String, String>
}

/// The default implement for `RucheStore`.
///
/// # Example:
///
/// ```rust
/// use ruche::RucheStore;
/// let mut store = RucheStore::new();
/// store.set("k1".to_owned(), "v1".to_owned());
/// let val = store.get("k1".to_owned());
/// assert_eq!("v1".to_owned(), val.unwrap().unwrap());
/// ```
impl RucheStore {
    /// Create a `RucheStore`
    pub fn new() -> Self {
        RucheStore {
            database: HashMap::new()
        }
    }

    /// Get the value of the given key.
    pub fn get(&mut self, key: String) -> RucheResult<Option<String>> {
        let value = self.database.get(&key);
        Ok(value.cloned())
    }

    /// Set the value with the given key.
    pub fn set(&mut self, key: String, value: String) -> RucheResult<()> {
        self.database.insert(key, value);
        Ok(())
    }

    /// Remove the value with the given key.
    pub fn remove(&mut self, key: String) -> RucheResult<()> {
        self.database.remove(&key);
        Ok(())
    }
}