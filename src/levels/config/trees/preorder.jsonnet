local num = import '../numerals/lib.libsonnet';
local lib = import 'lib.libsonnet';
local test_preorder(list) =
  num.test_list_num('f: f %s' % [lib.bst(list)], std.sort(list));
{
  name: 'preorder',
  description: |||
    Write a function PREORDER that takes a binary tree and returns an array.

    This should use preorder traversal. For a node (L V R):
    - First, add all values in L to the array.
    - Then, add V to the array.
    - Then, add all values in R to the array.

    Examples:
    - PREORDER ((1) 2 (3)) -> [1, 2, 3]
  |||,
  extra_info: |||
    Note that on a BST b using (PREORDER b) will return a sorted array.
  |||,
  test_cases: [
    test_preorder([2, 1, 3]),
    test_preorder([]),
    test_preorder([4, 3, 2]),
    test_preorder([4, 1, 3, 6, 5, 7]),
  ],
  provides_constant: true,
  solutions: [lib.bst_to_preorder()],
  wrong_solutions: [lib.bst_to_inorder()],
}
