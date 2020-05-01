use std::option::Option;
use std::convert::From;

#[derive(Debug)]
pub struct Value<'l, T> {
    val: Option<T>,
    value_type: ValueType,
    shadowed: &'l Option<Value<'l, T>>
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    None,
    Integer,
    Float,
    Bool,
    String,
    Map,
    Array
}

pub trait Readable<T> {
    fn get(&self) -> &Option<T>;
}

trait Typed<T> {
    fn value_type(&self) -> ValueType;
}

// impl From<i64> for ValueType {
//     fn from(val: i64) -> Self {
//         ValueType::Integer
//     }
// }

// impl From<f64> for ValueType {
//     fn from(val: f64) -> Self {
//         ValueType::Float
//     }
// }

// impl From<&str> for ValueType {
//     fn from(val: &str) -> Self {
//         ValueType::String
//     }
// }

// impl From<&String> for ValueType {
//     fn from(val: &String) -> Self {
//         ValueType::String
//     }
// }

// impl From<bool> for ValueType {
//     fn from(val: bool) -> Self {
//         ValueType::Bool
//     }
// }

// impl<T> From<Option<T>> for ValueType {
//     fn from(val: Option<T>) -> Self {
//         match val {
//             Some(inner_val) => ValueType::from(inner_val),
//             None => ValueType::None
//         }
//     }
// }

// impl<T> From<T> for ValueType {
//     fn from(val: T) -> Self {
//         ValueType::from(val)
//     }
// }

// impl<T> From<Option<T>> for ValueType
// where T: Into<ValueType>
// {
//     fn from(val: Option<T>) -> Self {
//         match val {
//             None => ValueType::None,
//             Some(inner_val) => inner_val.into()
//         }
//     }
// }

impl<'l, T> From<Option<T>> for Value<'l, T> {
    fn from(val: Option<T>) -> Self {
        Value {
            val,
            value_type: ValueType::None,
            shadowed: &Option::None
        }
    }
}

impl<'l> From<i64> for Value<'l, i64> {
    fn from(val: i64) -> Self {
        Value {
            val: Option::Some(val),
            value_type: ValueType::Integer,
            shadowed: &Option::None
        }
    }
}

impl<'l> From<f64> for Value<'l, f64> {
    fn from(val: f64) -> Self {
        Value {
            val: Option::Some(val),
            value_type: ValueType::Float,
            shadowed: &Option::None
        }
    }
}

impl<'l> From<bool> for Value<'l, bool> {
    fn from(val: bool) -> Self {
        Value {
            val: Option::Some(val),
            value_type: ValueType::Bool,
            shadowed: &Option::None
        }
    }
}

impl<'l> From<& str> for Value<'l, String> {
    fn from(val: & str) -> Self {
        Value {
            val: Option::Some(String::from(val)),
            value_type: ValueType::String,
            shadowed: &Option::None
        }
    }
}

impl<'l> From<String> for Value<'l, String> {
    fn from(val: String) -> Self {
        Value {
            val: Option::Some(val.clone()),
            value_type: ValueType::String,
            shadowed: &Option::None
        }
    }
}

impl<'l, T> Readable<T> for Value<'l, T> {
    fn get(&self) -> &Option<T> {
        &self.val
    }
}

impl<'l, T> PartialEq<Value<'l, T>> for Value<'l, T>
where T: PartialEq<T> {
    fn eq(&self, other: &Value<T>) -> bool {
        match self.get() {
            Some(val) => {
                match other.get() {
                    Some(other_val) => val.eq(&other_val),
                    None => false
                }
            },
            None => {
                match other.get() {
                    Some(_other_val) => false,
                    None => true
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{ValueType, Value};
    use crate::value::Readable;

    #[test]
    fn test_values() {
        assert_eq!(Value::from(5i64).get().unwrap(), 5i64);
        assert_eq!(Value::from(5i64), Value::from(5i64));
        assert_eq!(Value::from(5i64).value_type, ValueType::Integer);
        assert_ne!(Value::from(5i64), Value::from(6i64));
        assert_eq!(Value::from(5f64).get().unwrap(), 5f64);
        assert_eq!(Value::from(5.3f64), Value::from(5.3f64));
        assert_eq!(Value::from(5.3f64).value_type, ValueType::Float);
        assert_ne!(Value::from(5f64), Value::from(6f64));
        assert_eq!(Value::from(true).get().unwrap(), true);
        assert_eq!(Value::from(false), Value::from(false));
        assert_eq!(Value::from(false).value_type, ValueType::Bool);
        assert_ne!(Value::from(true), Value::from(false));
        assert_eq!(Value::from("string").get().as_ref().unwrap(), & String::from("string"));
        assert_eq!(Value::from("string").value_type, ValueType::String);
        assert_eq!(Value::from(String::from("string")).get().as_ref().unwrap(), & String::from("string"));
        assert_eq!(Value::from(String::from("string")).value_type, ValueType::String);
        assert_ne!(Value::from("string"), Value::from("other_string"));
        assert_eq!(Value::from(Option::<i64>::None), Value::from(Option::<i64>::None));
        assert_eq!(Value::from(Option::<& str>::None).value_type, ValueType::None);
        assert_ne!(Value::from(Option::None), Value::from("string"));
    }
}