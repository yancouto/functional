local bool = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
[
  bool.test_false(lib.list([])),
  bool.test_false('%s FALSE' % [lib.list(['A'])]),
  ['%s TRUE' % [lib.list(['A'])], 'A'],
  ['%s FALSE FALSE TRUE' % [lib.list(['A', 'B', 'C', 'D'])], 'C'],
]
