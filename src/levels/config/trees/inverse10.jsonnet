local bool = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
local test_build(list) =
  lib.test_bst('f: f %s' % [pl.list(list)], list);
{
  name: 'inverse 10',
  description: |||
    Write a function F that, given a binary search tree with distinct values between 0 and 9, returns an ordered array with all 0-9 numbers that are NOT in the tree.
    
    For example, assuming numbers in 0-4:
    - F ((FALSE 0 FALSE) 2 (FALSE 3 FALSE)) -> [1, 4]
    - F FALSE -> [0, 1, 2, 3, 4]
  |||,
  extra_info: |||
  |||,
  test_cases: [
    test_build([]),
    test_build([2]),
    test_build([1, 2, 3]),
    test_build([3, 2, 4]),
    test_build([4, 3, 5, 1, 2]),
  ],
  provides_constant: false,
  solutions: [],
  wrong_solutions: [
  ],
}
