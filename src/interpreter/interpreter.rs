use std::ops::{Generator, GeneratorState};

use thiserror::Error;

use super::{parser::Node, ConstantProvider};
use crate::levels::SectionName;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum InterpretError {
    #[error("The interpreter algorithm is wrong, this shouldn't happen.")]
    AlgorithmError,
    #[error("The interpretation has gone too deep, expression probably has no reduction.")]
    TooDeep,
}

trait AlgorithmAssert<T> {
    fn assert_some(self) -> Result<T, InterpretError>;
    fn assert_none(self) -> Result<(), InterpretError>;
}

impl<T> AlgorithmAssert<T> for Option<T> {
    fn assert_some(self) -> Result<T, InterpretError> { self.ok_or(InterpretError::AlgorithmError) }

    fn assert_none(self) -> Result<(), InterpretError> {
        if self.is_some() {
            Err(InterpretError::AlgorithmError)
        } else {
            Ok(())
        }
    }
}

const MAX_LEVEL: usize = 100;

use std::pin::Pin;

fn for_each_unbound_req<F: Fn(&mut usize) -> () + Copy>(root: &mut Node, cur_depth: usize, f: F) {
    match root {
        Node::Constant(_) => {},
        Node::Variable(v) =>
        // unbound variables in the root expression, not necessarily in the whole expression
        // for example, on (x:y: x), x is considered unbound in the subterm (y: x).
            if v.depth >= cur_depth {
                f(&mut v.depth);
            },
        Node::Function { variable: _, body } => {
            for_each_unbound_req(body, cur_depth + 1, f);
        },
        Node::Apply { left, right } => {
            for_each_unbound_req(left, cur_depth, f);
            for_each_unbound_req(right, cur_depth, f);
        },
    }
}

fn replace_req(root: Box<Node>, cur_depth: usize, value: Box<Node>) -> Box<Node> {
    box match *root {
        Node::Variable(mut v) =>
            if v.depth == cur_depth {
                let mut value = value.clone();
                // We need to increase the depth for unbound vars so they keep being unbound
                for_each_unbound_req(value.as_mut(), 0, |depth| *depth += cur_depth);
                *value
            } else {
                if v.depth > cur_depth {
                    // for variables that are "unbound" in the root note (may be bound before)
                    // we need to decrease depth by one
                    v.depth -= 1;
                }
                Node::Variable(v)
            },
        Node::Function { variable, body } => Node::Function {
            variable,
            body: replace_req(body, cur_depth + 1, value),
        },
        Node::Apply { left, right } => Node::Apply {
            left:  replace_req(left, cur_depth, value.clone()),
            right: replace_req(right, cur_depth, value.clone()),
        },
        node @ Node::Constant(_) => node,
    }
}

macro_rules! yield_from {
    ($x: expr, $f: expr) => {{
        let mut gen = $x;
        loop {
            match gen.as_mut().resume(()) {
                GeneratorState::Yielded(y) => yield $f(y),
                GeneratorState::Complete(r) => break r,
            }
        }
    }};
    ($x: expr) => {
        yield_from!($x, |x| x)
    };
}

#[derive(Debug, Clone, Copy)]
struct Interpreter {
    fully_resolve:       bool,
    yield_intermediates: bool,
    provider:            ConstantProvider,
}

type InterpretResult =
    Pin<Box<dyn Generator<Yield = Box<Node>, Return = Result<Box<Node>, InterpretError>>>>;

