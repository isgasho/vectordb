// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::errors::{Error, SQLError};
use crate::parsers::{KeyWord, Lexer, Select, Statement, IAST};

pub struct Parser {}

impl Parser {
    pub fn default() -> Self {
        Parser {}
    }

    pub fn parse(&self, sql: &str) -> Result<Statement, Error> {
        let mut lexer = Lexer::new(sql.to_string());
        let tokens = lexer.parse();
        let prefix = tokens.peek_token().unwrap();
        match prefix.keyword {
            KeyWord::SELECT => {
                let select = Select::default();
                select.parse(tokens);
                Ok(Statement::Select(Box::from(select)))
            }
            _ => Err(Error::from(SQLError::NotImplemented(format!(
                "Unsupported token:{:?}",
                prefix
            )))),
        }
    }
}

mod tests {
    #[test]
    fn test_parser() {
        use super::*;

        let p = Parser::default();
        let stmt = p.parse("select 1");
        assert_eq!(true, stmt.is_ok());
    }
}
