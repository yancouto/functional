local bool_lib = import '../boolean/lib.libsonnet';
{
  name: 'eq',
  description: |||
    Write function eq that, given two numerals X and Y, reduces to TRUE if X = Y and FALSE otherwise.
    
    Examples:
    - EQ 1 1 -> TRUE
    - EQ 1 0 -> FALSE

    The input is well formed, that is, it's always two numerals.
  |||,
  test_cases: [
    bool_lib.test_true('f: f 0 0'),
    bool_lib.test_true('f: f 1 1'),
    bool_lib.test_false('f: f 2 1'),
    bool_lib.test_false('f: f 2 4'),
    bool_lib.test_true('f: f 5 (ADD 2 3)'),
  ],
  provides_constant: true,
  solutions: [
    'x:y: AND (ZERO (SUB x y)) (ZERO (SUB y x))',
    'x:y: ZERO (x (z: ZERO z 100 (PRE z)) y)',
  ],
  wrong_solutions: [
    'x:y: ZERO (SUB x y)',
    'x:y: ZERO (SUB y x)',
  ],
}
