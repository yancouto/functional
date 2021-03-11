use super::tokenizer::{Constant, Token, Variable};

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
}

struct Level {
    prev_node: Option<Box<Node>>,
    enveloping_functions: Vec<Variable>,
}

pub fn parse<T: IntoIterator<Item = Token>>(tokens: T) -> Result<Box<Node>, ParseError> {
    let mut level = Level {
        prev_node: None,
        enveloping_functions: vec![],
    };
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
                    if level.prev_node.is_some() {
                        return Err(ParseError::FunctionInsideBody);
                    }
                    level.enveloping_functions.push(v);
                } else {
                    merge(&mut level, Box::new(Node::Variable(v)));
                }
            }
            Token::Constant(c) => {
                merge(&mut level, Box::new(Node::Constant(c)));
            }
            Token::Colon => {
                return Err(ParseError::ExtraColon);
            }
            _ => todo!(),
        }
    }
    let mut node = if let Some(n) = level.prev_node.take() {
        n
    } else {
        return Err(ParseError::MissingExpression);
    };
    while let Some(variable) = level.enveloping_functions.pop() {
        node = Box::new(Node::Function {
            variable,
            body: node,
        });
    }
    Ok(node)
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
