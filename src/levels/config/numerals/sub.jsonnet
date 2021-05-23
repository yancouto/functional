local bool_lib = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'sub',
  description: |||
    Write function SUB that, given two numerals X and Y, reduces to X-Y.
    
    If Y is greater than X, it should reduce to 0 instead.
    
    The input is well formed, that is, it's always two numerals.
  |||,
  test_cases: [
    bool_lib.test_true('f: ZERO (f 0 0)'),
    lib.test_num('f: f 2 0', 2),
    lib.test_num('f: f 0 2', 0),
    lib.test_num('f: f 7 3', 4),
    lib.test_num('f: f 3 7', 0),
    lib.test_num('f: f 7 (f 4 2)', 5),
    lib.test_num('f: ADD 2 (f 1 2)', 2),
  ],
  provides_constant: true,
  solutions: [
    'Y (f: a:b: (ZERO b) a (PRE (f a (PRE b))))',
  ],
  wrong_solutions: [
    'Y (f: a:b: (ZERO a) b (SUC (f (PRE a) b)))',
    'a:b: f:x: b f (a f x)',
  ],
}
