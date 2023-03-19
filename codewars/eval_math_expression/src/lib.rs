mod grammar {
    use self::{
        eval::{eval_parsed, EvalError},
        parse::{ExprParser, ParserError},
        tokens::tokens_iter,
    };

    mod tokens {
        use std::{
            cmp::Ordering,
            error::Error,
            fmt::Display,
            num::{ParseFloatError, ParseIntError},
            str::Chars,
        };

        #[derive(PartialEq, Eq, Debug, Clone)]
        pub enum Operator {
            Neg,
            Add,
            Sub,
            Mul,
            Div,
        }

        type Priority = u8;

        pub type Arity = u8;

        fn get_operator_priority(operator: &Operator) -> Priority {
            match operator {
                Operator::Neg => 0,
                Operator::Mul | Operator::Div => 1,
                Operator::Add | Operator::Sub => 2,
            }
        }

        impl Operator {
            pub fn arity(&self) -> Arity {
                match self {
                    Operator::Neg => 1,
                    _ => 2,
                }
            }
        }

        impl Ord for Operator {
            fn cmp(&self, other: &Self) -> Ordering {
                get_operator_priority(self).cmp(&get_operator_priority(other))
            }
        }

        impl PartialOrd for Operator {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        #[derive(PartialEq, Debug, Clone)]
        pub enum Group {
            Open,
            Close,
        }

        #[derive(PartialEq, Debug, Clone)]
        pub enum Number {
            Int(i32),
            Float(f64),
        }

        #[derive(PartialEq, Debug, Clone)]
        pub enum Token {
            Operator(Operator),
            Group(Group),
            Number(Number),
        }

        #[derive(Debug, PartialEq, Eq)]
        pub enum NumberParseErrorKind {
            Int(ParseIntError),
            Float(ParseFloatError),
        }

        #[derive(Debug, PartialEq, Eq)]
        pub enum TokenizerError {
            UnknownToken(char),
            NumberParseError { kind: NumberParseErrorKind },
        }

        impl Display for TokenizerError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::UnknownToken(token) => write!(f, "Unknown token `{token}` in the stream"),
                    Self::NumberParseError { .. } => write!(f, "Unable to parse number"),
                }
            }
        }

        impl Error for TokenizerError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                if let TokenizerError::NumberParseError { kind } = self {
                    match kind {
                        NumberParseErrorKind::Int(err) => Some(err),
                        NumberParseErrorKind::Float(err) => Some(err),
                    }
                } else {
                    None
                }
            }
        }

        impl From<ParseIntError> for TokenizerError {
            fn from(value: ParseIntError) -> Self {
                Self::NumberParseError {
                    kind: NumberParseErrorKind::Int(value),
                }
            }
        }

        impl From<ParseFloatError> for TokenizerError {
            fn from(value: ParseFloatError) -> Self {
                Self::NumberParseError {
                    kind: NumberParseErrorKind::Float(value),
                }
            }
        }

        pub type Result<T> = std::result::Result<T, TokenizerError>;

        pub struct TokenIterator<'stream> {
            stream: Chars<'stream>,
            input: Option<char>,
            expect_for_neg: bool,
        }

        impl<'stream> TokenIterator<'stream> {
            pub fn new(stream: Chars<'stream>) -> Self {
                Self {
                    stream,
                    input: None,
                    expect_for_neg: true,
                }
            }

            fn exhaust_whitespace(&mut self) {
                for input in self.stream.by_ref() {
                    if input != ' ' {
                        self.input = Some(input);

                        return;
                    }
                }
            }

            fn exhaust_number(&mut self, first_digit: char) -> Result<Number> {
                let mut had_dot = first_digit == '.';
                let mut digits = vec![first_digit];

                for input in self.stream.by_ref() {
                    match input {
                        '0'..='9' => digits.push(input),
                        '.' => {
                            had_dot = true;
                            digits.push(input);
                        }
                        _ => {
                            self.input = Some(input);
                            break;
                        }
                    }
                }

                let digits = String::from_iter(digits);

                if had_dot {
                    Ok(Number::Float(digits.parse::<f64>()?))
                } else {
                    Ok(Number::Int(digits.parse::<i32>()?))
                }
            }
        }

        impl<'stream> Iterator for TokenIterator<'stream> {
            type Item = Result<Token>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.input.is_none() || self.input == Some(' ') {
                    self.exhaust_whitespace();
                }

                let Some(input) = self.input.take() else {
                    return None;
                };

                let result = match input {
                    '+' => Token::Operator(Operator::Add),
                    '-' => {
                        if self.expect_for_neg {
                            Token::Operator(Operator::Neg)
                        } else {
                            Token::Operator(Operator::Sub)
                        }
                    }
                    '*' => Token::Operator(Operator::Mul),
                    '/' => Token::Operator(Operator::Div),
                    '(' => Token::Group(Group::Open),
                    ')' => Token::Group(Group::Close),
                    '0'..='9' | '.' => match self.exhaust_number(input) {
                        Ok(num) => Token::Number(num),
                        Err(err) => return Some(Err(err)),
                    },
                    _ => return Some(Err(TokenizerError::UnknownToken(input))),
                };

                self.expect_for_neg = match &result {
                    Token::Operator(operator) => *operator != Operator::Neg,
                    Token::Group(Group::Open) => true,
                    _ => false,
                };

                Some(Ok(result))
            }
        }

        pub fn tokens_iter(text: &str) -> TokenIterator {
            TokenIterator::new(text.chars())
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            macro_rules! assert_tokens {
                ($str: literal $(, $token:expr )*) => {
                    assert_eq!(tokens_iter($str).collect::<Vec<_>>(), vec![$($token),*])
                };
            }

            #[test]
            fn empty_string() {
                assert_tokens!("");
            }

            #[test]
            fn single_token() {
                assert_tokens!("+", Ok(Token::Operator(Operator::Add)));
                assert_tokens!("/", Ok(Token::Operator(Operator::Div)));
                assert_tokens!("(", Ok(Token::Group(Group::Open)));
                assert_tokens!("1", Ok(Token::Number(Number::Int(1))));
                assert_tokens!("1.25", Ok(Token::Number(Number::Float(1.25))));
                assert_tokens!(
                    "-3.8",
                    Ok(Token::Operator(Operator::Neg)),
                    Ok(Token::Number(Number::Float(3.8)))
                );
                assert_tokens!(".5", Ok(Token::Number(Number::Float(0.5))));
                assert_tokens!("5.", Ok(Token::Number(Number::Float(5.0))))
            }

            #[test]
            fn complex_neg() {
                assert_tokens!(
                    "(-(-1))",
                    Ok(Token::Group(Group::Open)),
                    Ok(Token::Operator(Operator::Neg)),
                    Ok(Token::Group(Group::Open)),
                    Ok(Token::Operator(Operator::Neg)),
                    Ok(Token::Number(Number::Int(1))),
                    Ok(Token::Group(Group::Close)),
                    Ok(Token::Group(Group::Close))
                );

                assert_tokens!(
                    "- ( 2) - (-3)",
                    Ok(Token::Operator(Operator::Neg)),
                    Ok(Token::Group(Group::Open)),
                    Ok(Token::Number(Number::Int(2))),
                    Ok(Token::Group(Group::Close)),
                    Ok(Token::Operator(Operator::Sub)),
                    Ok(Token::Group(Group::Open)),
                    Ok(Token::Operator(Operator::Neg)),
                    Ok(Token::Number(Number::Int(3))),
                    Ok(Token::Group(Group::Close))
                );
            }

            #[test]
            fn list_of_tokens() {
                assert_tokens!(
                    " 2 +   3",
                    Ok(Token::Number(Number::Int(2))),
                    Ok(Token::Operator(Operator::Add)),
                    Ok(Token::Number(Number::Int(3)))
                );

                assert_tokens!(
                    " 2.3 + ( 2.9 - 3.5 * (2.1 / 2.9 - 3253252.12) + 3 ) + 212",
                    Ok(Token::Number(Number::Float(2.3))),
                    Ok(Token::Operator(Operator::Add)),
                    Ok(Token::Group(Group::Open)),
                    Ok(Token::Number(Number::Float(2.9))),
                    Ok(Token::Operator(Operator::Sub)),
                    Ok(Token::Number(Number::Float(3.5))),
                    Ok(Token::Operator(Operator::Mul)),
                    Ok(Token::Group(Group::Open)),
                    Ok(Token::Number(Number::Float(2.1))),
                    Ok(Token::Operator(Operator::Div)),
                    Ok(Token::Number(Number::Float(2.9))),
                    Ok(Token::Operator(Operator::Sub)),
                    Ok(Token::Number(Number::Float(3253252.12))),
                    Ok(Token::Group(Group::Close)),
                    Ok(Token::Operator(Operator::Add)),
                    Ok(Token::Number(Number::Int(3))),
                    Ok(Token::Group(Group::Close)),
                    Ok(Token::Operator(Operator::Add)),
                    Ok(Token::Number(Number::Int(212)))
                )
            }

            #[test]
            fn wrong_single_token() {
                let parse_float_error = "..".parse::<f32>().unwrap_err();

                assert_tokens!(":", Err(TokenizerError::UnknownToken(':')));
                assert_tokens!("a", Err(TokenizerError::UnknownToken('a')));
                assert_tokens!(
                    "2213.2132.233",
                    Err(TokenizerError::NumberParseError {
                        kind: NumberParseErrorKind::Float(parse_float_error)
                    })
                );
                assert_tokens!(
                    "2 + a",
                    Ok(Token::Number(Number::Int(2))),
                    Ok(Token::Operator(Operator::Add)),
                    Err(TokenizerError::UnknownToken('a'))
                );
                assert_tokens!(
                    ": + (3 + 2)",
                    Err(TokenizerError::UnknownToken(':')),
                    Ok(Token::Operator(Operator::Add)),
                    Ok(Token::Group(Group::Open)),
                    Ok(Token::Number(Number::Int(3))),
                    Ok(Token::Operator(Operator::Add)),
                    Ok(Token::Number(Number::Int(2))),
                    Ok(Token::Group(Group::Close))
                )
            }
        }
    }

    mod parse {
        use std::{cmp::Ordering, fmt::Display};

        use super::tokens::{Group, Result as TokenizerResult, Token, TokenizerError};

        #[derive(Debug, PartialEq)]
        pub enum ParserError {
            TokenizerError(TokenizerError),
            EmptyExpr,
            UnbalancedBrackets(Option<Token>),
            OperatorExpected(Option<Token>),
            OperandExpected {
                token: Option<Token>,
                operator: Option<Token>,
            },
        }

        impl From<TokenizerError> for ParserError {
            fn from(error: TokenizerError) -> Self {
                Self::TokenizerError(error)
            }
        }

        impl Display for ParserError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::TokenizerError(error) => write!(f, "{error}"),
                    Self::EmptyExpr => write!(f, "Expression is empty"),
                    Self::UnbalancedBrackets(_) => write!(f, "Unbalanced brackets"),
                    Self::OperatorExpected(_) => write!(f, "Expected operator"),
                    Self::OperandExpected { .. } => write!(f, "Expected operand"),
                }
            }
        }

        impl std::error::Error for ParserError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    Self::TokenizerError(error) => Some(error),
                    _ => None,
                }
            }
        }

        pub type Result<T> = std::result::Result<T, ParserError>;

        pub trait Ast<'token>: IntoIterator<Item = &'token Token> {}

        #[derive(Default, Debug)]
        enum State {
            #[default]
            Start,
            Operand,
            OperatorOrEnd,
        }

        #[derive(Default, Debug)]
        pub struct ExprAst {
            stack: Vec<Token>,
            state: State,
            group_nesting_index: u32,
        }

        impl<'token> IntoIterator for &'token ExprAst {
            type Item = &'token Token;

            type IntoIter = std::iter::Rev<std::slice::Iter<'token, Token>>;

            fn into_iter(self) -> Self::IntoIter {
                self.stack.iter().rev()
            }
        }

        impl<'token> Ast<'token> for &'token ExprAst {}

        impl ExprAst {
            fn parse(
                &mut self,
                tokens_iter: &mut impl Iterator<Item = TokenizerResult<Token>>,
            ) -> Result<()> {
                self.parse_group(tokens_iter)?;

                if self.group_nesting_index > 0 {
                    return Err(ParserError::UnbalancedBrackets(None));
                }

                if let State::Start = self.state {
                    return Err(ParserError::EmptyExpr);
                }

                Ok(())
            }

            fn parse_group(
                &mut self,
                tokens_iter: &mut impl Iterator<Item = TokenizerResult<Token>>,
            ) -> Result<()> {
                let mut operator_stack = vec![];

                while let Some(token) = tokens_iter.next() {
                    let token = token?;

                    match self.state {
                        State::Start | State::Operand => match token {
                            Token::Number(_) => {
                                self.stack.push(token);
                                self.state = State::OperatorOrEnd
                            }
                            Token::Operator(ref operator) => {
                                if operator.arity() > 1 {
                                    return Err(ParserError::OperandExpected {
                                        token: Some(token),
                                        operator: operator_stack.pop(),
                                    });
                                }

                                let last_token = operator_stack.last();

                                if let Some(Token::Operator(prev_op)) = last_token {
                                    if operator == prev_op {
                                        return Err(ParserError::OperandExpected {
                                            token: Some(token),
                                            operator: operator_stack.pop(),
                                        });
                                    }
                                }

                                operator_stack.push(token);
                                self.state = State::Operand;
                            }
                            Token::Group(Group::Open) => {
                                self.group_nesting_index += 1;
                                self.state = State::Start;
                                self.parse_group(tokens_iter)?;
                            }
                            Token::Group(Group::Close) => {
                                return Err(if operator_stack.is_empty() {
                                    if self.group_nesting_index > 0 {
                                        ParserError::EmptyExpr
                                    } else {
                                        ParserError::UnbalancedBrackets(Some(token))
                                    }
                                } else {
                                    ParserError::OperandExpected {
                                        token: Some(token),
                                        operator: operator_stack.pop(),
                                    }
                                })
                            }
                        },
                        State::OperatorOrEnd => match token {
                            Token::Number(_) | Token::Group(Group::Open) => {
                                return Err(ParserError::OperatorExpected(Some(token)))
                            }
                            Token::Operator(ref operator) => {
                                let last_token = operator_stack.last();

                                if let Some(Token::Operator(prev_op)) = last_token {
                                    match operator.cmp(prev_op) {
                                        Ordering::Greater | Ordering::Equal => {
                                            while let Some(prev_op) = operator_stack.last() {
                                                if let Token::Operator(prev_op) = prev_op {
                                                    if operator.cmp(prev_op) == Ordering::Less {
                                                        break;
                                                    }
                                                }

                                                self.stack.push(operator_stack.pop().unwrap());
                                            }
                                        }
                                        _ => {}
                                    }
                                }

                                operator_stack.push(token);
                                self.state = State::Operand;
                            }
                            Token::Group(Group::Close) => {
                                if self.group_nesting_index == 0 {
                                    return Err(ParserError::UnbalancedBrackets(Some(token)));
                                }

                                self.group_nesting_index -= 1;
                                self.state = State::OperatorOrEnd;
                                break;
                            }
                        },
                    }
                }

                if let State::Operand = self.state {
                    return Err(ParserError::OperandExpected {
                        token: None,
                        operator: operator_stack.pop(),
                    });
                }

                while let Some(operator) = operator_stack.pop() {
                    self.stack.push(operator);
                }

                Ok(())
            }
        }

        #[derive(Default)]
        pub struct ExprParser {}

        impl ExprParser {
            pub fn new() -> Self {
                Self {}
            }

            pub fn parse(
                &self,
                tokens_iter: &mut impl Iterator<Item = TokenizerResult<Token>>,
            ) -> Result<ExprAst> {
                let mut ast = ExprAst::default();

                ast.parse(tokens_iter)?;

                Ok(ast)
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::grammar::tokens::{tokens_iter, Number, Operator};

            macro_rules! assert_parse {
                ($expr: literal $(, $token: expr)*) => {
                    let parser = ExprParser::new();
                    let ast = parser.parse(&mut tokens_iter($expr)).unwrap();
                    assert_eq!(ast.into_iter().collect::<Vec<_>>(), vec![$(&$token, )*])
                };
            }

            macro_rules! assert_parse_error {
                ($expr: literal, $err: expr) => {
                    let parser = ExprParser::new();
                    let err = parser.parse(&mut tokens_iter($expr)).unwrap_err();
                    assert_eq!(err, $err)
                };
            }

            #[test]
            fn unit_expr() {
                assert_parse!("2", Token::Number(Number::Int(2)));
                assert_parse!("3.7", Token::Number(Number::Float(3.7)));
            }

            #[test]
            fn simple_expr() {
                assert_parse!(
                    "2 +  3",
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(3)),
                    Token::Number(Number::Int(2))
                );
            }

            #[test]
            fn priority_expr() {
                assert_parse!(
                    "2 + 2 - 3",
                    Token::Operator(Operator::Sub),
                    Token::Number(Number::Int(3)),
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Int(2))
                );

                assert_parse!(
                    "2 + 2 * 3",
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Int(3)),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Int(2))
                );

                assert_parse!(
                    "2 * 2 + 3",
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(3)),
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Int(2))
                );

                assert_parse!(
                    "2 + 2 * 3 * -2",
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Mul),
                    Token::Operator(Operator::Neg),
                    Token::Number(Number::Int(2)),
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Int(3)),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Int(2))
                );
            }

            #[test]
            fn grouping() {
                assert_parse!("(1)", Token::Number(Number::Int(1)));
                assert_parse!("(((1)))", Token::Number(Number::Int(1)));
                assert_parse!(
                    "-(-(1))",
                    Token::Operator(Operator::Neg),
                    Token::Operator(Operator::Neg),
                    Token::Number(Number::Int(1))
                );

                assert_parse!(
                    "2 * 3 + (2 + 3 ) * 5.1",
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Float(5.1)),
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(3)),
                    Token::Number(Number::Int(2)),
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Int(3)),
                    Token::Number(Number::Int(2))
                );
            }

            #[test]
            fn unbalanced_brackets() {
                assert_parse_error!(
                    ")",
                    ParserError::UnbalancedBrackets(Some(Token::Group(Group::Close)))
                );
                assert_parse_error!(
                    "5)",
                    ParserError::UnbalancedBrackets(Some(Token::Group(Group::Close)))
                );
                assert_parse_error!("(", ParserError::UnbalancedBrackets(None));
                assert_parse_error!("( 3", ParserError::UnbalancedBrackets(None));
                assert_parse_error!(
                    "1 + 2 * (3 - 2 ) + )",
                    ParserError::OperandExpected {
                        token: Some(Token::Group(Group::Close)),
                        operator: Some(Token::Operator(Operator::Add))
                    }
                );
                assert_parse_error!(
                    "1 + 2 * (3 - 2 ) * (",
                    ParserError::UnbalancedBrackets(None)
                );
                assert_parse_error!("( 3 + (2 + 2)", ParserError::UnbalancedBrackets(None));
                assert_parse_error!(
                    "( 3 + (2 + 2) ) )",
                    ParserError::UnbalancedBrackets(Some(Token::Group(Group::Close)))
                );

                assert_parse_error!("", ParserError::EmptyExpr);

                assert_parse_error!("()", ParserError::EmptyExpr);
            }

            #[test]
            fn missing_operand() {
                assert_parse_error!(
                    "+",
                    ParserError::OperandExpected {
                        token: Some(Token::Operator(Operator::Add)),
                        operator: None
                    }
                );
                assert_parse_error!(
                    "* 3 - 2",
                    ParserError::OperandExpected {
                        token: Some(Token::Operator(Operator::Mul)),
                        operator: None
                    }
                );
                assert_parse_error!(
                    "2 -",
                    ParserError::OperandExpected {
                        token: None,
                        operator: Some(Token::Operator(Operator::Sub))
                    }
                );
                assert_parse_error!(
                    "2 + (3 - 2) *",
                    ParserError::OperandExpected {
                        token: None,
                        operator: Some(Token::Operator(Operator::Mul))
                    }
                );
                assert_parse_error!(
                    "2 * / 2",
                    ParserError::OperandExpected {
                        token: Some(Token::Operator(Operator::Div)),
                        operator: Some(Token::Operator(Operator::Mul))
                    }
                );
                assert_parse_error!(
                    "- ",
                    ParserError::OperandExpected {
                        token: None,
                        operator: Some(Token::Operator(Operator::Neg))
                    }
                );
                assert_parse_error!(
                    " 2 + -",
                    ParserError::OperandExpected {
                        token: None,
                        operator: Some(Token::Operator(Operator::Neg))
                    }
                );
                assert_parse_error!(
                    " 2 + (-)",
                    ParserError::OperandExpected {
                        token: Some(Token::Group(Group::Close)),
                        operator: Some(Token::Operator(Operator::Neg))
                    }
                );
                assert_parse_error!(
                    "- - 2",
                    ParserError::OperandExpected {
                        token: Some(Token::Operator(Operator::Sub)),
                        operator: Some(Token::Operator(Operator::Neg))
                    }
                );
                assert_parse_error!(
                    "2 * - - 2",
                    ParserError::OperandExpected {
                        token: Some(Token::Operator(Operator::Sub)),
                        operator: Some(Token::Operator(Operator::Neg))
                    }
                );
            }

            #[test]
            fn missing_operator() {
                assert_parse_error!(
                    "2 3",
                    ParserError::OperatorExpected(Some(Token::Number(Number::Int(3))))
                );
                assert_parse_error!(
                    "(2 +3 ) (5*6)",
                    ParserError::OperatorExpected(Some(Token::Group(Group::Open)))
                );
            }
        }
    }

    mod eval {
        use super::tokens::*;

        #[derive(Debug, PartialEq)]
        pub enum CalculationError {
            ZeroDivision,
        }

        impl std::fmt::Display for CalculationError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::ZeroDivision => write!(f, "division by zero"),
                }
            }
        }

        impl std::error::Error for CalculationError {}

        #[derive(Debug, PartialEq)]
        pub enum EvalError {
            UnexpectedToken(Token),
            UnexpectedEndOfInput,
            UnconsumedToken(Token),
            CalculationError(CalculationError),
        }

        impl std::fmt::Display for EvalError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::UnexpectedToken(_) => write!(f, "Unexpected token"),
                    Self::UnexpectedEndOfInput => write!(f, "Input stream has ended unexpectedly"),
                    Self::UnconsumedToken(_) => write!(f, "Expression was calculated, but the stream contains more elements that were ignored"),
                    Self::CalculationError(err) => write!(f, "{err}"),
                }
            }
        }

        impl std::error::Error for EvalError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    Self::CalculationError(err) => Some(err),
                    _ => None,
                }
            }
        }

        impl From<CalculationError> for EvalError {
            fn from(error: CalculationError) -> Self {
                Self::CalculationError(error)
            }
        }

        pub type Result<T> = std::result::Result<T, EvalError>;

        const ZERO: f64 = 0.0;

        fn eval<'token>(tokens: &mut impl Iterator<Item = &'token Token>) -> Result<f64> {
            let Some(token) = tokens.next() else {
                return Err(EvalError::UnexpectedEndOfInput);
            };

            match token {
                Token::Number(Number::Float(num)) => Ok(*num),
                Token::Number(Number::Int(num)) => Ok(*num as f64),
                Token::Operator(operator) => {
                    let arity = operator.arity();
                    let right_arg = if arity > 0 { eval(tokens)? } else { ZERO };
                    let left_arg = if arity > 1 { eval(tokens)? } else { ZERO };

                    Ok(match operator {
                        Operator::Neg => -right_arg,
                        Operator::Add => left_arg + right_arg,
                        Operator::Sub => left_arg - right_arg,
                        Operator::Mul => left_arg * right_arg,
                        Operator::Div => {
                            if right_arg == ZERO {
                                return Err(CalculationError::ZeroDivision.into());
                            }

                            left_arg / right_arg
                        }
                    })
                }
                _ => Err(EvalError::UnexpectedToken(token.clone())),
            }
        }

        pub fn eval_parsed<'token>(
            tokens: &mut impl Iterator<Item = &'token Token>,
        ) -> Result<f64> {
            let value = eval(tokens)?;

            if let Some(token) = tokens.next() {
                return Err(EvalError::UnconsumedToken(token.clone()));
            }

            Ok(value)
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::grammar::tokens::{Number, Operator, Token};

            macro_rules! assert_eval {
                ($result: literal $(, $expr: expr)*) => {
                    let tokens = vec![$($expr,)*];
                    assert_eq!(eval_parsed(&mut tokens.iter()).unwrap(), $result)
                };
            }

            macro_rules! assert_eval_error {
                ($err: expr $(, $expr: expr)*) => {
                    let tokens = vec![$($expr,)*];
                    assert_eq!(eval_parsed(&mut tokens.iter()).unwrap_err(), $err)
                };
            }

            #[test]
            fn simple_expr() {
                assert_eval!(
                    3.0,
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(1)),
                    Token::Number(Number::Int(2))
                );

                assert_eval!(
                    5.0,
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Float(2.5))
                );
            }

            #[test]
            fn complex_expr() {
                // (2 + 2 * 2 - 4 + 50 + (10 - 30)) / (3 + 2.8 * 2 - (0.3 * 2))
                assert_eval!(
                    4.0,
                    // () / ()
                    Token::Operator(Operator::Div),
                    // (2+...)
                    Token::Operator(Operator::Sub),
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Float(0.3)),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Float(2.8)),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Int(3)),
                    // (3+...)
                    Token::Operator(Operator::Add),
                    // (10 - 30)
                    Token::Operator(Operator::Sub),
                    Token::Number(Number::Int(30)),
                    Token::Number(Number::Int(10)),
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(50)),
                    Token::Operator(Operator::Sub),
                    Token::Number(Number::Int(4)),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Mul),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Int(2))
                );
            }

            #[test]
            fn empty_expr() {
                assert_eval_error!(EvalError::UnexpectedEndOfInput);
            }

            #[test]
            fn unexpected_end() {
                assert_eval_error!(
                    EvalError::UnexpectedEndOfInput,
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(2))
                );

                assert_eval_error!(
                    EvalError::UnexpectedEndOfInput,
                    Token::Operator(Operator::Neg)
                );
            }

            #[test]
            fn unconsumed_tokens() {
                assert_eval_error!(
                    EvalError::UnconsumedToken(Token::Number(Number::Int(3))),
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(1)),
                    Token::Number(Number::Int(2)),
                    Token::Number(Number::Int(3))
                );
                assert_eval_error!(
                    EvalError::UnconsumedToken(Token::Number(Number::Int(2))),
                    Token::Number(Number::Int(1)),
                    Token::Number(Number::Int(2))
                );
            }

            #[test]
            fn division_by_zero() {
                assert_eval_error!(
                    EvalError::CalculationError(CalculationError::ZeroDivision),
                    Token::Operator(Operator::Div),
                    Token::Number(Number::Int(0)),
                    Token::Number(Number::Int(1))
                );
                assert_eval_error!(
                    EvalError::CalculationError(CalculationError::ZeroDivision),
                    Token::Operator(Operator::Div),
                    Token::Number(Number::Float(0.0)),
                    Token::Number(Number::Float(1.0))
                );
            }
        }
    }

    #[derive(Debug)]
    pub enum ExprError {
        ParserError(ParserError),
        EvalError(EvalError),
    }

    impl std::fmt::Display for ExprError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ParserError(err) => write!(f, "{err}"),
                Self::EvalError(err) => write!(f, "{err}"),
            }
        }
    }

    impl std::error::Error for ExprError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Self::ParserError(err) => Some(err),
                Self::EvalError(err) => Some(err),
            }
        }
    }

    impl From<ParserError> for ExprError {
        fn from(error: ParserError) -> Self {
            Self::ParserError(error)
        }
    }

    impl From<EvalError> for ExprError {
        fn from(error: EvalError) -> Self {
            Self::EvalError(error)
        }
    }

    pub type Result<T> = std::result::Result<T, ExprError>;

    pub fn eval(expr: &str) -> Result<f64> {
        let mut tokens = tokens_iter(expr);
        let parser = ExprParser::new();
        let parser = parser.parse(&mut tokens)?;

        Ok(eval_parsed(&mut parser.into_iter())?)
    }
}

pub fn calc(expr: &str) -> f64 {
    // The task itself works only with valid expressions so no error handling for codewars wrapper here
    grammar::eval(expr).unwrap()
}

#[cfg(test)]
mod tests {
    use super::calc;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        };
    }

    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
        assert_expr_eq!("((80 - (19)))", 61.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    }
}
