use anyhow::Result;
use itertools::Itertools;

pub fn part1(input: String) -> Result<usize> {
    Ok(input.lines().map(evaluate).sum())
}

pub fn part2(input: String) -> Result<usize> {
    Ok(input
        .lines()
        .map(|l| {
            let toks: Vec<Token> = Lexer::new(l).collect();
            evaluate_with_precedence(&toks)
        })
        .sum())
}

fn evaluate(expr: &str) -> usize {
    let lex = Lexer::new(expr);
    let mut stack = ExprStack::new();
    for tok in lex {
        stack.push(tok);
    }
    stack.value()
}

fn evaluate_with_precedence(toks: &[Token]) -> usize {
    let mut flat_expr = Vec::new();
    let mut depth = 0;
    let mut start = None;
    for (i, t) in toks.iter().cloned().enumerate() {
        match t {
            Token::ParenOpen => {
                if depth == 0 {
                    start = Some(i + 1);
                }
                depth += 1;
            }
            Token::ParenClose => {
                assert!(depth != 0);
                depth -= 1;
                if depth == 0 {
                    flat_expr.push(Token::Number(evaluate_with_precedence(
                        &toks[start.unwrap()..i],
                    )));
                    start = None;
                }
            }
            _ => {
                if depth == 0 {
                    flat_expr.push(t)
                }
            }
        }
    }

    let mut iter = flat_expr.into_iter().peekable();

    let mut iter = std::iter::from_fn(move || {
        let tok = iter.next()?;
        match tok {
            Token::Number(mut num) => {
                while iter.peek() == Some(&Token::OpAdd) {
                    iter.next();
                    match iter.next() {
                        Some(Token::Number(oth)) => num += oth,
                        _ => panic!("invalid expr"),
                    }
                }
                Some(Token::Number(num))
            }
            _ => Some(tok),
        }
    })
    .peekable();

    let iter = std::iter::from_fn(move || {
        let tok = iter.next()?;
        match tok {
            Token::Number(mut num) => {
                while iter.peek() == Some(&Token::OpMul) {
                    iter.next();
                    match iter.next() {
                        Some(Token::Number(oth)) => num *= oth,
                        _ => panic!("invalid expr"),
                    }
                }
                Some(Token::Number(num))
            }
            _ => Some(tok),
        }
    });

    match iter.exactly_one() {
        Ok(Token::Number(num)) => num,
        _ => panic!("invalid expr"),
    }
}

struct ExprStack(Vec<Token>);

impl ExprStack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn peek(&self) -> Option<Token> {
        self.0.last().cloned()
    }

    fn value(self) -> usize {
        assert_eq!(1, self.0.len());
        match self.0[0] {
            Token::Number(val) => val,
            _ => panic!("invalid expr"),
        }
    }

    fn push(&mut self, tok: Token) {
        match tok {
            Token::ParenOpen | Token::OpMul | Token::OpAdd => self.0.push(tok),
            Token::Number(val) => match self.peek() {
                None | Some(Token::ParenOpen) => self.0.push(Token::Number(val)),
                Some(Token::OpAdd) | Some(Token::OpMul) => {
                    let op = self.0.pop().unwrap();
                    let oth = match self.0.pop().unwrap() {
                        Token::Number(oth) => oth,
                        _ => panic!("invalid expr"),
                    };
                    let res = match op {
                        Token::OpAdd => val + oth,
                        Token::OpMul => val * oth,
                        _ => unreachable!(),
                    };
                    self.0.push(Token::Number(res));
                }
                _ => panic!("invalid expr"),
            },
            Token::ParenClose => {
                let val = match self.0.pop().unwrap() {
                    Token::Number(val) => val,
                    _ => panic!("invalid expr"),
                };
                assert_eq!(Token::ParenOpen, self.0.pop().unwrap());
                self.push(Token::Number(val));
            }
        }
    }
}

struct Lexer<'a> {
    expr: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(expr: &'a str) -> Self {
        Self { expr }
    }

    fn _next_token(&self) -> Option<(Token, usize)> {
        let (start, c) = self
            .expr
            .char_indices()
            .find(|&(_, c)| !c.is_whitespace())?;

        if c.is_digit(10) {
            let end = (&self.expr[start..])
                .char_indices()
                .find(|&(_, c)| !c.is_digit(10))
                .map(|(i, _)| i + start)
                .unwrap_or(self.expr.len());

            let num = self.expr[start..end].parse::<usize>().ok()?;
            return Some((Token::Number(num), end));
        }

        let tok = match c {
            '+' => Token::OpAdd,
            '*' => Token::OpMul,
            '(' => Token::ParenOpen,
            ')' => Token::ParenClose,
            _ => return None,
        };
        Some((tok, start + 1))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (tok, len) = self._next_token()?;
        self.expr = &self.expr[len..];
        Some(tok)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(usize),
    OpAdd,
    OpMul,
    ParenOpen,
    ParenClose,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = "1 + 2 * 3 + 4 * 5 + 6";
    const EXAMPLE2: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    const EXAMPLE3: &str = "2 * 3 + (4 * 5)";
    const EXAMPLE4: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const EXAMPLE5: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const EXAMPLE6: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn expression_lexer() {
        let mut lexer = Lexer::new(EXAMPLE3);

        assert_eq!(Some(Token::Number(2)), lexer.next());
        assert_eq!(Some(Token::OpMul), lexer.next());
        assert_eq!(Some(Token::Number(3)), lexer.next());
        assert_eq!(Some(Token::OpAdd), lexer.next());
        assert_eq!(Some(Token::ParenOpen), lexer.next());
        assert_eq!(Some(Token::Number(4)), lexer.next());
        assert_eq!(Some(Token::OpMul), lexer.next());
        assert_eq!(Some(Token::Number(5)), lexer.next());
        assert_eq!(Some(Token::ParenClose), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn evaluate_examples() {
        assert_eq!(71, evaluate(EXAMPLE1));
        assert_eq!(51, evaluate(EXAMPLE2));
        assert_eq!(26, evaluate(EXAMPLE3));
        assert_eq!(437, evaluate(EXAMPLE4));
        assert_eq!(12240, evaluate(EXAMPLE5));
        assert_eq!(13632, evaluate(EXAMPLE6));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(231, part2(EXAMPLE1.to_string()).unwrap());
    }
}
