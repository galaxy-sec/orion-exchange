use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VarType {
    #[serde(rename = "string")]
    String(VarDefinition<String>),
    #[serde(rename = "bool")]
    Bool(VarDefinition<bool>),
    #[serde(rename = "int")]
    Int(VarDefinition<u64>),
    #[serde(rename = "float")]
    Float(VarDefinition<f64>),
}
impl VarType {
    pub fn name(&self) -> &str {
        match self {
            VarType::String(var) => &var.name,
            VarType::Bool(var) => &var.name,
            VarType::Int(var) => &var.name,
            VarType::Float(var) => &var.name,
        }
    }
    pub fn constraint(mut self, constr: ValueConstraint) -> Self {
        match &mut self {
            VarType::String(var_define) => {
                var_define.constr = Some(constr);
            }
            VarType::Bool(var_define) => {
                var_define.constr = Some(constr);
            }
            VarType::Int(var_define) => {
                var_define.constr = Some(constr);
            }
            VarType::Float(var_define) => {
                var_define.constr = Some(constr);
            }
        }
        self
    }
}

#[derive(Getters, Clone, Debug, Serialize, Deserialize)]
pub struct VarCollection {
    vars: Vec<VarType>,
}
impl VarCollection {
    pub fn define(vars: Vec<VarType>) -> Self {
        Self { vars }
    }
    // 基于VarType的name进行合并，相同的name会被覆盖
    pub fn merge(&self, other: &VarCollection) -> Self {
        let mut merged = HashMap::new();
        let mut order = Vec::new();

        // 先添加self的变量并记录顺序
        for var in &self.vars {
            let name = var.name().to_string();
            if !merged.contains_key(&name) {
                order.push(name.clone());
            }
            merged.insert(name, var.clone());
        }

        // 添加other的变量，同名会覆盖
        for var in &other.vars {
            let name = var.name().to_string();
            if !merged.contains_key(&name) {
                order.push(name.clone());
            }
            merged.insert(name, var.clone());
        }

        // 按原始顺序重新排序
        let mut result = Vec::new();
        for name in order {
            if let Some(var) = merged.get(&name) {
                result.push(var.clone());
            }
        }

        Self { vars: result }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueScope {
    pub beg: u64,
    pub end: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValueConstraint {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "scope")]
    Scope(ValueScope),
}
impl ValueConstraint {
    pub fn scope(beg: u64, end: u64) -> Self {
        ValueConstraint::Scope(ValueScope { beg, end })
    }
}

#[derive(Getters, Clone, Debug, Serialize, Deserialize)]
pub struct VarDefinition<T>
where
    T: serde::Serialize,
{
    name: String,
    value: T,
    constr: Option<ValueConstraint>,
}

impl From<(&str, &str)> for VarType {
    fn from(value: (&str, &str)) -> Self {
        Self::String(VarDefinition {
            name: value.0.to_string(),
            value: value.1.to_string(),
            constr: None,
        })
    }
}
impl From<(&str, bool)> for VarType {
    fn from(value: (&str, bool)) -> Self {
        Self::Bool(VarDefinition {
            name: value.0.to_string(),
            value: value.1,
            constr: None,
        })
    }
}
impl From<(&str, u64)> for VarType {
    fn from(value: (&str, u64)) -> Self {
        Self::Int(VarDefinition {
            name: value.0.to_string(),
            value: value.1,
            constr: None,
        })
    }
}
impl From<(&str, f64)> for VarType {
    fn from(value: (&str, f64)) -> Self {
        Self::Float(VarDefinition {
            name: value.0.to_string(),
            value: value.1,
            constr: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_vars() {
        let vars1 = VarCollection::define(vec![
            VarType::from(("a", "1")),
            VarType::from(("b", true)),
            VarType::from(("c", 10)),
        ]);

        let vars2 = VarCollection::define(vec![
            VarType::from(("b", false)),
            VarType::from(("d", 3.14)),
        ]);

        let merged = vars1.merge(&vars2);

        // 验证合并后的变量数量
        assert_eq!(merged.vars().len(), 4);

        // 验证变量顺序
        let names: Vec<&str> = merged.vars().iter().map(|v| v.name()).collect();
        assert_eq!(names, vec!["a", "b", "c", "d"]);

        // 验证变量b被正确覆盖
        if let VarType::Bool(var) = &merged.vars()[1] {
            assert_eq!(var.value(), &false);
        } else {
            panic!("变量b类型错误");
        }
    }

    #[test]
    fn test_value_constraint_serialization() {
        // 测试 Locked 变体的序列化
        let locked = ValueConstraint::Locked;
        let serialized = serde_json::to_string(&locked).unwrap();
        assert_eq!(serialized, r#""locked""#);

        // 测试 Scope 变体的序列化
        let scope = ValueConstraint::scope(1, 100);
        let serialized = serde_json::to_string(&scope).unwrap();
        assert_eq!(serialized, r#"{"scope":{"beg":1,"end":100}}"#);

        // 测试 Bool 类型的序列化
        let bool_var = VarType::Bool(VarDefinition {
            name: "test_bool".to_string(),
            value: true,
            constr: Some(ValueConstraint::scope(1, 10)),
        });
        let serialized = serde_json::to_string(&bool_var).unwrap();
        assert_eq!(
            serialized,
            r#"{"bool":{"name":"test_bool","value":true,"constr":{"scope":{"beg":1,"end":10}}}}"#
        );
    }

    #[test]
    fn test_value_constraint_deserialization() {
        // 测试 Locked 变体的反序列化
        let json = r#"{"locked":null}"#;
        let deserialized: ValueConstraint = serde_json::from_str(json).unwrap();
        assert!(matches!(deserialized, ValueConstraint::Locked));

        // 测试 Scope 变体的反序列化
        let json = r#"{"scope":{"beg":1, "end":100}}"#;
        let deserialized: ValueConstraint = serde_json::from_str(json).unwrap();
        let _constr = ValueConstraint::scope(5, 50);
        assert!(matches!(deserialized, _constr));
    }

    #[test]
    fn test_vartype_toml_serialization() {
        // 测试 String 类型的 TOML 序列化
        let string_var = VarType::String(VarDefinition {
            name: "test_str".to_string(),
            value: "hello".to_string(),
            constr: Some(ValueConstraint::Locked),
        });
        let serialized = toml::to_string(&string_var).unwrap();
        let expected = r#"[string]
name = "test_str"
value = "hello"
constr = "locked"
"#;
        assert_eq!(serialized, expected);

        // 测试 Bool 类型的 TOML 序列化
        let bool_var = VarType::Bool(VarDefinition {
            name: "test_bool".to_string(),
            value: true,
            constr: Some(ValueConstraint::scope(1, 10)),
        });
        let serialized = toml::to_string(&bool_var).unwrap();
        let expected = r#"[bool]
name = "test_bool"
value = true

[bool.constr.scope]
beg = 1
end = 10
"#;
        assert_eq!(serialized, expected);

        // 测试 Int 类型的 TOML 序列化
        let int_var = VarType::Int(VarDefinition {
            name: "test_int".to_string(),
            value: 42,
            constr: None,
        });
        let serialized = toml::to_string(&int_var).unwrap();
        let expected = r#"[int]
name = "test_int"
value = 42
"#;
        assert_eq!(serialized, expected);

        // 测试 Float 类型的 TOML 序列化
        let float_var = VarType::Float(VarDefinition {
            name: "test_float".to_string(),
            value: 3.14,
            constr: None,
        });
        let serialized = toml::to_string(&float_var).unwrap();
        let expected = r#"[float]
name = "test_float"
value = 3.14
"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_vartype_toml_deserialization() {
        // 测试 String 类型的 TOML 反序列化
        let toml_str = r#"
            [string]
            name = "test_str"
            value = "hello"
            constr = "locked"
        "#;
        let deserialized: VarType = toml::from_str(toml_str).unwrap();
        assert!(matches!(
            deserialized,
            VarType::String(VarDefinition {
                name: _,
                value: _,
                constr: Some(ValueConstraint::Locked)
            })
        ));

        // 测试 Bool 类型的 TOML 反序列化
        let toml_str = r#"
            [bool]
            name = "test_bool"
            value = false

            [bool.constr.scope]
            beg = 5
            end = 50
        "#;
        let deserialized: VarType = toml::from_str(toml_str).unwrap();
        let _constr = ValueConstraint::scope(5, 50);
        assert!(matches!(
            deserialized,
            VarType::Bool(VarDefinition {
                name: _,
                value: false,
                constr: Some(_constr)
            })
        ));

        // 测试 Int 类型的 TOML 反序列化
        let toml_str = r#"
            [int]
            name = "test_int"
            value = 100
        "#;
        let deserialized: VarType = toml::from_str(toml_str).unwrap();
        assert!(matches!(
            deserialized,
            VarType::Int(VarDefinition {
                name: _,
                value: 100,
                constr: None
            })
        ));

        // 测试 Float 类型的 TOML 反序列化
        let toml_str = r#"
            [float]
            name = "test_float"
            value = 1.618
        "#;
        let deserialized: VarType = toml::from_str(toml_str).unwrap();
        assert!(matches!(
            deserialized,
            VarType::Float(VarDefinition {
                name: _,
                value: _,
                constr: None
            })
        ));
    }
}
