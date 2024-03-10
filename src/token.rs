use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword, // program, var, integer, real, boolean, procedure, begin, end, if, then, else, while, do, not
    Identifier, // ([a..z]* | [A..Z]*) _ | [0..1] | ([a..z]* | [A..Z]*)
    Integer, // [0..9]*
    Real, // [0..9]* . [0..9]
    Delimiter, // : ponto e virgula (;), ponto final (.), dois pontos (:), abre e fecha parênteses ( () ) e virgula (,)
    Attribution, // :=
    RelationalOperator, // =, <, >, <=, >=, <>
    AdditiveOperator, // +, -, or
    MultiplicativeOperator, // *, /, and
    EOF
}


#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}


impl Token {
    pub fn new(lexeme: &str, token_type: TokenType, line: usize, column: usize) -> Self {
        let lexeme = lexeme.to_string();
        Token {lexeme, token_type, line, column}
    }
}


impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenType::Keyword => {write!(f, "Keyword")}
            TokenType::Identifier => {write!(f, "Identifier")}
            TokenType::Integer => {write!(f, "Integer")}
            TokenType::Real => {write!(f, "Real")}
            TokenType::Delimiter => {write!(f, "Delimiter")}
            TokenType::Attribution => {write!(f, "Attribution")}
            TokenType::RelationalOperator => {write!(f, "RelationalOperator")}
            TokenType::AdditiveOperator => {write!(f, "AdditiveOperator")}
            TokenType::MultiplicativeOperator => {write!(f, "MultiplicativeOperator")}
            TokenType::EOF => { write!(f, "EOF") }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<{}, {}, (line: {}, column: {})>", self.lexeme, self.token_type, self.line, self.column)
    }
}