mod source;
mod value;

pub use value::ValueType;
pub use value::Value;
pub use source::Source;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
