mod test;

use std::fmt::{Display, Formatter};
use std::process::exit;
use crate::scanner::State::*;
use crate::token::*;
use crate::token::TokenType::*;

#[derive(PartialEq)]
enum State {
    S0, // initial state
    S1, S2, S3, S4, S5, S6, S7, // program
    S8, S9, S10, // var
    S11, S12, S13, S14, S15, S16, S17, // integer
    S18, // if
    S19, S20, S21, S22, //real,
    S23, S24, S25, S26, S27, S28,S29, // boolean
    S30, S31, S32, S33, // begin
    S34, S35, S36, // end
    S37, S38, S39, S40, // else
    S41, S42, S43, // then
    S44, S45, S46, S47, S48, // while
    S49, S50, // do
    S51, S52, S53, // not
    S54, S55, S56, S57, S58, S59, // procedure
    S60, // comments
    S61, // identifiers
    S62, // integer number
    S63, // real number
    S64, // ;
    S65, // :
    S66, // .
    S67, // ,
    S68, // (
    S69, // )
    S70, // :=
    S71, // +
    S72, // -
    S73, S74, // or
    S75, // *
    S76, // /
    S77, S78, S79, // and
    S80, // <
    S81, // <=
    S82, // >
    S83, // >=
    S84, // <>
    ERROR
}

#[derive(Clone, Debug)]
pub struct ScannerError {
    position: (usize, usize),
    message: String,
}

pub struct Scanner {
    buffer: String,
    tokens: Vec<Token>,
    current_state: State,
    acceptance_states: Vec<State>,
    current: usize,
    line: usize,
    offset: usize,
    error: ScannerError,
}


impl ScannerError {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        let message = message.to_string();
        let position = (line, column);
        ScannerError {message, position}
    }
    pub fn print(&self) {
        eprintln!("The error: {}. Occurred at line: {} and column: {}.", self.message, self.position.0, self.position.1);
        exit(1);
    }


}

impl Scanner {
    pub fn new(buffer: &str) -> Self {
        Scanner {
            buffer: buffer.to_string(),
            tokens: vec![],
            current_state: S0,
            acceptance_states: vec![S7, S10, S17, S18, S22, S29, S33, S36, S39, S43, S48, S50, S53, S59, S61,
                S64, S65, S66, S67, S68, S69, S70, S71, S72, S74, S75, S76, S79, S80, S81, S82, S83, S84],
            current: 0,
            line: 1,
            offset: 0,
            error: ScannerError::new("", 0, 0)
        }
    }

    pub fn init(&mut self) -> Result<Vec<Token>, ScannerError> {
        while !(self.current >= self.buffer.len()) {
            self.transition();

            if self.current_state == ERROR {
                return  Err(self.error.clone());
            }
        }
/*

        for current_char in buffer.chars() {
            self.transition(current_char);
            if self.current_state == ERROR { return Err(self.error.clone()); }
        }
 */

        let last_token = self.generate_token();
        match last_token {
            None => {}
            Some(token) => {
                self.tokens.push(token);
            }
        }

        self.tokens.push(Token::new("", EOF, self.line, self.current));
        Ok(self.tokens.clone())
    }
}

impl Scanner {
    fn generate_token(&self) -> Option<Token> {
        let lexeme = &self.buffer[self.offset..self.current];
        match &self.current_state {
            S7 | S10 | S17 | S18 | S22 | S29 | S33 | S36 | S39 | S43 | S48 | S50 | S53 | S59 => {
                Some(Token::new(lexeme, Keyword, self.line, self.offset))
            }
            S61 => { Some(Token::new(lexeme, Identifier, self.line, self.offset)) }
            S62  => { Some(Token::new(lexeme, Integer, self.line, self.offset)) }
            S63  => { Some(Token::new(lexeme, Real, self.line, self.offset)) }
            S64 | S65 | S66 | S67 | S68 | S69 => { Some(Token::new(lexeme, Delimiter, self.line, self.offset)) }
            S70 => { Some(Token::new(lexeme, Attribution, self.line, self.offset)) }
            S71 | S72 | S74 => { Some(Token::new(lexeme, AdditiveOperator, self.line, self.offset)) }
            S75 | S76 | S79 => { Some(Token::new(lexeme, MultiplicativeOperator, self.line, self.offset)) }
            S80 | S81 | S82 | S83 | S84 => { Some(Token::new(lexeme, RelationalOperator, self.line, self.offset)) }
            _ => {None}
        }
    }

