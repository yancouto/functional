local bool = import '../boolean/lib.libsonnet';
{
  name: 'binary tree',
  description: |||
    A binary tree is a data structure made of tree nodes. Each node may have a left and a right child (the L and R terms in it) and always contains a value (the term V). We use FALSE to indicate an empty binary tree, or to show that a node doesn't have a left or right child.
    
    Write a function BEMPTY that givena binary tree returns whether it is empty or not.
    
    For example:
    - BEMPTY (FALSE 1 FALSE) -> FALSE
    - BEMPTY FALSE -> TRUE
    - BEMPTY ((FALSE 2 FALSE) 1 FALSE) -> FALSE
  |||,
  extra_info: |||
    More formally, if L and R are binary trees and V is any term then:
    - FALSE is an empty binary tree (no values).
    - (L V R) is a binary tree with value V, and all values from the trees L and R.
    
    So ((FALSE 4 (FALSE 20 FALSE)) 4 (FALSE TRUE FALSE)) is a binary tree with values 4, 20, 4, and TRUE.
    
    If you need a hint, look at the hint for the "empty" level in the "pair and list" section.
  |||,
  test_cases: [
    bool.test_false('f: f (NODE FALSE TRUE FALSE)'),
    bool.test_true('f: f FALSE'),
    bool.test_false('f: f (NODE (NODE FALSE FALSE FALSE) 2 (NODE FALSE 3 FALSE))'),
  ],
  provides_constant: false,
  solutions: ['n: n (l:v:r:t: FALSE) TRUE'],
  wrong_solutions: ['NOT', 'EMPTY'],
}
