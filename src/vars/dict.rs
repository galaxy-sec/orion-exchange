use std::collections::HashMap;

use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

use super::types::ValueType;

#[derive(Getters, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ValueDict {
    dist: HashMap<String, ValueType>,
}
impl ValueDict {
    pub(crate) fn new() -> Self {
        Self {
            dist: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, v: ValueType) -> Option<ValueType> {
        self.dist.insert(v.name().to_string(), v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_serialization() {
        let mut dict = ValueDict::new();
        dict.insert(ValueType::from(("key1", "value1")));
        dict.insert(ValueType::from(("key2", 42)));
        let content = toml::to_string(&dict).unwrap();
        println!("{}", content);

        let loaded: ValueDict = toml::from_str(content.as_str()).unwrap();
        assert_eq!(dict, loaded);
    }
}
