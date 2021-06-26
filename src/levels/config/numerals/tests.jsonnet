local lib = import 'lib.libsonnet';
[
  lib.test_num('(f:x: x)', 0),
  lib.test_num('(FALSE)', 0),
  lib.test_num('(f:x: f x)', 1),
  lib.test_num('(f:x: f (f x))', 2),
  lib.test_num('(5)', 5),
  lib.test_list_num('PAIR 5 FALSE', [5]),
  lib.test_list_num('PAIR 3 (PAIR 2 FALSE)', [3, 2]),
]
