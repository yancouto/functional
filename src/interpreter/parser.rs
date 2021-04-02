use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{self, Debug},
    hash::Hash,
};
use thiserror::Error;

use super::tokenizer::{Constant, TVariable, Token};
use vec1::Vec1;

/// uid is used to identify variables in different bindings. For example, in
/// ((x: x) (x: x x)), the x's on both functions will have different uids, but
/// on the same function they will have the same uid.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Variable {
    /// Unique number between all variables in the term
    uid: u32,
    /// Original name of the variable, may have duplicates
    original: TVariable,
}

impl Variable {
    fn new(uid: &mut u32, var: TVariable) -> Self {
        *uid += 1;
        Self {
            uid: *uid - 1,
            original: var,
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}_{}", self.original, self.uid))
    }
}

#[derive(Clone)]
pub enum Node {
    Constant(Constant),
    Variable(Variable),
    Function { variable: Variable, body: Box<Node> },
    Apply { left: Box<Node>, right: Box<Node> },
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Constant(c) => f.write_str(c),
            Node::Variable(v) => v.fmt(f),
            Node::Function { variable, body } => {
                f.write_fmt(format_args!("({:?}: {:?})", variable, body))
            }
            Node::Apply { left, right } => f.write_fmt(format_args!("({:?} {:?})", left, right)),
        }
    }
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
    prev_node: Option<Box<Node>>,
    enveloping_functions: Vec<Variable>,
}

impl Level {
    /// Add a new term to the right of this level, merging it with prev_node if
    /// it exists
    fn merge(&mut self, node: Box<Node>) {
        self.prev_node = if let Some(prev) = self.prev_node.take() {
            Some(Box::new(Node::Apply {
                left: prev,
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
            bindings.pop_var(variable.original);
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
    map: HashMap<TVariable, Vec1<Variable>>,
    num_vars: u32,
}

impl Bindings {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            num_vars: 0,
        }
    }

    /// Create a new variable with given name, register and return it.
    fn push_var(&mut self, name: TVariable) -> Variable {
        let var = Variable::new(&mut self.num_vars, name);
        match self.map.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(Vec1::new(var));
            }
            Entry::Occupied(mut entry) => entry.get_mut().push(var),
        };
        var
    }

    /// Get given variable, creating a new one if it doesn't exist (that will
    /// happen only if it is a free variable)
    fn get_var(&mut self, name: TVariable) -> Variable {
        // Need to borrow this outside for compiler to understand we're not double borrowing
        let num_vars = &mut self.num_vars;
        self.map
            .entry(name)
            .or_insert_with(|| Vec1::new(Variable::new(num_vars, name)))
            .last()
            .clone()
    }

    fn pop_var(&mut self, name: TVariable) {
        match self.map.entry(name) {
            Entry::Occupied(mut entry) => {
                if entry.get_mut().pop().is_err() {
                    entry.remove();
                }
            }
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
            Token::Variable(name) => {
                if iter.peek() == Some(&Token::Colon) {
                    iter.next().unwrap();
                    if levels.last().prev_node.is_some() {
                        return Err(ParseError::FunctionInsideBody);
                    }
                    levels
                        .last_mut()
                        .enveloping_functions
                        .push(bindings.push_var(name));
                } else {
                    levels
                        .last_mut()
                        .merge(Box::new(Node::Variable(bindings.get_var(name))));
                }
            }
            Token::Constant(c) => {
                levels.last_mut().merge(Box::new(Node::Constant(c)));
            }
            Token::Colon => {
                return Err(ParseError::ExtraColon);
            }
            Token::OpenPar => levels.push(Level::default()),
            Token::ClosePar => {
                if let Ok(last) = levels.pop() {
                    levels.last_mut().merge(last.close(&mut bindings)?);
                } else {
                    return Err(ParseError::ExtraCloseParenthesis);
                }
            }
        }
    }
    if levels.len() > 1 {
        return Err(ParseError::UnclosedParenthesis);
    }
    Vec::from(levels).pop().unwrap().close(&mut bindings)
}

#[derive(Debug, Default)]
struct OneToOne<A: Eq + Hash, B: Eq + Hash> {
    left_to_right: HashMap<A, B>,
    right_to_left: HashMap<B, A>,
}

impl<A: Eq + Hash + Copy, B: Eq + Hash + Copy> OneToOne<A, B> {
    fn check(&mut self, a: A, b: B) -> bool {
        *self.left_to_right.entry(a).or_insert(b) == b
            && *self.right_to_left.entry(b).or_insert(a) == a
    }

