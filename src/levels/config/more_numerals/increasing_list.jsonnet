local bool = import '../boolean/lib.libsonnet';
local lib = import '../numerals/lib.libsonnet';
{
  name: 'increasing list',
  description: |||
    Write a function F that takes one number N and reduces to the list of numbers up to it.
    
    Examples:
    - F 0 -> [0]
    - F 3 -> [0, 1, 2, 3]
  |||,
  test_cases: [
    lib.test_num('f: f 0 FST', 0),
    bool.test_true('f: EMPTY (POP (f 0))'),
    lib.test_num('f: f 4 FALSE FALSE TRUE', 2),
    lib.test_num('f: 5 SND (f 5) TRUE', 5),
    bool.test_true('f: EMPTY (4 SND (f 3))'),
  ],
  solutions: ['n: Y (f: a: ZERO a (PAIR n FALSE) (PUSH (SUB n a) (f (PRE a)))) n'],
  wrong_solutions: ['n: (f:x: f n)'],
}
