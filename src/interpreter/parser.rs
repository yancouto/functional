use super::tokenizer::{Constant, Token, Variable};
use non_empty_vec::NonEmpty;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Constant(Constant),
    Variable(Variable),
    Function { variable: Variable, body: Box<Node> },
    Apply { left: Box<Node>, right: Box<Node> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    FunctionInsideBody,
    ExtraColon,
    MissingExpression,
    UnclosedParenthesis,
    ExtraCloseParenthesis,
}

#[derive(Default)]
struct Level {
    prev_node: Option<Box<Node>>,
    enveloping_functions: Vec<Variable>,
}

impl Level {
    fn close(mut self) -> Result<Box<Node>, ParseError> {
        let mut node = if let Some(n) = self.prev_node.take() {
            n
        } else {
            return Err(ParseError::MissingExpression);
        };
        while let Some(variable) = self.enveloping_functions.pop() {
            node = Box::new(Node::Function {
                variable,
                body: node,
            });
        }
        Ok(node)
    }
}

pub fn parse<T: IntoIterator<Item = Token>>(tokens: T) -> Result<Box<Node>, ParseError> {
    let mut levels = NonEmpty::from((Level::default(), vec![]));
    let merge = |l: &mut Level, n: Box<Node>| {
        l.prev_node = if let Some(prev) = l.prev_node.take() {
            Some(Box::new(Node::Apply {
                left: prev,
                right: n,
            }))
        } else {
            Some(n)
        };
    };
    let mut iter = tokens.into_iter().peekable();
    while let Some(token) = iter.next() {
        match token {
            Token::Variable(v) => {
                if iter.peek() == Some(&Token::Colon) {
                    iter.next().unwrap();
                    if levels.last().prev_node.is_some() {
                        return Err(ParseError::FunctionInsideBody);
                    }
                    levels.last_mut().enveloping_functions.push(v);
                } else {
                    merge(levels.last_mut(), Box::new(Node::Variable(v)));
                }
            }
            Token::Constant(c) => {
                merge(levels.last_mut(), Box::new(Node::Constant(c)));
            }
            Token::Colon => {
                return Err(ParseError::ExtraColon);
            }
            Token::OpenPar => levels.push(Level::default()),
            Token::ClosePar => {
                if let Some(last) = levels.pop() {
                    merge(levels.last_mut(), last.close()?);
                } else {
                    return Err(ParseError::ExtraCloseParenthesis);
                }
            }
        }
    }
    if levels.len().get() > 1 {
        return Err(ParseError::UnclosedParenthesis);
    }
    Vec::from(levels).pop().unwrap().close()
}

#[cfg(test)]
mod test {
    use super::super::tokenizer::tokenize;
    use super::*;

    fn parse_ok(str: &str) -> Box<Node> {
        parse(tokenize(str.chars()).unwrap()).unwrap()
    }

    fn parse_err(str: &str) -> ParseError {
        parse(tokenize(str.chars()).unwrap()).unwrap_err()
    }

    #[test]
    fn simple() {
        assert_eq!(
            parse(vec![Token::Variable('x')]).unwrap(),
            Box::new(Node::Variable('x'))
        );
        assert_eq!(
            parse_ok("a b c"),
            Box::new(Node::Apply {
                left: Box::new(Node::Apply {
                    left: Box::new(Node::Variable('a')),
                    right: Box::new(Node::Variable('b')),
                }),
                right: Box::new(Node::Variable('c'))
            })
        );
        assert_eq!(
            parse_ok("x:y:x"),
            Box::new(Node::Function {
                variable: 'x',
                body: Box::new(Node::Function {
                    variable: 'y',
                    body: Box::new(Node::Variable('x'))
                })
            })
        );
        assert_eq!(parse_err(""), ParseError::MissingExpression);
    }

    #[test]
    fn parenthesis() {
        assert_eq!(
            parse_ok("((x) (x: x))"),
            Box::new(Node::Apply {
                left: Box::new(Node::Variable('x')),
                right: Box::new(Node::Function {
                    variable: 'x',
                    body: Box::new(Node::Variable('x'))
                })
            })
        );
        assert_eq!(parse_ok("a b c"), parse_ok("((a b) c)"));
        assert_eq!(parse_err("(a b ())"), ParseError::MissingExpression);
        assert_eq!(parse_err("a)"), ParseError::ExtraCloseParenthesis);
        assert_eq!(parse_err("a (b c"), ParseError::UnclosedParenthesis);
    }

    #[test]
    fn some_errors() {
        assert_eq!(
            parse(vec![Token::Colon]).unwrap_err(),
            ParseError::ExtraColon
        );
        assert_eq!(parse_err("a: b c: d"), ParseError::FunctionInsideBody);
        assert_eq!(parse_err("x:"), ParseError::MissingExpression);
        assert_eq!(parse_err("x: : y"), ParseError::ExtraColon);
    }
}
