local lib = import 'lib.libsonnet';
{
  name: 'div',
  description: |||
    Write function DIV that, given two numerals X and Y, reduces to X/Y, that is, X divided by Y.
    
    Examples:
    - DIV 6 3 -> 2
    - DIV 3 1 -> 3
    - DIV 0 2 -> 0
    
    It is guaranteed that X is always divisible by Y.
  |||,
  test_cases: [
    lib.test_num('f: f 6 3', 2),
    lib.test_num('f: f 6 2', 3),
    lib.test_num('f: f 2 2', 1),
    lib.test_num('f: f 1 1', 1),
    lib.test_num('f: f 4 1', 4),
    lib.test_num('f: f 25 5', 5),
    lib.test_num('f: f 0 2', 0),
  ],
  provides_constant: true,
  solutions: [
    'a:b: Y (f: a: ZERO a 0 (ADD 1 (f (SUB a b)))) a',
  ],
}
