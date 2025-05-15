use std::collections::HashMap;

use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

use super::types::ValueType;

#[derive(Getters, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ValueDict {
    dict: HashMap<String, ValueType>,
}
impl ValueDict {
    pub(crate) fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, k: String, v: ValueType) -> Option<ValueType> {
        self.dict.insert(k, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict_toml_serialization() {
        let mut dict = ValueDict::new();
        dict.insert("key1".to_string(), ValueType::from("value1"));
        dict.insert("key2".to_string(), ValueType::from(42));
        let content = toml::to_string(&dict).unwrap();
        println!("{}", content);

        let loaded: ValueDict = toml::from_str(content.as_str()).unwrap();
        assert_eq!(dict, loaded);

        let content = serde_yml::to_string(&dict).unwrap();
        println!("{}", content);

        let content = serde_json::to_string(&dict).unwrap();
        println!("{}", content);
    }
}
