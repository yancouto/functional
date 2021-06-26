local lib = import 'lib.libsonnet';
{
  name: 'max',
  description: |||
    Write function MAX that, given two numerals X and Y, reduces to the largest of them.
    
    The input is well formed.
  |||,
  test_cases: [
    lib.test_num('f: f 3 2', 3),
    lib.test_num('f: f 2 3', 3),
    lib.test_num('f: f 5 0', 5),
    lib.test_num('f: f 1 1', 1),
  ],
  provides_constant: true,
  solutions: [
    'x:y: ZERO (SUB x y) y x',
    'x:y: ADD (SUB x y) y',
  ],
  wrong_solutions: [
    'SUB ADD',
    'x:y: ZERO (SUB x y) x y',
  ],
}
