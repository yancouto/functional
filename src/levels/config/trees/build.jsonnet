local bool = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
local test_build(list) =
  lib.test_bst('f: f %s' % [pl.list(list)], list);
{
  name: 'build',
  description: |||
    A binary search tree (BST) is a binary tree where all values are <distinct> integers, and it follows this property:
    For a node (L V R), all values in the subtree L are less than V, and all values in the subtree R are more than V.
    
    You can insert a new value X in a BST by starting at the root node and:
    - If the root node is empty, create a node (FALSE X FALSE).
    - Otherwise if (L V R) is not empty and X > V, insert X in R.
    - Otherwise, X < V and you should insert X in L.
    
    Write function BUILD that given an array of integers, inserts them in order in an empty tree and returns the result.
    
    For example:
    - BUILD [5] -> (FALSE 5 FALSE)
    - BUILD [2, 1, 3] -> ((FALSE 1 FALSE) 2 (FALSE 3 FALSE))
    - BUILD [1, 3, 2] -> (FALSE 1 ((FALSE 2 FALSE) 3 FALSE))
  |||,
  extra_info: |||
    Binary search trees are just a fancy way of storing a sorted array.
  |||,
  test_cases: [
    test_build([]),
    test_build([2]),
    test_build([1, 2, 3]),
    test_build([3, 2, 4]),
    test_build([4, 3, 5, 1, 2]),
  ],
  provides_constant: true,
  solutions: ['a: ACC a (Y (f: b:x: b (l:v:r:t: (ZERO (SUB x v)) (NODE (f l x) v r) (NODE l v (f r x)) ) (NODE FALSE x FALSE) )) FALSE'],
  wrong_solutions: [
    'NODE',
    'x:x',
    'a: ACC a (Y (f: b:x: b (l:v:r:t: (ZERO (SUB v x)) (NODE (f l x) v r) (NODE l v (f r x)) ) (NODE FALSE x FALSE) )) FALSE',
  ],
}
