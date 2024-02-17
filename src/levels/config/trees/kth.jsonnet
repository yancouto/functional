local num = import '../numerals/lib.libsonnet';
local lib = import 'lib.libsonnet';

local test_kth(list, k) =
  assert std.length(list) >= k;
  num.test_num('f: f %s %d' % [lib.bst(list), k - 1], std.sort(list)[k - 1]);
{
  name: 'kth',
  description: |||
    Write a function KTH that, given a binary search tree B and a positive number K, returns the k-th smallest value in B (0-indexed).

    It is guaranteed that B always has at least K + 1 elements.

    Examples:
    - KTH ((2) 4 (6)) 0 -> 2
    - KTH ((2) 4 (6)) 1 -> 4
    - KTH ((2) 4 (6)) 2 -> 6
  |||,
  extra_info: |||
    Try to avoid visiting the whole tree, it may be big.
  |||,
  test_cases: [
    test_kth([3, 1, 5], 2),
    test_kth([3, 1, 5], 3),
    test_kth([2], 1),
    test_kth([5, 3, 4], 2),
    test_kth([3, 1], 1),
    test_kth([7, 4, 2, 3, 6, 15, 11, 12, 8, 9, 19], 4),
    test_kth([7, 4, 2, 3, 6, 15, 11, 12, 8, 9, 19], 3),
  ],
  provides_constant: true,
  solutions: [
    // traverse in inorder until you get to k, but try to be smart about not using "PRE" which is innefficient
    'b:x: SND (Y (f: b:p: b (l:v:r:z: f l p (i:o: i (h:t:z: h (PAIR FALSE v) (f r t)) (PAIR i o))) (PAIR p p)) b (x (p: PAIR FALSE p) (PAIR TRUE FALSE)))',
    // traverse in inorder until you get to k
    'b:x: SND (Y (f: b:x: b (l:v:r:z: f l x (i:o: ZERO i (PAIR 0 o) (ZERO (PRE i) (PAIR 0 v) (f r (PRE i))))) (PAIR x x)) b (SUC x))',
    // turn to array, get ith
    'b:x: FST (x POP (PREORDER b))',
  ],
  wrong_solutions: [
    'b:x: b (l:v:r: v)',
  ],
}
