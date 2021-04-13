use std::{
    collections::{BTreeSet, HashMap}, fmt::{self, Display}
};

use super::{Node, TVariable, Variable};

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Constant(c) => f.write_str(c),
            Node::Variable(v) => v.fmt(f),
            Node::Function { variable, body } =>
                f.write_fmt(format_args!("({:?}: {:?})", variable, body)),
            Node::Apply { left, right } => f.write_fmt(format_args!("({:?} {:?})", left, right)),
        }
    }
}

struct Data {
    // Bound variables by level. First the actual name of it, and second the String name, which may
    // have a suffix if the expression has two variables with the same name
    bound_vars:  Vec<String>,
    // How many times this is variable is bound
    bound_times: HashMap<char, usize>,
}

impl Data {
    fn new() -> Self {
        Self {
            bound_vars:  Vec::new(),
            bound_times: HashMap::new(),
        }
    }

    fn get_text(&mut self, var: Variable) -> String {
        let cur_depth = self.bound_vars.len();
        if var.depth >= cur_depth {
            debug_assert!(var.depth == cur_depth, "Can't have more depth than current");
            // Unbound variables start with _ to differentiate them
            format!("_{}", var.original)
        } else {
            self.bound_vars[cur_depth - var.depth - 1].clone()
        }
    }

    fn with_bound_var<F: FnOnce(&mut Data) -> fmt::Result>(
        &mut self,
        f: F,
        var: char,
    ) -> fmt::Result {
        {
            let times = self.bound_times.entry(var).or_insert(0);
            let mut str: String = var.into();
            for _ in 0..*times {
                str.push('\'');
            }
            *times += 1;
            self.bound_vars.push(str);
        }
        let r = f(self);
        *self.bound_times.get_mut(&var).unwrap() -= 1;
        self.bound_vars.pop().unwrap();
        r
    }
}

fn rec(
    node: &Node,
    data: &mut Data,
    f: &mut fmt::Formatter<'_>,
    func_prefix: bool,
    needs_assoc_par: bool,
) -> fmt::Result {
    // also need to carry bound vars
    match node {
        Node::Constant(c) => c.fmt(f)?,
        Node::Variable(v) => data.get_text(*v).fmt(f)?,
        Node::Function { variable, body } => {
            if !func_prefix || needs_assoc_par {
                "(".fmt(f)?;
            }
            f.write_fmt(format_args!("{}: ", variable))?;
            data.with_bound_var(|data| rec(&body, data, f, true, false), *variable)?;
            if !func_prefix || needs_assoc_par {
                ")".fmt(f)?;
            }
        },
        Node::Apply { left, right } => {
            if needs_assoc_par {
                "(".fmt(f)?;
            }
            rec(&left, data, f, false, false)?;
            " ".fmt(f)?;
            rec(&right, data, f, false, true)?;
            if needs_assoc_par {
                ")".fmt(f)?;
            }
        },
    }
    Ok(())
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        rec(self, &mut Data::new(), f, true, false)
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::parser::test::parse_ok;

    fn display_eq(original: &str, display: &str) {
        assert_eq!(display, &format!("{}", parse_ok(original)));
    }

    #[test]
    fn test_display() {
        display_eq("A", "A");
        display_eq("(x:x) A", "(x: x) A");
        display_eq("x:y:A", "x: y: A");
        display_eq("(x:x)", "x: x");
        display_eq("(x:x x)(x:x x)", "(x: x x) (x: x x)")
    }

    #[test]
    fn test_display_func_prefix() {
        display_eq("a:b:c: a", "a: b: c: a");
        display_eq("x (x: (y: y x))", "x (x: y: y x)");
        display_eq("f:x: (f (f x))", "f: x: f (f x)");
    }

    #[test]
    fn test_display_right_associative() {
        display_eq("((a b) c)", "a b c");
        display_eq("(a (b c))", "a (b c)");
        display_eq("((a b) c) d", "a b c d");
        display_eq("((a: b) c)", "(a: b) c");
        display_eq("x: x ((a b) c)", "x: x (a b c)");
    }
}
