#![warn(clippy::pedantic)]

mod ast;
mod parser;

pub use ast::{
    Attribute, DataType, Dependency, Enum, EnumValue, Handler, Import, NameTypePair, Namespace,
    OrderedMap, Parameter, Service, SsdcFile,
};
pub use parser::{parse, parse_file, ParseError};
