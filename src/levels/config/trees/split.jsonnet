local bool = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
local num = import '../numerals/lib.libsonnet';

# TODO: Actually test the shape of the BST
local test_split(orig, x) =
  assert std.find(x, orig) == [];
  local is_list_l = num.list_num_eq_func(std.filter(function(y) y < x, orig));
  local is_list_r = num.list_num_eq_func(std.filter(function(y) y > x, orig));
  local to_arr = lib.bst_to_arr();
  bool.test_true('f: (f ' + lib.bst(orig) + ' ' + x + ') (a:b: AND (' + is_list_l + ' (' + to_arr + ' a)) (' + is_list_r + ' (' + to_arr + ' b)))');
local test_split(orig, x, left, right) =
  assert std.find(x, orig) == [];
  bool.test_true('f: (f %s) (a:b: AND (%s a) (%s b))' % [lib.bst(orig), lib.test_bst_shape_func(left), lib.test_bst_shape_func(right)])
{
  name: 'split',
  description: |||
    Write a function SPLIT that, given a binary search tree T and a number X, returns a pair (A, B) where:
    - A is a BST with all values of T that are less than X.
    - B is a BST with all values of T that are more than X.

    A and B should roughly maintain the same shape as T.
    It is guaranteed that X is NOT present in T.

    Examples:
    - SPLIT ((FALSE 1 FALSE) 3 (FALSE 4 FALSE)) -> ((FALSE 1 FALSE), (FALSE 3 (FALSE 4 FALSE)))
    - SPLIT (FALSE 5 FALSE) 3 -> (FALSE, (FALSE 5 FALSE))
  |||,
  extra_info: |||
    When we say "roughly maintain the same shape", we mean:
    If Y and Z are numbers from the tree T and Z is a (recursive) child of Y, plus both are smaller or both are greater than X, then in the resulting tree they should mantain the fact that Z is a (recursive) child of Y.

    In general terms, you should modify the tree as little as possible when splitting it up in subtrees.
  |||,
  test_cases: [
    test_split([3, 1, 5], 2),
    #test_split([5, 6, 2, 4, 1], 3),
    #test_split([], 3),
    #test_split([2, 1, 4, 3, 6], 5),
  ],
  provides_constant: true,
  solutions: [
    'Y (f: b:x: b (l:v:r:z: (ZERO (SUB x v)) ((f l x) (a:b: (PAIR a (NODE b v r)))) ((f r x) (a:b: (PAIR (NODE l v a) b) ))) (PAIR FALSE FALSE))'
  ],
  wrong_solutions: [
    'b:x: PAIR FALSE b',
  ],
}
