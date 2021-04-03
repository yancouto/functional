use std::ops::{Generator, GeneratorState};

use thiserror::Error;

use super::parser::{Node, Variable};

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

fn replace_req(root: Box<Node>, var: Variable, value: Box<Node>) -> Box<Node> {
    box match *root {
        Node::Variable(v) =>
            if v == var {
                *value.clone()
            } else {
                Node::Variable(v)
            },
        Node::Function { variable, body } =>
            if variable == var {
                Node::Function { variable, body }
            } else {
                Node::Function {
                    variable,
                    body: replace_req(body, var, value),
                }
            },
        Node::Apply { left, right } => Node::Apply {
            left:  replace_req(left, var, value.clone()),
            right: replace_req(right, var, value.clone()),
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
                        Node::Function { variable, body } if do_apply => {
                            let body = replace_req(body, variable, right);
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
                node @ Node::Constant(..) => box node,
            })
        })
    }
}

pub fn interpret(root: Box<Node>, fully_resolve: bool) -> Result<Box<Node>, InterpretError> {
    let mut gen = Interpreter {
        fully_resolve,
        yield_intermediates: false,
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
        }
        .interpret(0, root, true),
        finished: false,
    }
}

#[cfg(test)]
mod test {
    use super::{super::parser::test::parse_ok, *};

    const Y_COMB: &str = "(f: (x: f (x x)) (x: f (x x)))";

    fn interpret_lazy(root: Box<Node>) -> Result<Box<Node>, InterpretError> {
        interpret(root, false)
    }

    fn interpret_ok(str: &str) -> Box<Node> { interpret_lazy(parse_ok(str)).unwrap() }

    fn interpret_err(str: &str) -> InterpretError { interpret_lazy(parse_ok(str)).unwrap_err() }

    fn interpret_eq_full(src: &str, expected: &str, fully_resolve: bool) {
        assert_eq!(
            interpret(parse_ok(src), fully_resolve).unwrap(),
            parse_ok(expected)
        );
    }

    fn interpret_eq(src: &str, expected: &str) {
        assert_eq!(interpret_ok(src), parse_ok(expected));
    }

    #[test]
    fn no_apply() {
        interpret_eq("z", "x");
        interpret_eq("y:y", "y : y");
        interpret_eq("y:hello", "z : hello");
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
    fn tricky2() { interpret_eq("(x: x x) (y: x)", "x"); }

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
        interpret_eq("(x: z) ((x: x x) (x: x x))", "k");
        interpret_eq(&format!("({} (f: x:y: y x)) a b", Y_COMB), "b a");
    }

    #[test]
    fn trick_eq() {
        // This may error if we're copying variables incorrectly, since the replacement
        // will have two functions with the same variable. This is not really a problem
        // but the eq checker might fail.
        interpret_eq("(x: w x x) (x: x)", "z (x: x) (x:x)");
        interpret_eq("(x: w x x) (y: z)", "z (a: b) (c:b)");
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
}
