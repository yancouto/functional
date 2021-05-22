use super::{tokenizer::Constant, Node};

fn all_constants_rec<'a>(root: &'a Node, vec: &mut Vec<&'a Constant>) {
    match root {
        Node::Constant(c) => vec.push(c),
        Node::Variable(_) => {},
        Node::Function { variable: _, body } => all_constants_rec(&body, vec),
        Node::Apply { left, right } => {
            all_constants_rec(&left, vec);
            all_constants_rec(&right, vec);
        },
    }
}

pub fn all_constants<'a>(root: &'a Node) -> impl Iterator<Item = &'a Constant> {
    // Can be improved with generators if needed
    let mut v = Vec::new();
    all_constants_rec(&root, &mut v);
    v.into_iter()
}
