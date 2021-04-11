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
    unused_unbound_vars: BTreeSet<TVariable>,
    unbound_vars:        HashMap<u32, String>,
    bound_vars:          BTreeSet<u32>,
}

impl Data {
    fn new() -> Self {
        Self {
            unused_unbound_vars: ('a'..='z').into_iter().collect(),
            unbound_vars:        HashMap::new(),
            bound_vars:          BTreeSet::new(),
        }
    }

    fn unbound_var(&mut self, var: Variable) -> String {
        let unused = &mut self.unused_unbound_vars;
        self.unbound_vars
            .entry(var.uid)
            .or_insert_with(|| {
                if unused.remove(&var.original) {
                    var.original.into()
                } else if let Some(c) = unused.pop_first() {
                    c.into()
                } else {
                    format!("{}_{}", var.original, var.uid)
                }
            })
            .clone()
    }

    fn get_text(&mut self, var: Variable) -> String {
        if self.bound_vars.contains(&var.uid) {
            var.original.into()
        } else {
            self.unbound_var(var)
        }
    }

    fn with_bound_var<F: FnOnce(&mut Data) -> fmt::Result>(
        &mut self,
        f: F,
        var: Variable,
    ) -> fmt::Result {
        self.bound_vars.insert(var.uid);
        let r = f(self);
        self.bound_vars.remove(&var.uid);
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
            f.write_fmt(format_args!("{}: ", variable.original))?;
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
