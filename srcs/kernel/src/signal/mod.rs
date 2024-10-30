use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Echo(String),
    Exit,
}