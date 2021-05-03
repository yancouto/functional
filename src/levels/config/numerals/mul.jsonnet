local bool_lib = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'mul',
  description: |||
    Write function MUL that, given two numerals X and Y, reduces to X.Y, that is, X multiplied by Y.
    
    The input is well formed, that is, it's always two numerals.
  |||,
  test_cases: [
    lib.test_num('f: f 3 2', 6),
    lib.test_num('f: f 2 3', 6),
    bool_lib.test_true('f: ZERO (f 0 0)'),
    lib.test_num('f: f 2 0', 0),
    lib.test_num('f: f 0 2', 0),
    lib.test_num('f: f 1 3', 3),
    lib.test_num('f: f 2 5', 10),
  ],
  provides_constant: true,
  solutions: [
    'Y (f: a:b: (ZERO a) (i:i) ((ZERO (PRE a)) b (ADD b (f (PRE a) b))))',
    'Y (f: a:b: (ZERO a) (i:i) ((ZERO (PRE a)) b (ADD (ADD b b) (f (PRE (PRE a)) b))))',
  ],
}
