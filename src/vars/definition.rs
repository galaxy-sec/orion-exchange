use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

use super::constraint::ValueConstraint;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VarDefinition<T>
where
    T: serde::Serialize + Clone,
{
    name: String,
    value: T,
    constr: Option<ValueConstraint>,
}
impl<T> VarDefinition<T>
where
    T: serde::Serialize + Clone,
{
    pub(crate) fn set_constr(&mut self, constr: ValueConstraint) {
        self.constr = Some(constr)
    }
    pub(crate) fn var_value(&self) -> VarValue<T> {
        VarValue {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Getters, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VarValue<T>
where
    T: serde::Serialize,
{
    name: String,
    value: T,
}

impl From<(&str, &str)> for VarDefinition<String> {
    fn from(value: (&str, &str)) -> Self {
        VarDefinition {
            name: value.0.to_string(),
            value: value.1.to_string(),
            constr: None,
        }
    }
}
impl From<(&str, bool)> for VarDefinition<bool> {
    fn from(value: (&str, bool)) -> Self {
        VarDefinition {
            name: value.0.to_string(),
            value: value.1,
            constr: None,
        }
    }
}
impl From<(&str, u64)> for VarDefinition<u64> {
    fn from(value: (&str, u64)) -> Self {
        VarDefinition {
            name: value.0.to_string(),
            value: value.1,
            constr: None,
        }
    }
}
impl From<(&str, f64)> for VarDefinition<f64> {
    fn from(value: (&str, f64)) -> Self {
        VarDefinition {
            name: value.0.to_string(),
            value: value.1,
            constr: None,
        }
    }
}

impl From<(&str, &str)> for VarValue<String> {
    fn from(value: (&str, &str)) -> Self {
        VarValue {
            name: value.0.to_string(),
            value: value.1.to_string(),
        }
    }
}
impl From<(&str, bool)> for VarValue<bool> {
    fn from(value: (&str, bool)) -> Self {
        VarValue {
            name: value.0.to_string(),
            value: value.1,
        }
    }
}
impl From<(&str, u64)> for VarValue<u64> {
    fn from(value: (&str, u64)) -> Self {
        VarValue {
            name: value.0.to_string(),
            value: value.1,
        }
    }
}
impl From<(&str, f64)> for VarValue<f64> {
    fn from(value: (&str, f64)) -> Self {
        VarValue {
            name: value.0.to_string(),
            value: value.1,
        }
    }
}