    fn with_added_eq<F: FnOnce(&mut Self) -> R, R>(&mut self, a: A, b: B, f: F) -> R {
        let prev_b = self.left_to_right.insert(a, b);
        let prev_a = self.right_to_left.insert(b, a);
        let r = f(self);
        match prev_b {
            Some(b) => self.left_to_right.insert(a, b),
            None => self.left_to_right.remove(&a),
        }
        .unwrap();
        match prev_a {
            Some(a) => self.right_to_left.insert(b, a),
            None => self.right_to_left.remove(&b),
        }
        .unwrap();
        r
    }
}

impl Node {
    fn synthatic_eq(&self, other: &Node, var_eqs: &mut OneToOne<u32, u32>) -> bool {
        match (self, other) {
            (Node::Constant(c1), Node::Constant(c2)) => c1 == c2,
            (Node::Variable(v1), Node::Variable(v2)) => var_eqs.check(v1.uid, v2.uid),
            (
                Node::Function { variable, body },
                Node::Function {
                    variable: variable2,
                    body: body2,
                },
            ) => var_eqs.with_added_eq(variable.uid, variable2.uid, |vars| {
                body.synthatic_eq(body2, vars)
            }),
            (
                Node::Apply { left, right },
                Node::Apply {
                    left: left2,
                    right: right2,
                },
            ) => left.synthatic_eq(left2, var_eqs) && right.synthatic_eq(right2, var_eqs),
            _ => false,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.synthatic_eq(other, &mut OneToOne::default())
    }
}

impl Eq for Node {}

#[cfg(test)]
pub mod test {
    use super::super::tokenizer::tokenize;
    use super::*;

    pub fn parse_ok(str: &str) -> Box<Node> {
        parse(tokenize(str.chars()).unwrap()).unwrap()
    }

    fn parse_err(str: &str) -> ParseError {
        parse(tokenize(str.chars()).unwrap()).unwrap_err()
    }

    impl From<u32> for Variable {
        fn from(uid: u32) -> Self {
            Self { uid, original: '-' }
        }
    }

    impl From<u32> for Box<Node> {
        fn from(uid: u32) -> Self {
            Box::new(Node::Variable(uid.into()))
        }
    }

    impl From<&str> for Box<Node> {
        fn from(val: &str) -> Self {
            Box::new(Node::Constant(val.to_string()))
        }
    }

    impl From<(Box<Node>, Box<Node>)> for Box<Node> {
        fn from(args: (Box<Node>, Box<Node>)) -> Self {
            Box::new(Node::Apply {
                left: args.0,
                right: args.1,
            })
        }
    }

    impl From<(u32, Box<Node>)> for Box<Node> {
        fn from(args: (u32, Box<Node>)) -> Self {
            Box::new(Node::Function {
                variable: args.0.into(),
                body: args.1,
            })
        }
    }

    trait ConvertToNode {
        fn n(self) -> Box<Node>;
    }

    impl<T: Into<Box<Node>>> ConvertToNode for T {
        fn n(self) -> Box<Node> {
            self.into()
        }
    }

    #[test]
    fn simple() {
        assert_eq!(parse_ok("A"), Box::new(Node::Constant("A".to_string())));
        assert_eq!(parse(vec![Token::Variable('x')]).unwrap(), 12.into(),);
        assert_eq!(parse_ok("a bc c"), ((0.n(), "bc".n()).n(), 2.n()).n());
        assert_eq!(parse_ok("a b c"), ((1.n(), 3.n()).n(), 13.n()).n(),);
        assert_eq!(parse_ok("x:y:x"), (0, (1, 0.n()).n()).n());
        assert_eq!(parse_err(""), ParseError::MissingExpression);
    }

    #[test]
    fn parenthesis() {
        assert_eq!(
            parse_ok("((x) (x: x))"),
            (Box::<Node>::from(9), (3, 3.n()).n()).n()
        );
        assert_eq!(parse_ok("a b c"), parse_ok("((a b) c)"));
        assert_eq!(parse_err("(a b ())"), ParseError::MissingExpression);
        assert_eq!(parse_err("a)"), ParseError::ExtraCloseParenthesis);
        assert_eq!(parse_err("a (b c"), ParseError::UnclosedParenthesis);
    }

    #[test]
    fn test_eq() {
        assert_eq!(12.n(), 2.n());
        assert_ne!("ab".n(), "hi".n());
        // (x y) == (z w)
        assert_eq!((0.n(), 0.n()).n(), (1.n(), 1.n()).n());
        // (x y) != (x x)
        assert_ne!((0.n(), 1.n()).n(), (0.n(), 0.n()).n());
        assert_ne!((0.n(), 0.n()).n(), (0.n(), 1.n()).n());
    }

    #[test]
    fn parse_eq() {
        assert_eq!(parse_ok("(x: x) x"), parse_ok("(y: y) z"));
        assert_eq!(parse_ok("(x: x) (x: x)"), parse_ok("(y: y) (z :z)"));
        assert_eq!(parse_ok("(x: x y) (z: z y)"), parse_ok("(x: x z) (y: y z)"));
        assert_ne!(parse_ok("(x: x y) (z: z y)"), parse_ok("(x: x z) (z: z n)"));
        assert_ne!(parse_ok("(x: x x)"), parse_ok("(x: x y)"));
        // Variable names may be reused if they're bound, and it still works
        assert_eq!((0, (0, 0.n()).n()).n(), (0, (1, 1.n()).n()).n(),);
        assert_eq!(
            parse_ok("(x: x) (x: x)"),
            ((0, 0.n()).n(), (0, 0.n()).n()).n(),
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
