local lib = import 'lib.libsonnet';
{
  name: 'min',
  description: |||
    Write function MIN that, given two numerals X and Y, reduces to the smallest of them.
    
    The input is well formed.
  |||,
  test_cases: [
    lib.test_num('f: f 3 2', 2),
    lib.test_num('f: f 2 3', 2),
    lib.test_num('f: f 5 0', 0),
    lib.test_num('f: f 1 1', 1),
  ],
  provides_constant: true,
  solutions: [
    'x:y: ZERO (SUB x y) x y',
  ],
  wrong_solutions: [
    'SUB',
  ],
}
