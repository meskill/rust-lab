mod grammar {
    use self::{eval::eval_parsed, parse::ExprParser, tokens::tokens_iter};

    mod tokens {
        use std::{cmp::Ordering, str::Chars};

        #[derive(PartialEq, Eq, Debug)]
        pub enum Operator {
            Neg,
            Add,
            Sub,
            Mul,
            Div,
        }

        type Priority = u8;

        fn get_operator_priority(operator: &Operator) -> Priority {
            match operator {
                Operator::Neg => 0,
                Operator::Mul | Operator::Div => 1,
                Operator::Add | Operator::Sub => 2,
            }
        }

        impl Operator {
            pub fn arity(&self) -> u8 {
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

        #[derive(PartialEq, Debug)]
        pub enum Group {
            Open,
            Close,
        }

        #[derive(PartialEq, Debug)]
        pub enum Number {
            Int(i32),
            Float(f64),
        }

        #[derive(PartialEq, Debug)]
        pub enum Token {
            Operator(Operator),
            Group(Group),
            Number(Number),
        }

        pub struct TokenIterator<'stream> {
            stream: Chars<'stream>,
            input: Option<char>,
            expect_for_neg: bool,
        }

        impl<'stream> TokenIterator<'stream> {
            pub fn new(stream: Chars<'stream>) -> Self {
                let mut iter = Self {
                    stream,
                    input: None,
                    expect_for_neg: true,
                };

                iter.exhaust_whitespace();

                iter
            }

            fn exhaust_whitespace(&mut self) {
                for input in self.stream.by_ref() {
                    if input != ' ' {
                        self.input = Some(input);

                        return;
                    }
                }
            }

            fn exhaust_number(&mut self, first_digit: char) -> Number {
                let mut had_dot = first_digit == '.';
                let mut digits = vec![first_digit];

                for input in self.stream.by_ref() {
                    match input {
                        '0'..='9' => digits.push(input),
                        '.' => {
                            if had_dot {
                                panic!("Unknown number format");
                            }

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
                    Number::Float(digits.parse::<f64>().unwrap())
                } else {
                    Number::Int(digits.parse::<i32>().unwrap())
                }
            }
        }

        impl<'stream> Iterator for TokenIterator<'stream> {
            type Item = Token;

            fn next(&mut self) -> Option<Self::Item> {
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
                    '0'..='9' | '.' => Token::Number(self.exhaust_number(input)),
                    // @TODO: use Error
                    _ => panic!("Unknown token {input}"),
                };

                self.expect_for_neg = match &result {
                    Token::Operator(operator) => *operator != Operator::Neg,
                    Token::Group(Group::Open) => true,
                    _ => false,
                };

                if self.input.is_none() || self.input == Some(' ') {
                    self.exhaust_whitespace();
                }

                Some(result)
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
                assert_tokens!("+", Token::Operator(Operator::Add));
                assert_tokens!("/", Token::Operator(Operator::Div));
                assert_tokens!("(", Token::Group(Group::Open));
                assert_tokens!("1", Token::Number(Number::Int(1)));
                assert_tokens!("1.25", Token::Number(Number::Float(1.25)));
                assert_tokens!(
                    "-3.8",
                    Token::Operator(Operator::Neg),
                    Token::Number(Number::Float(3.8))
                );
            }

            #[test]
            fn complex_neg() {
                assert_tokens!(
                    "(-(-1))",
                    Token::Group(Group::Open),
                    Token::Operator(Operator::Neg),
                    Token::Group(Group::Open),
                    Token::Operator(Operator::Neg),
                    Token::Number(Number::Int(1)),
                    Token::Group(Group::Close),
                    Token::Group(Group::Close)
                );

                assert_tokens!(
                    "- ( 2) - (-3)",
                    Token::Operator(Operator::Neg),
                    Token::Group(Group::Open),
                    Token::Number(Number::Int(2)),
                    Token::Group(Group::Close),
                    Token::Operator(Operator::Sub),
                    Token::Group(Group::Open),
                    Token::Operator(Operator::Neg),
                    Token::Number(Number::Int(3)),
                    Token::Group(Group::Close)
                );
            }

            #[test]
            fn list_of_tokens() {
                assert_tokens!(
                    " 2 +   3",
                    Token::Number(Number::Int(2)),
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(3))
                );

                assert_tokens!(
                    " 2.3 + ( 2.9 - 3.5 * (2.1 / 2.9 - 3253252.12) + 3 ) + 212",
                    Token::Number(Number::Float(2.3)),
                    Token::Operator(Operator::Add),
                    Token::Group(Group::Open),
                    Token::Number(Number::Float(2.9)),
                    Token::Operator(Operator::Sub),
                    Token::Number(Number::Float(3.5)),
                    Token::Operator(Operator::Mul),
                    Token::Group(Group::Open),
                    Token::Number(Number::Float(2.1)),
                    Token::Operator(Operator::Div),
                    Token::Number(Number::Float(2.9)),
                    Token::Operator(Operator::Sub),
                    Token::Number(Number::Float(3253252.12)),
                    Token::Group(Group::Close),
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(3)),
                    Token::Group(Group::Close),
                    Token::Operator(Operator::Add),
                    Token::Number(Number::Int(212))
                )
            }
        }
    }

    mod parse {
        use std::cmp::Ordering;

        use super::tokens::{Group, Token};

        pub trait Parser<'token>: IntoIterator<Item = &'token Token> {}

        #[derive(Default)]
        pub struct ExprParser {
            stack: Vec<Token>,
        }

        impl<'token> IntoIterator for &'token ExprParser {
            type Item = &'token Token;

            type IntoIter = std::iter::Rev<std::slice::Iter<'token, Token>>;

            fn into_iter(self) -> Self::IntoIter {
                self.stack.iter().rev()
            }
        }

        impl<'token> Parser<'token> for &'token ExprParser {}

        impl ExprParser {
            pub fn new() -> Self {
                Default::default()
            }

            pub fn parse(&mut self, tokens_iter: &mut impl Iterator<Item = Token>) {
                let mut operator_stack = vec![];

                while let Some(token) = tokens_iter.next() {
                    match token {
                        Token::Number(_) => self.stack.push(token),
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
                        }
                        Token::Group(Group::Open) => self.parse(tokens_iter),
                        Token::Group(Group::Close) => break,
                    }
                }

                while let Some(operator) = operator_stack.pop() {
                    self.stack.push(operator);
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::grammar::tokens::{tokens_iter, Number, Operator};

            macro_rules! assert_parse {
                ($expr: literal $(, $token: expr)*) => {
                    let mut parser = ExprParser::new();
                    parser.parse(&mut tokens_iter($expr));
                    assert_eq!(parser.into_iter().collect::<Vec<_>>(), vec![$(&$token, )*])
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
        }
    }

    mod eval {
        use super::tokens::*;

        pub fn eval_parsed<'token>(tokens: &mut impl Iterator<Item = &'token Token>) -> f64 {
            let Some(token) = tokens.next() else {
                return 0.0;
            };

            match token {
                Token::Number(Number::Float(num)) => *num,
                Token::Number(Number::Int(num)) => *num as f64,
                Token::Operator(operator) => {
                    let arity = operator.arity();
                    let right_arg = if arity > 0 { eval_parsed(tokens) } else { 0.0 };
                    let left_arg = if arity > 1 { eval_parsed(tokens) } else { 0.0 };

                    match operator {
                        Operator::Neg => -right_arg,
                        Operator::Add => left_arg + right_arg,
                        Operator::Sub => left_arg - right_arg,
                        Operator::Mul => left_arg * right_arg,
                        Operator::Div => left_arg / right_arg,
                    }
                }
                _ => unreachable!("Should not provide other tokens"),
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::grammar::tokens::{Number, Operator, Token};

            macro_rules! assert_eval {
                ($result: literal $(, $expr: expr)*) => {
                    let tokens = vec![$($expr,)*];
                    assert_eq!(eval_parsed(&mut tokens.iter()), $result)
                };
            }
            #[test]
            fn empty_expr() {
                assert_eval!(0.0);
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
        }
    }

    pub fn eval(expr: &str) -> f64 {
        let mut tokens = tokens_iter(expr);
        let mut parser = ExprParser::new();
        parser.parse(&mut tokens);

        eval_parsed(&mut parser.into_iter())
    }
}

pub fn calc(expr: &str) -> f64 {
    grammar::eval(expr)
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
