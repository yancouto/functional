use std::{
    collections::{HashMap, HashSet}, fmt::{self, Display}
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
    bound_vars:       Vec<String>,
    // How many times this is variable is bound
    bound_times:      HashMap<char, usize>,
    // Unbound variables that are shadowed by a variable of the same name, somewhere in the term
    shadowed_unbound: HashSet<TVariable>,
}

impl Data {
    fn new(shadowed: HashSet<TVariable>) -> Self {
        Self {
            bound_vars:       Vec::new(),
            bound_times:      HashMap::new(),
            shadowed_unbound: shadowed,
        }
    }

    fn get_text(&mut self, var: Variable) -> String {
        let cur_depth = self.bound_vars.len();
        if var.depth >= cur_depth {
            debug_assert!(var.depth == cur_depth, "Can't have more depth than current");
            if self.shadowed_unbound.contains(&var.original) {
                // Shadowed unbound variables start with _ to differentiate them
                format!("_{}", var.original)
            } else {
                var.original.into()
            }
        } else {
            self.bound_vars[cur_depth - var.depth - 1].clone()
        }
    }

    fn get_suffix(&self, var: TVariable) -> String {
        self.bound_times
            .get(&var)
            .map(|n| "'".repeat(*n))
            .unwrap_or_default()
    }

    fn with_bound_var<F: FnOnce(&mut Data) -> fmt::Result>(
        &mut self,
        f: F,
        var: TVariable,
    ) -> fmt::Result {
        self.bound_vars
            .push(var.to_string() + &self.get_suffix(var));
        *self.bound_times.entry(var).or_insert(0) += 1;
        let r = f(self);
        *self.bound_times.get_mut(&var).unwrap() -= 1;
        self.bound_vars.pop().unwrap();
        r
    }
}

fn rec_display(
    node: &Node,
    data: &mut Data,
    f: &mut fmt::Formatter<'_>,
    func_prefix: bool,
    needs_assoc_par: bool,
) -> fmt::Result {
    match node {
        Node::Constant(c) => c.fmt(f)?,
        Node::Variable(v) => data.get_text(*v).fmt(f)?,
        Node::Function { variable, body } => {
            if !func_prefix || needs_assoc_par {
                "(".fmt(f)?;
            }
            f.write_fmt(format_args!("{}{}: ", variable, data.get_suffix(*variable)))?;
            data.with_bound_var(|data| rec_display(&body, data, f, true, false), *variable)?;
            if !func_prefix || needs_assoc_par {
                ")".fmt(f)?;
            }
        },
        Node::Apply { left, right } => {
            if needs_assoc_par {
                "(".fmt(f)?;
            }
            rec_display(&left, data, f, false, false)?;
            " ".fmt(f)?;
            rec_display(&right, data, f, false, true)?;
            if needs_assoc_par {
                ")".fmt(f)?;
            }
        },
    }
    Ok(())
}

fn mark_shadowed_unbound_variables(
    node: &Node,
    map: &mut HashMap<TVariable, u32>,
    shadowed: &mut HashSet<TVariable>,
    cur_depth: usize,
) {
    match node {
        Node::Constant(_) => {},
        Node::Variable(v) =>
            if v.depth == cur_depth && *map.get(&v.original).unwrap_or(&0) > 0 {
                shadowed.insert(v.original);
            },
        Node::Function { variable, body } => {
            *map.entry(*variable).or_insert(0) += 1;
            mark_shadowed_unbound_variables(body, map, shadowed, cur_depth + 1);
            *map.get_mut(variable).unwrap() -= 1;
        },
        Node::Apply { left, right } => {
            mark_shadowed_unbound_variables(left, map, shadowed, cur_depth);
            mark_shadowed_unbound_variables(right, map, shadowed, cur_depth);
        },
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut shadowed = HashSet::new();
        mark_shadowed_unbound_variables(self, &mut HashMap::new(), &mut shadowed, 0);
        rec_display(self, &mut Data::new(shadowed), f, true, false)
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::{
        interpreter::test::interpret_ok_full, parser::test::{parse_ok, ConvertToNode}
    };

    fn display_eq(original: &str, display: &str) {
        assert_eq!(display, &format!("{}", parse_ok(original)));
    }
    fn interpret_eq(original: &str, display: &str) {
        assert_eq!(display, &format!("{}", interpret_ok_full(original, true)));
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

    #[test]
    fn test_display_unbound() {
        // Simple variables don't need change
        display_eq("a", "a");
        display_eq("(x: x) x", "(x: x) x");
        // Need some interpretation to create unbound vars with same names
        // as bound vars
        interpret_eq("(x: y: x) x", "y: x");
        interpret_eq("(x: y: x) z", "y: z");
        interpret_eq("(x: y: x) y", "y: _y");
        interpret_eq("(x: y: x) y", "y: _y");
        // possible tricky case. Variable appears first without shadow binding
        // and later with shadow
        interpret_eq("y ((x: y: x) y)", "_y (y: _y)");
        interpret_eq("z ((x: y: x) z)", "z (y: z)");
    }

    #[test]
    fn test_same_name_vars() {
        assert_eq!(
            "-: -': - -'",
            &format!("{}", ((), ((), (1.n(), 0.n()).n()).n()).n())
        );
        assert_eq!(
            "-: -': -' -",
            &format!("{}", ((), ((), (0.n(), 1.n()).n()).n()).n())
        );
    }
}
