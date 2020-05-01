use std::fs::File;

pub trait Source {
    fn read(config: String);
}