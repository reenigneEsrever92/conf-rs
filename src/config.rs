use std::collections::HashMap;

struct ConfigStruct {
    options: HashMap<String, >
}

trait Config {
    fn load(source: &Source) -> Self;
    fn get(key: &str, T: );
    fn get_int(key: &str) -> i64;
    fn get_string(key: &str) -> str;
    fn get_float(key: &str) -> f64;
}

impl Config for ConfigStruct {
    options: Vec<Value>
}