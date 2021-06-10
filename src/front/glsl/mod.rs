pub use error::ErrorKind;
pub use token::{SourceMetadata, Token};

use crate::{FastHashMap, Module, ShaderStage};

mod lex;

mod ast;
use ast::Program;

mod error;
pub use error::ParseError;
mod constants;
mod functions;
mod parser;
#[cfg(test)]
mod parser_tests;
mod token;
mod types;
mod variables;

pub struct Options {
    pub entry_points: FastHashMap<String, ShaderStage>,
    pub defines: FastHashMap<String, String>,
}

impl Options {
    pub fn simple(vertex: bool, fragment: bool, compute: bool) -> Self {
        let mut entry_points = crate::FastHashMap::default();

        if vertex {
            entry_points.insert("".to_string(), crate::ShaderStage::Vertex);
        }
        if fragment {
            entry_points.insert("".to_string(), crate::ShaderStage::Fragment);
        }
        if compute {
            entry_points.insert("".to_string(), crate::ShaderStage::Compute);
        }

        let defines = crate::FastHashMap::default();

        Options{
            entry_points: entry_points,
            defines: defines
        }
    }
}


pub fn parse_str(source: &str, options: &Options) -> Result<Module, ParseError> {
    let mut program = Program::new(&options.entry_points);

    let lex = lex::Lexer::new(source, &options.defines);
    let mut parser = parser::Parser::new(&mut program, lex);
    parser.parse()?;

    Ok(program.module)
}
