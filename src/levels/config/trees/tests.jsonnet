local lib = import 'lib.libsonnet';
local bool = import '../boolean/lib.libsonnet';
local num = import '../numerals/lib.libsonnet';
local test_not_bst(term, list) =
  local test = lib.test_bst(term, list);
  assert(test[1] == 'A');
  [test[0], 'B'];
[
  bool.test_false(lib.bst([])),
  num.test_num('%s (l:v:r: v)' % [lib.bst([3])], 3),
  [lib.bst([2, 1]), 'NODE (NODE FALSE 1 FALSE) 2 FALSE'],
  # Test return value is really a single expression
  ['(x:A) %s' % [lib.bst([2,1,3])], 'A'],
  lib.test_bst("FALSE", []),
  test_not_bst("FALSE", [1]),
  lib.test_bst(lib.bst([1, 2, 3]), [1, 2, 3]),
  lib.test_bst(lib.bst([2, 3, 1]), [2, 1, 3]),
  test_not_bst(lib.bst([1, 2]), [1]),
  test_not_bst(lib.bst([2, 1, 3]), [2, 1, 4]),
]