impl Interpreter {
    fn interpret(self, level: usize, root: Box<Node>, do_apply: bool) -> InterpretResult {
        Box::pin(move || {
            if level > MAX_LEVEL {
                return Err(InterpretError::TooDeep);
            }
            Ok(match *root {
                Node::Apply { left, right } => {
                    let left = yield_from!(self.interpret(level + 1, left, do_apply), |left| {
                        box Node::Apply {
                            left,
                            right: right.clone(),
                        }
                    })?;
                    let right = yield_from!(
                        self.interpret(level + 1, right, self.fully_resolve),
                        |right| box Node::Apply {
                            left: left.clone(),
                            right
                        }
                    )?;
                    match *left {
                        Node::Function { variable: _, body } if do_apply => {
                            let body = replace_req(body, 0, right);
                            if self.yield_intermediates {
                                yield body.clone();
                            }
                            yield_from!(self.interpret(level + 1, body, true))?
                        },
                        _ => box Node::Apply { left, right },
                    }
                },
                Node::Variable(v) => box Node::Variable(v),
                Node::Function { variable, body } => {
                    let inner = yield_from!(
                        self.interpret(level + 1, body, self.fully_resolve),
                        |inner| box Node::Function {
                            variable,
                            body: inner
                        }
                    )?;
                    Box::new(Node::Function {
                        variable,
                        body: inner,
                    })
                },
                Node::Constant(c) => self
                    .provider
                    // No need to interpret constants here... they should be fully reduced
                    .get(&c)
                    .unwrap_or_else(|| box Node::Constant(c)),
            })
        })
    }
}

pub fn interpret(root: Box<Node>, fully_resolve: bool) -> Result<Box<Node>, InterpretError> {
    let mut gen = Interpreter {
        fully_resolve,
        yield_intermediates: false,
        // TODO: Not use all constants all the time
        provider: ConstantProvider::new((SectionName::Boolean, 100)),
    }
    .interpret(0, root, true);
    loop {
        match gen.as_mut().resume(()) {
            GeneratorState::Yielded(_) => {
                debug_assert!(false, "yield_intermediates is set to false")
            },
            GeneratorState::Complete(ret) => break ret,
        }
    }
}

struct InterpretIter {
    gen:      InterpretResult,
    finished: bool,
}

impl Iterator for InterpretIter {
    type Item = Box<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            match self.gen.as_mut().resume(()) {
                GeneratorState::Yielded(y) => Some(y),
                GeneratorState::Complete(_) => {
                    self.finished = true;
                    None
                },
            }
        }
    }
}

pub fn interpret_itermediates(
    root: Box<Node>,
    fully_resolve: bool,
) -> impl Iterator<Item = Box<Node>> {
    InterpretIter {
        gen:      Interpreter {
            fully_resolve,
            yield_intermediates: true,
            provider: ConstantProvider::new((SectionName::Boolean, 100)),
        }
        .interpret(0, root, true),
        finished: false,
    }
}

#[cfg(test)]
pub mod test {
    use super::{
        super::parser::test::{parse_ok, ConvertToNode}, *
    };

    const Y_COMB: &str = "(f: (x: f (x x)) (x: f (x x)))";

    fn interpret_lazy(root: Box<Node>) -> Result<Box<Node>, InterpretError> {
        interpret(root, false)
    }

    pub fn interpret_ok(str: &str) -> Box<Node> { interpret_lazy(parse_ok(str)).unwrap() }

    fn interpret_err(str: &str) -> InterpretError { interpret_lazy(parse_ok(str)).unwrap_err() }

    pub fn interpret_ok_full(str: &str, fully_resolve: bool) -> Box<Node> {
        interpret(parse_ok(str), fully_resolve).unwrap()
    }

    fn interpret_eq_full(src: &str, expected: &str, fully_resolve: bool) {
        assert_eq!(
            interpret_ok_full(src, fully_resolve),
            interpret_ok_full(expected, fully_resolve)
        );
    }

    fn interpret_eq(src: &str, expected: &str) {
        assert_eq!(interpret_ok(src), parse_ok(expected));
    }

    #[test]
    fn no_apply() {
        interpret_eq("z", "z");
        interpret_eq("y:y", "y : y");
        interpret_eq("y:y", "x : x");
        interpret_eq("y:hello", "z : hello");
        // unbound vars are not equal
        assert_ne!(interpret_ok("a"), interpret_ok("b"));
        assert_ne!(interpret_ok("y:hello"), parse_ok("y:other"));
    }

    #[test]
    fn simple_apply() { interpret_eq("(x: x) z", "z"); }
    #[test]
    fn more_apply() {
        interpret_eq("(x: x x) (y z)", "(y z) (y z)");
        interpret_eq("(x: x x) (y: y)", "(y: y)");
        interpret_eq("(x: x x) (y: z)", "z");
        interpret_eq("(x: x x x) (x: x)", "y: y");
        interpret_eq_full("((x:x)(y:y))(z:z)", "x:x", true);
    }

