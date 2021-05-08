local lib = import 'lib.libsonnet';
[
  lib.test_list(lib.list(lst), lst)
  for lst in [
    [],
    ['A'],
    ['A', 'B', 'C', 'D'],
  ]
]
