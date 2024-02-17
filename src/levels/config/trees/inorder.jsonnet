local num = import '../numerals/lib.libsonnet';
local lib = import 'lib.libsonnet';
local test_inorder(list) =
  num.test_list_num('f: f %s' % [lib.bst(list)], list);
{
  name: 'inorder',
  description: |||
    Write a function INORDER that takes a binary tree and returns an array.

    This should use inorder traversal. For a node (L V R):
    - First, add V to the array.
    - Then, add all values in L to the array.
    - Then, add all values in R to the array.

    Examples:
    - INORDER ((1) 2 (3)) -> [2, 1, 3]
  |||,
  extra_info: |||
    Note that on a BST b using (BUILD (INORDER b)) will reconstruct the same BST with exactly the same shape.
  |||,
  test_cases: [
    test_inorder([2, 1, 3]),
    test_inorder([]),
    test_inorder([2, 3, 4]),
    test_inorder([4, 1, 3, 6, 5, 7]),
  ],
  provides_constant: true,
  solutions: [lib.bst_to_inorder()],
  wrong_solutions: [lib.bst_to_preorder()],
}
