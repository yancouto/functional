use std::{
    collections::{hash_map::Entry, HashMap}, fmt::{self, Debug}, hash::Hash
};

use thiserror::Error;
use vec1::Vec1;

use super::tokenizer::{Constant, TVariable, Token};

/// depth can be uniquely used to determine the expression, as it points to
/// which function binded the variable.
/// It's better than an uid as replacing values make sense. For example,
/// on (f: f f) (x: y: x y) we may end up with (y: y: y y), but each y
/// points to a different function.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Variable {
    /// How many "levels above" is the variable created
    /// If unbound, then this should be the depth of this node
    /// Depth is inside how many functions
    pub depth:    usize,
    /// Original name of the variable, may have duplicates
    pub original: TVariable,
}

impl Variable {
    fn new(depth: usize, var: TVariable) -> Self {
        Self {
            depth,
            original: var,
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}_{}", self.original, self.depth))
    }
}

#[derive(Clone)]
pub enum Node {
    Constant(Constant),
    Variable(Variable),
    Function {
        // This variable doesn't have a depth.
        variable: TVariable,
        body:     Box<Node>,
    },
    Apply {
        left:  Box<Node>,
        right: Box<Node>,
    },
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("Function must be first part of expression!")]
    FunctionInsideBody,
    #[error("Extra ':' in the input")]
    ExtraColon,
    #[error("Some subexpression '()' is empty")]
    MissingExpression,
    #[error("Parenthesis is unclosed")]
    UnclosedParenthesis,
    #[error("Extra close parenthesis")]
    ExtraCloseParenthesis,
}

/// Level represents an unclosed sequence of terms. It is represented by some
/// enveloping functions ("a: b: c:" in the beginning of the level), followed
/// by a single term (which may be the application of several terms in a row).
#[derive(Debug, Default)]
struct Level {
    prev_node:            Option<Box<Node>>,
    enveloping_functions: Vec<TVariable>,
}

impl Level {
    /// Add a new term to the right of this level, merging it with prev_node if
    /// it exists
    fn merge(&mut self, node: Box<Node>) {
        self.prev_node = if let Some(prev) = self.prev_node.take() {
            Some(Box::new(Node::Apply {
                left:  prev,
                right: node,
            }))
        } else {
            Some(node)
        };
    }

    /// Finish this level, and turn it into a single term. Fails if prev_node is
    /// None.
    fn close(mut self, bindings: &mut Bindings) -> Result<Box<Node>, ParseError> {
        let mut node = if let Some(n) = self.prev_node.take() {
            n
        } else {
            return Err(ParseError::MissingExpression);
        };
        while let Some(variable) = self.enveloping_functions.pop() {
            bindings.pop_var(variable);
            node = Box::new(Node::Function {
                variable,
                body: node,
            });
        }
        Ok(node)
    }
}

#[derive(Debug)]
struct Bindings {
    /// For each variable, at which depths it was defined
    /// Example, for (x: x: x), you would get x with depth 1 and 2
    map:       HashMap<TVariable, Vec1<usize>>,
    cur_depth: usize,
}

impl Bindings {
    fn new() -> Self {
        Self {
            map:       HashMap::new(),
            cur_depth: 0,
        }
    }

    /// Create a new variable with given name, register and return it.
    fn push_var(&mut self, name: TVariable) {
        self.cur_depth += 1;
        match self.map.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(Vec1::new(self.cur_depth));
            },
            Entry::Occupied(mut entry) => entry.get_mut().push(self.cur_depth),
        };
    }

    /// Get the depth for the variable if it was added now
    fn get_var(&mut self, name: TVariable) -> usize {
        self.cur_depth - self.map.get(&name).map(|v| *v.last()).unwrap_or(0)
    }

    fn pop_var(&mut self, name: TVariable) {
        self.cur_depth -= 1;
        match self.map.entry(name) {
            Entry::Occupied(mut entry) =>
                if entry.get_mut().pop().is_err() {
                    entry.remove();
                },
            // TODO: Add algorithm error here
            Entry::Vacant(..) => panic!("Should have entry"),
        }
    }
}

pub fn parse<T: IntoIterator<Item = Token>>(tokens: T) -> Result<Box<Node>, ParseError> {
    // Levels keep track of all the current terms being created. Opening a new parenthesis
    // means creating a new level, and closing one means merging it upward.
    let mut levels = Vec1::new(Level::default());
    let mut iter = tokens.into_iter().peekable();
    let mut bindings = Bindings::new();
    while let Some(token) = iter.next() {
        match token {
            Token::Variable(name) =>
                if iter.peek() == Some(&Token::Colon) {
                    iter.next().unwrap();
                    if levels.last().prev_node.is_some() {
                        return Err(ParseError::FunctionInsideBody);
                    }
                    bindings.push_var(name);
                    levels.last_mut().enveloping_functions.push(name);
                } else {
                    levels
                        .last_mut()
                        .merge(Box::new(Node::Variable(Variable::new(
                            bindings.get_var(name),
                            name,
                        ))));
                },
            Token::Constant(c) => {
                levels.last_mut().merge(Box::new(Node::Constant(c)));
            },
            Token::Colon => {
                return Err(ParseError::ExtraColon);
            },
            Token::OpenPar => levels.push(Level::default()),
            Token::ClosePar =>
                if let Ok(last) = levels.pop() {
                    levels.last_mut().merge(last.close(&mut bindings)?);
                } else {
                    return Err(ParseError::ExtraCloseParenthesis);
                },
        }
    }
    if levels.len() > 1 {
        return Err(ParseError::UnclosedParenthesis);
    }
    Vec::from(levels).pop().unwrap().close(&mut bindings)
}

