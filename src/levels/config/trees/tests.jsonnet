local lib = import 'lib.libsonnet';
local bool = import '../boolean/lib.libsonnet';
local num = import '../numerals/lib.libsonnet';
[
  bool.test_false(lib.bst([])),
  num.test_num('%s (l:v:r: v)' % [lib.bst([3])], 3),
  [lib.bst([2, 1]), 'NODE (NODE FALSE 1 FALSE) 2 FALSE'],
  # Test return value is really a single expression
  ['(x:A) %s' % [lib.bst([2,1,3])], 'A'],
]