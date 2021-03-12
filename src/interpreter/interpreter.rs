use std::collections::HashMap;

use super::parser::Node;
use super::tokenizer::Variable;

#[derive(Debug, Clone)]
pub enum InterpretError {
    /// Shouldn't really happen
    AlgorithmError,
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

fn interpret_req(
    root: Box<Node>,
    do_apply: bool,
    assigned_values: &mut HashMap<Variable, Box<Node>>,
) -> Result<Box<Node>, InterpretError> {
    println!("IN {:?} {:?} {:?}\n", root, do_apply, assigned_values);
    let ans = Ok(match *root.clone() {
        Node::Apply { left, right } => {
            let left = interpret_req(left, do_apply, assigned_values)?;
            match *left {
                Node::Function { variable, body } if do_apply => {
                    let prev = assigned_values.insert(variable, right);
                    let ans = interpret_req(body, true, assigned_values)?;
                    if let Some(n) = prev {
                        assigned_values.insert(variable, n).assert_some()?;
                    } else {
                        assigned_values.remove(&variable).assert_some()?;
                    }
                    ans
                }
                _ => Box::new(Node::Apply {
                    left,
                    right: interpret_req(right, false, assigned_values)?,
                }),
            }
        }
        Node::Variable(v) => assigned_values
            .get(&v)
            .map(Clone::clone)
            .map(|n| {
                let prev = assigned_values.remove(&v);
                let ans = interpret_req(n, do_apply, assigned_values);
                if let Some(prev_node) = prev {
                    assigned_values.insert(v, prev_node).assert_none()?;
                }
                ans
            })
            .unwrap_or_else(|| Ok(Box::new(Node::Variable(v))))?,
        Node::Function { variable, body } => {
            let prev = assigned_values.remove(&variable);
            let inner = interpret_req(body, false, assigned_values)?;
            if let Some(prev_node) = prev {
                assigned_values.insert(variable, prev_node).assert_none()?;
            }
            Box::new(Node::Function {
                variable,
                body: inner,
            })
        }
        node @ Node::Constant(..) => Box::new(node),
    });
    println!(
        "OUT {:?} {:?} {:?} = {:?}\n",
        root,
        do_apply,
        assigned_values,
        ans.clone()
    );
    ans
}

pub fn interpret(root: Box<Node>) -> Result<Box<Node>, InterpretError> {
    interpret_req(root, true, &mut HashMap::new())
}

#[cfg(test)]
mod test {
    use super::super::parser::test::parse_ok;
    use super::*;

    fn interpret_str(str: &str) -> Box<Node> {
        interpret(parse_ok(str)).unwrap()
    }

    fn interpret_eq(src: &str, expected: &str) {
        assert_eq!(interpret_str(src), parse_ok(expected));
    }

    #[test]
    fn no_apply() {
        interpret_eq("z", "z");
        interpret_eq("y:y", "y : y");
    }

    #[test]
    fn simple_apply() {
        interpret_eq("(x: x) z", "z");
    }
    #[test]
    fn double_apply() {
        interpret_eq("(x: x x) (y z)", "(y z) (y z)");
        interpret_eq("(x: x x) (y: y)", "(y: y)");
        interpret_eq("(x: x x) (y: z)", "z");
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
}
