use std::collections::HashMap;

use super::parser::{Node, Variable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretError {
    /// Shouldn't really happen
    AlgorithmError,
    /// Interpretation has gone too deep, the expression probaly has no reduction
    TooDeep,
}

trait AlgorithmAssert<T> {
    fn assert_some(self) -> Result<T, InterpretError>;
    fn assert_none(self) -> Result<(), InterpretError>;
}

impl<T> AlgorithmAssert<T> for Option<T> {
    fn assert_some(self) -> Result<T, InterpretError> {
        self.ok_or(InterpretError::AlgorithmError)
    }
    fn assert_none(self) -> Result<(), InterpretError> {
        if self.is_some() {
            Err(InterpretError::AlgorithmError)
        } else {
            Ok(())
        }
    }
}

const MAX_LEVEL: usize = 100;

fn interpret_req(
    level: usize,
    root: Box<Node>,
    do_apply: bool,
    assigned_values: &mut HashMap<Variable, Box<Node>>,
    fully_resolve: bool,
) -> Result<Box<Node>, InterpretError> {
    if level > MAX_LEVEL {
        return Err(InterpretError::TooDeep);
    }
    Ok(match *root {
        Node::Apply { left, right } => {
            let left = interpret_req(level + 1, left, do_apply, assigned_values, fully_resolve)?;
            let right = interpret_req(
                level + 1,
                right,
                fully_resolve,
                assigned_values,
                fully_resolve,
            )?;
            match *left {
                Node::Function { variable, body } if do_apply => {
                    let prev = assigned_values.insert(variable, right);
                    let ans = interpret_req(level + 1, body, true, assigned_values, fully_resolve)?;
                    if let Some(n) = prev {
                        assigned_values.insert(variable, n).assert_some()?;
                    } else {
                        assigned_values.remove(&variable).assert_some()?;
                    }
                    ans
                }
                _ => Box::new(Node::Apply { left, right }),
            }
        }
        Node::Variable(v) => assigned_values
            .get(&v)
            .map(Clone::clone)
            .map(|n| {
                let prev = assigned_values.remove(&v);
                let ans = interpret_req(level + 1, n, do_apply, assigned_values, fully_resolve);
                if let Some(prev_node) = prev {
                    assigned_values.insert(v, prev_node).assert_none()?;
                }
                ans
            })
            .unwrap_or_else(|| Ok(Box::new(Node::Variable(v))))?,
        Node::Function { variable, body } => {
            let prev = assigned_values.remove(&variable);
            let inner = interpret_req(
                level + 1,
                body,
                fully_resolve,
                assigned_values,
                fully_resolve,
            )?;
            if let Some(prev_node) = prev {
                assigned_values.insert(variable, prev_node).assert_none()?;
            }
            Box::new(Node::Function {
                variable,
                body: inner,
            })
        }
        node @ Node::Constant(..) => Box::new(node),
    })
}

pub fn interpret(root: Box<Node>, fully_resolve: bool) -> Result<Box<Node>, InterpretError> {
    interpret_req(0, root, true, &mut HashMap::new(), fully_resolve)
}

pub fn interpret_lazy(root: Box<Node>) -> Result<Box<Node>, InterpretError> {
    interpret(root, false)
}

#[cfg(test)]
mod test {
    use super::super::parser::test::parse_ok;
    use super::*;

    const Y_COMB: &str = "(f: (x: f (x x)) (x: f (x x)))";

    fn interpret_ok(str: &str) -> Box<Node> {
        interpret_lazy(parse_ok(str)).unwrap()
    }

    fn interpret_err(str: &str) -> InterpretError {
        interpret_lazy(parse_ok(str)).unwrap_err()
    }

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
    fn simple_apply() {
        interpret_eq("(x: x) z", "z");
    }
    #[test]
    fn more_apply() {
        interpret_eq("(x: x x) (y z)", "(y z) (y z)");
        interpret_eq("(x: x x) (y: y)", "(y: y)");
        interpret_eq("(x: x x) (y: z)", "z");
        interpret_eq("(x: x x x) (x: x)", "y: y");
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
        interpret_eq("(x: x x) (y: x)", "x");
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
        interpret_eq("(x: z) ((x: x x) (x: x x))", "k");
        interpret_eq(&format!("({} (f: x:y: y x)) a b", Y_COMB), "b a");
    }

    #[test]
    fn recursive() {
        assert_eq!(
            interpret_err(&format!("({} (f: x:y: f x y)) a b", Y_COMB)),
            InterpretError::TooDeep
        );
    }

    #[test]
    fn some_levels() {
        interpret_eq_full("(f: x: f (f x)) (x: x x) A", "(A A) (A A)", true);
    }
}
