#[cfg(test)]
mod scanner_tests {
    use crate::scanner::Scanner;
    use crate::token::Token;
    use crate::token::TokenType::*;

    #[test]
    fn read_program() {
        let token = Scanner::new("program").init().unwrap();
        let expect = Token::new("program", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_var() {
        let token = Scanner::new("var").init().unwrap();
        let expect = Token::new("var", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_integer() {
        let token = Scanner::new("integer").init().unwrap();
        let expect = Token::new("integer", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_if() {
        let token = Scanner::new("if").init().unwrap();
        let expect = Token::new("if", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_real() {
        let token = Scanner::new("real").init().unwrap();
        let expect = Token::new("real", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_boolean() {
        let token = Scanner::new("boolean").init().unwrap();
        let expect = Token::new("boolean", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_begin() {
        let token = Scanner::new("begin").init().unwrap();
        let expect = Token::new("begin", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_end() {
        let token = Scanner::new("end").init().unwrap();
        let expect = Token::new("end", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_else() {
        let token = Scanner::new("else").init().unwrap();
        let expect = Token::new("else", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_then() {
        let token = Scanner::new("then").init().unwrap();
        let expect = Token::new("then", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_while() {
        let token = Scanner::new("while").init().unwrap();
        let expect = Token::new("while", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_do() {
        let token = Scanner::new("do").init().unwrap();
        let expect = Token::new("do", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_not() {
        let token = Scanner::new("not").init().unwrap();
        let expect = Token::new("not", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_procedure() {
        let token = Scanner::new("procedure").init().unwrap();
        let expect = Token::new("procedure", Keyword, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_open_comment() {
        let token = Scanner::new("{commentário aberto").init();
        assert!(token.is_err())
    }

    #[test]
    fn read_closed_comment() {
        let token = Scanner::new("{commentário aberto}").init();
        assert!(token.is_ok())
    }

    #[test]
    fn read_identifier() {
        let token = Scanner::new("teste").init().unwrap();
        let expect = Token::new("teste", Identifier, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_integer_number() {
        let token = Scanner::new("5").init().unwrap();
        let expect = Token::new("5", Integer, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_real_number() {
        let token = Scanner::new("7.56").init().unwrap();
        let expect = Token::new("7.56", Real, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_semicolon() {
        let token = Scanner::new(";").init().unwrap();
        let expect = Token::new(";", Delimiter, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_colon() {
        let token = Scanner::new(":").init().unwrap();
        let expect = Token::new(":", Delimiter, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_dot() {
        let token = Scanner::new(".").init().unwrap();
        let expect = Token::new(".", Delimiter, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_open_parenthesis() {
        let token = Scanner::new("(").init().unwrap();
        let expect = Token::new("(", Delimiter, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_close_parenthesis() {
        let token = Scanner::new(")").init().unwrap();
        let expect = Token::new(")", Delimiter, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_attribution() {
        let token = Scanner::new(":=").init().unwrap();
        let expect = Token::new(":=", Attribution, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_sum() {
        let token = Scanner::new("+").init().unwrap();
        let expect = Token::new("+", AdditiveOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_sub() {
        let token = Scanner::new("-").init().unwrap();
        let expect = Token::new("-", AdditiveOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_or() {
        let token = Scanner::new("or").init().unwrap();
        let expect = Token::new("or", AdditiveOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_multi() {
        let token = Scanner::new("*").init().unwrap();
        let expect = Token::new("*", MultiplicativeOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_div() {
        let token = Scanner::new("/").init().unwrap();
        let expect = Token::new("/", MultiplicativeOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_and() {
        let token = Scanner::new("and").init().unwrap();
        let expect = Token::new("and", MultiplicativeOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_less() {
        let token = Scanner::new("<").init().unwrap();
        let expect = Token::new("<", RelationalOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_less_equal() {
        let token = Scanner::new("<=").init().unwrap();
        let expect = Token::new("<=", RelationalOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_more() {
        let token = Scanner::new(">").init().unwrap();
        let expect = Token::new(">", RelationalOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_more_equal() {
        let token = Scanner::new(">=").init().unwrap();
        let expect = Token::new(">=", RelationalOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_less_more() {
        let token = Scanner::new("<>").init().unwrap();
        let expect = Token::new("<>", RelationalOperator, 1, 0);
        assert_eq!(&expect, token.get(0).unwrap());
    }

    #[test]
    fn read_2_token() {
        let token = Scanner::new("program var").init().unwrap();
        assert_eq!(3, token.len());
    }
}