impl Node {
    fn synthatic_eq(&self, other: &Node, cur_depth: usize) -> bool {
        match (self, other) {
            (Node::Constant(c1), Node::Constant(c2)) => c1 == c2,
            (Node::Variable(v1), Node::Variable(v2)) =>
            // Unbound vars (depth = cur_depth) need to have same char
                v1.depth == v2.depth && (v1.depth < cur_depth || v1.original == v2.original),
            (
                Node::Function { variable: _, body },
                Node::Function {
                    variable: _,
                    body: body2,
                },
            ) => body.synthatic_eq(body2, cur_depth + 1),
            (
                Node::Apply { left, right },
                Node::Apply {
                    left: left2,
                    right: right2,
                },
            ) => left.synthatic_eq(left2, cur_depth) && right.synthatic_eq(right2, cur_depth),
            _ => false,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool { self.synthatic_eq(other, 0) }
}

impl Eq for Node {}

#[cfg(test)]
pub mod test {
    use super::{super::tokenizer::tokenize, *};

    pub fn parse_ok(str: &str) -> Box<Node> { parse(tokenize(str.chars()).unwrap()).unwrap() }

    fn parse_err(str: &str) -> ParseError { parse(tokenize(str.chars()).unwrap()).unwrap_err() }

    impl From<usize> for Variable {
        fn from(depth: usize) -> Self {
            Self {
                depth,
                original: '-',
            }
        }
    }

    impl From<usize> for Box<Node> {
        fn from(depth: usize) -> Self { Box::new(Node::Variable(depth.into())) }
    }

    impl From<(usize, char)> for Box<Node> {
        fn from((depth, original): (usize, char)) -> Self {
            Box::new(Node::Variable(Variable { depth, original }))
        }
    }

    impl From<&str> for Box<Node> {
        fn from(val: &str) -> Self { Box::new(Node::Constant(val.to_string())) }
    }

    impl From<(Box<Node>, Box<Node>)> for Box<Node> {
        fn from(args: (Box<Node>, Box<Node>)) -> Self {
            Box::new(Node::Apply {
                left:  args.0,
                right: args.1,
            })
        }
    }

    impl From<((), Box<Node>)> for Box<Node> {
        fn from(args: ((), Box<Node>)) -> Self {
            Box::new(Node::Function {
                variable: '-',
                body:     args.1,
            })
        }
    }

    pub trait ConvertToNode {
        fn n(self) -> Box<Node>;
    }

    impl<T: Into<Box<Node>>> ConvertToNode for T {
        fn n(self) -> Box<Node> { self.into() }
    }

    #[test]
    fn simple() {
        assert_eq!(parse_ok("A"), Box::new(Node::Constant("A".to_string())));
        assert_eq!(parse(vec![Token::Variable('x')]).unwrap(), (0, 'x').n());
        assert_ne!(parse(vec![Token::Variable('x')]).unwrap(), (0, 'y').n());
        assert_eq!(
            parse_ok("a bc c"),
            (((0, 'a').n(), "bc".n()).n(), (0, 'c').n()).n()
        );
        assert_eq!(
            parse_ok("a b c"),
            (((0, 'a').n(), (0, 'b').n()).n(), (0, 'c').n()).n(),
        );
        assert_eq!(parse_ok("x:y:x"), ((), ((), 1.n()).n()).n());
        assert_eq!(parse_ok("x:y:y"), ((), ((), 0.n()).n()).n());
        assert_eq!(parse_ok("x:y:z"), ((), ((), (2, 'z').n()).n()).n());
        assert_eq!(parse_ok("(x: x) x"), (((), 0.n()).n(), (0, 'x').n()).n());
        assert_eq!(parse_err(""), ParseError::MissingExpression);
    }

    #[test]
    fn parenthesis() {
        assert_eq!(
            parse_ok("((x) (x: x))"),
            (Box::<Node>::from((0, 'x')), ((), 0.n()).n()).n()
        );
        assert_eq!(parse_ok("a b c"), parse_ok("((a b) c)"));
        assert_eq!(parse_err("(a b ())"), ParseError::MissingExpression);
        assert_eq!(parse_err("a)"), ParseError::ExtraCloseParenthesis);
        assert_eq!(parse_err("a (b c"), ParseError::UnclosedParenthesis);
    }

    #[test]
    fn test_eq() {
        assert_eq!((0, 'x').n(), (0, 'x').n());
        assert_ne!((0, 'x').n(), (0, 'y').n());
        assert_ne!("ab".n(), "hi".n());
        // (x: x) == (y: y)
        assert_eq!(((), (0, 'x').n()).n(), ((), (0, 'y').n()).n());
        // (x: z) != (x: y)
        assert_ne!(((), (1, 'z').n()).n(), ((), (1, 'y').n()).n());
    }

    #[test]
    fn parse_eq() {
        assert_eq!(parse_ok("(x: x) x"), parse_ok("(y: y) x"));
        assert_eq!(parse_ok("(x: x) (x: x)"), parse_ok("(y: y) (z :z)"));
        assert_eq!(parse_ok("(x: x y) (z: z y)"), parse_ok("(x: x y) (x: x y)"));
        assert_ne!(parse_ok("(x: x y) (z: z y)"), parse_ok("(x: x y) (z: z a)"));
        assert_ne!(parse_ok("(x: x x)"), parse_ok("(x: x y)"));
        // Variable names may be reused if they're bound, and it still works
        assert_eq!(
            ((), ((), (0, 'x').n()).n()).n(),
            ((), ((), (0, 'y').n()).n()).n()
        );
        assert_ne!(((), ((), 0.n()).n()).n(), ((), ((), 1.n()).n()).n());
        assert_eq!(
            parse_ok("(x: x) (x: x)"),
            (((), 0.n()).n(), ((), 0.n()).n()).n(),
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