    fn transition(&mut self) {
        if self.current >= self.buffer.len() {return;}

        let current = match self.buffer.chars().nth(self.current) {
            None => {' '}
            Some(c) => {c}
        };

        if !self.acceptance_states.contains(&self.current_state) {
            self.current +=1;
        }

        match self.current_state {
            // Estados de aceite
            S7 | S10 | S17 | S18 | S22 | S29 | S33 | S36 | S39 | S43 | S48 | S50 | S53 | S59 |
            S64 | S65 | S66 | S67 | S68 | S69 | S70 | S71 | S72 | S74 | S75 | S76 | S79 |
            S80 | S81 | S82 | S83 | S84 => {

                if self.current_state == S65 && current == '=' { self.current_state = S70; self.current += 1; }
                else if self.current_state == S80 && current == '=' {self.current_state = S81; self.current+=1; }
                else if self.current_state == S80 && current == '>' {self.current_state = S82; self.current+=1; }
                else if self.current_state == S83 && current == '=' {self.current_state = S84; self.current+=1; }
                else if self.current_state == S70 {
                    self.tokens.push(self.generate_token().unwrap());
                    self.offset = self.current;
                    self.current_state = S0;
                }
                else if current.is_alphanumeric() || current == '_' {
                    self.current_state = S61;
                }
                else {
                    self.tokens.push(self.generate_token().unwrap());
                    self.offset = self.current;
                    self.current_state = S0;
                }
            }
            // Estados de transição
            S0 => {
                match current {
                    '\0' => { }
                    '\n' => { self.line+=1; self.offset = self.current;}
                    '\t' | '\r' | ' ' => { self.offset = self.current;}
                    'p' => { self.current_state = S1; }
                    'v' => { self.current_state = S8; }
                    'i' => { self.current_state = S11; }
                    'r' => { self.current_state = S19; }
                    'b' => { self.current_state = S23; }
                    'e' => { self.current_state = S34; }
                    't' => { self.current_state = S40; }
                    'w' => { self.current_state = S44; }
                    'd' => { self.current_state = S49; }
                    'n' => { self.current_state = S51; }
                    '{' => { self.current_state = S60; }
                    ';' => { self.current_state = S64; }
                    ':' => { self.current_state = S65; }
                    '.' => { self.current_state = S66; }
                    ',' => { self.current_state = S67; }
                    '(' => { self.current_state = S68; }
                    ')' => { self.current_state = S69; }
                    '+' => { self.current_state = S71; }
                    '-' => { self.current_state = S72; }
                    'o' => { self.current_state = S73; }
                    '*' => { self.current_state = S75; }
                    '/' => { self.current_state = S76; }
                    'a' => { self.current_state = S77; }
                    '<' => { self.current_state = S80; }
                    '>' => { self.current_state = S83; }
                    '=' => { self.current_state = ERROR; }
                    c if c.is_alphabetic() => { self.current_state = S61; }
                    c if c.is_numeric() => { self.current_state = S62; }

                    _ => { self.current_state = S0 }
                }
            }
            S1 => {
                match current {
                    'r' => { self.current_state = S2; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S2 => {
                match current {
                    'o' => { self.current_state = S3; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S3 => {
                match current {
                    'g' => { self.current_state = S4; }
                    'c' => { self.current_state = S54; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S4 => {
                match current {
                    'r' => { self.current_state = S5; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S5 => {
                match current {
                    'a' => { self.current_state = S6; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S6 => {
                match current {
                    'm' => { self.current_state = S7; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S8 => {
                match current {
                    'a' => { self.current_state = S9; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S9 => {
                match current {
                    'r' => { self.current_state = S10; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S11 => {
                match current {
                    'n' => { self.current_state = S12; }
                    'f' => { self.current_state = S18; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S12 => {
                match current {
                    't' => { self.current_state = S13; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S13 => {
                match current {
                    'e' => { self.current_state = S14; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S14 => {
                match current {
                    'g' => { self.current_state = S15; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S15 => {
                match current {
                    'e' => { self.current_state = S16; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S16 => {
                match current {
                    'r' => { self.current_state = S17; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S19 => {
                match current {
                    'e' => { self.current_state = S20; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S20 => {
                match current {
                    'a' => { self.current_state = S21; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S21 => {
                match current {
                    'l' => { self.current_state = S22; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S23 => {
                match current {
                    'o' => { self.current_state = S24; }
                    'e' => { self.current_state = S30; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S24 => {
                match current {
                    'o' => { self.current_state = S25; }
                     _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S25 => {
                match current {
                    'l' => { self.current_state = S26; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S26 => {
                match current {
                    'e' => { self.current_state = S27; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S27 => {
                match current {
                    'a' => { self.current_state = S28; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S28 => {
                match current {
                    'n' => { self.current_state = S29; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S30 => {
                match current {
                    'g' => { self.current_state = S31; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S31 => {
                match current {
                    'i' => { self.current_state = S32; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S32 => {
                match current {
                    'n' => { self.current_state = S33; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S34 => {
                match current {
                    'n' => { self.current_state = S35; }
                    'l' => { self.current_state = S37; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S35 => {
                match current {
                    'd' => { self.current_state = S36; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S37 => {
                match current {
                    's' => { self.current_state = S38; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S38 => {
                match current {
                    'e' => { self.current_state = S39; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S40 => {
                match current {
                    'h' => { self.current_state = S41; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S41 => {
                match current {
                    'e' => { self.current_state = S42; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S42 => {
                match current {
                    'n' => { self.current_state = S43; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S44 => {
                match current {
                    'h' => { self.current_state = S45; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S45 => {
                match current {
                    'i' => { self.current_state = S46; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S46 => {
                match current {
                    'l' => { self.current_state = S47; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S47 => {
                match current {
                    'e' => { self.current_state = S48; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S49 => {
                match current {
                    'o' => { self.current_state = S50; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S51 => {
                match current {
                    'o' => { self.current_state = S52; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S52 => {
                match current {
                    't' => { self.current_state = S53; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S54 => {
                match current {
                    'e' => { self.current_state = S55; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S55 => {
                match current {
                    'd' => { self.current_state = S56; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S56 => {
                match current {
                    'u' => { self.current_state = S57; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S57 => {
                match current {
                    'r' => { self.current_state = S58; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S58 => {
                match current {
                    'e' => { self.current_state = S59; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S60 => {
                match current {
                    '}' => { self.current_state = S0; }
                    _ => match self.peek() {
                        None => {
                            self.current_state = ERROR;
                            self.error = ScannerError::new("Unclosed comment.", self.line, self.offset);
                        }
                        Some(_) => {}
                    }
                }
            }
            S61 => {
                match current {
                    c if c.is_alphanumeric() || c == '_' => { self.current += 1; }
                    _ => {
                        self.tokens.push(self.generate_token().unwrap());
                        self.offset = self.current;
                        self.current_state = S0;
                    }

                }
            }
            S62 => {
                match current {
                    c if c.is_numeric() => {}
                    '.' => { self.current_state = S63; }
                    c if !c.is_numeric() => {
                        self.current-=1;
                        self.tokens.push(self.generate_token().unwrap());
                        self.offset = self.current;
                        self.current_state = S0;
                    }
                    _ => {
                        self.current_state = S0; self.current -= 1;
                        self.error = ScannerError::new("Invalid character after number", self.line, self.offset);
                    }
                }
            }
            S63 => {
                match current {
                    c if c.is_numeric() => { }
                    c if !c.is_numeric() => {
                        self.current-=1;
                        self.tokens.push(self.generate_token().unwrap());
                        self.offset = self.current;
                        self.current_state = S0;
                    }
                    _ => {
                        self.current_state = S0; self.current -= 1;
                        self.error = ScannerError::new("Invalid character after number", self.line, self.offset);
                    }
                }
            }
            S73 => {
                match current {
                    'r' => { self.current_state = S74; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S77 => {
                match current {
                    'n' => { self.current_state = S78; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            S78 => {
                match current {
                    'd' => { self.current_state = S79; }
                    _ => { self.current_state = S61; self.current-=1;}
                }
            }
            ERROR => {}
        }
    }

    fn peek(&self) -> Option<char> {
        self.buffer.chars().nth(self.current)
    }
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The error: {}. Occurred at line: {} and column: {}.", self.message, self.position.0, self.position.1)
    }
}