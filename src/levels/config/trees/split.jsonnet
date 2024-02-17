local bool = import '../boolean/lib.libsonnet';
local num = import '../numerals/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
local lib = import 'lib.libsonnet';

local test_split(orig, x) =
  assert std.find(x, orig) == [];
  local left = [y for y in orig if y < x];
  local right = [y for y in orig if y > x];
  bool.test_true('f: (f %s %d) (a:b: AND (%s a) (%s b))' % [lib.bst(orig), x, lib.test_bst_shape_func(left), lib.test_bst_shape_func(right)]);
{
  name: 'split',
  description: |||
    Write a function SPLIT that, given a binary search tree T and a number X, returns a pair (A, B) where:
    - A is a BST with all values of T that are less than X.
    - B is a BST with all values of T that are more than X.

    A and B should roughly maintain the same shape as T.
    It is guaranteed that X is NOT present in T.

    Examples:
    - SPLIT ((1) 3 (4)) -> ((1), (3 (4)))
    - SPLIT (5) 3 -> (FALSE, (5))
  |||,
  extra_info: |||
    When we say "roughly maintain the same shape", we mean:
    If Y and Z are numbers from the tree T and Z is a (recursive) child of Y, plus both are smaller or both are greater than X, then in the resulting tree they should mantain the fact that Z is a (recursive) child of Y.

    In general terms, you should modify the tree as little as possible when splitting it up in subtrees.
  |||,
  test_cases: [
    test_split([3, 1, 5], 2),
    test_split([5, 6, 2, 4, 1], 3),
    test_split([], 3),
    test_split([2, 1, 4, 3, 6], 5),
  ],
  provides_constant: true,
  solutions: [
    'Y (f: b:x: b (l:v:r:z: (ZERO (SUB x v)) ((f l x) (a:b: (PAIR a (NODE b v r)))) ((f r x) (a:b: (PAIR (NODE l v a) b) ))) (PAIR FALSE FALSE))',
    'b:x: (a: (PAIR (BUILD (FILTER a (y: ZERO (SUB y x)))) (BUILD (FILTER a (y: ZERO (SUB x y)))))) (%s b)' % [lib.bst_to_inorder()],
  ],
  wrong_solutions: [
    'b:x: PAIR FALSE b',
  ],
}
