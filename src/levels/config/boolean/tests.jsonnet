local lib = import 'lib.libsonnet';
[
  lib.test_true('TRUE'),
  lib.test_false('FALSE'),
  lib.test_true('AND TRUE TRUE'),
  // "unresolved" trues and falses
  lib.test_true('(a:b: (x:y: x) a b)'),
  lib.test_false('(a:b: (x:y: y) a b)'),
]
