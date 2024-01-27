local num = import '../numerals/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'bsize',
  description: |||
    Write function BSIZE that takes a binary tree and outputs its size, that is, it ignores all its values and outputs the total number of nodes in the tree.
    
    For example:
    - BSIZE (FALSE 10 FALSE) -> 1
    - BSIZE (FALSE TRUE (FALSE FALSE FALSE)) -> 2
  |||,
  test_cases: [
    num.test_num('f: f FALSE', 0),
    num.test_num('f: f (NODE FALSE FALSE FALSE)', 1),
    num.test_num('f: f %s' % lib.bst([1, 2, 3]), 3),
    num.test_num('f: f %s' % lib.bst([2, 1, 3]), 3),
    num.test_num('f: f %s' % lib.bst([3, 1, 2, 6, 4]), 5),
  ],
  provides_constant: true,
  solutions: ['Y (y: n: n (l:v:r:z: SUC (ADD (y l) (y r))) 0)'],
  wrong_solutions: ['VAL'],
}