    #[test]
    fn tricky1() {
        // easy, no name conflicts
        interpret_eq("(x: y: x) z", "y: z");
        interpret_eq("(x: y: x) w z", "w");
        // hard, name conflicts
        interpret_eq("(x: y: x) y", "z: y");
        interpret_eq("(x: y: x) y z", "y");
    }
    #[test]
    fn tricky2() {
        interpret_eq("(x: x) (y: x)", "y: x");
        interpret_eq("(x: x x) (y: x)", "x");
    }

    #[test]
    // Problems regarding conflicting variable names
    fn tricky3() {
        // Issue is we'll copy the function twice, with same var names, and at the end we'll have
        // something like (y: y': y y'), which is very easy to interpret as (y: y: y y) which is wrong
        interpret_eq_full("(f: f f) (x: y: x y)", "(x: y: x y)", true);
        // (0: 1: 0 1) 1
        // Using pure expressions to create the conflict in var uids that might
        // come from e.g. concatenating terms
        let expr = (((), ((), (1.n(), 0.n()).n()).n()).n(), (0, 'z').n()).n();
        assert_eq!(interpret(expr, false).unwrap(), parse_ok("x: z x"));
        // No variable conflicts when replacing
        let ex = "y: (x: y: x y) y";
        interpret_eq_full(ex, "y: z: y z", true);
        interpret_eq_full(&format!("({}) A B", ex), "A B", true);
        interpret_eq_full(&format!("({}) A B", ex), "A B", false);
        // Display can't reuse variable names for different vars
        assert_eq!(format!("{}", interpret_ok_full(ex, true)), "y: y': y y'");
    }

    #[test]
    fn tricky4() {
        interpret_eq_full("(a: b: a) A B", "A", true);
        interpret_eq_full("(x: y: (a: b: a) x)", "(x: y: b: x)", true);
        interpret_eq_full("(b: x: y: b x y) (a: b: a) A B", "A", true);
    }

    #[test]
    fn infinite() {
        assert_eq!(interpret_err("(x: x x) (y: y y)"), InterpretError::TooDeep);
        assert_eq!(interpret_err("(x: x x) (x: x x)"), InterpretError::TooDeep);
        assert_eq!(
            interpret_err("(x: x x x) (y: y y)"),
            InterpretError::TooDeep
        );
        assert_eq!(
            interpret(parse_ok("(x: z) ((x: x x) (x: x x))"), true).unwrap_err(),
            InterpretError::TooDeep
        );
    }

    #[test]
    fn actually_not_infinite() {
        interpret_eq("(x: z) ((x: x x) (x: x x))", "z");
        interpret_eq(&format!("({} (f: x:y: y x)) a b", Y_COMB), "b a");
    }

    #[test]
    fn trick_eq() {
        // This may error if we're copying variables incorrectly, since the replacement
        // will have two functions with the same variable. This is not really a problem
        // but the eq checker might fail.
        interpret_eq("(x: w x x) (x: x)", "w (x: x) (x:x)");
        interpret_eq("(x: w x x) (y: z)", "w (a: z) (c:z)");
    }

    #[test]
    fn recursive() {
        assert_eq!(
            interpret_err(&format!("({} (f: x:y: f x y)) a b", Y_COMB)),
            InterpretError::TooDeep
        );
    }

    #[test]
    fn some_levels() { interpret_eq_full("(f: x: f (f x)) (x: x x) A", "(A A) (A A)", true); }

    fn assert_partial(code: &str, intermediates: Vec<&str>) {
        assert_eq!(
            interpret_itermediates(parse_ok(code), false).collect::<Vec<_>>(),
            intermediates
                .into_iter()
                .map(|e| parse_ok(e))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn partial() { assert_partial("(x: x x) (y: z)", vec!["(y: z) (y:z)", "z"]); }

    #[test]
    fn test_constants() {
        interpret_eq("TRUE A B", "A");
        interpret_eq("FALSE A B", "B");
        interpret_eq_full("(f:a:b: f b a) FALSE", "TRUE", true);
    }
}
