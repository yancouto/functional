local bool_lib = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'zero',
  description: |||
    Write function ZERO that, given a numeral N, reduces to TRUE if it's zero, and FALSE if it's not.
    
    The input is well formed, that is, it's always a numeral.
  |||,
  extra_info: |||
    Notice that this definition of numeral is quite arbitrary.
    
    In fact, the same is true for booleans, pairs and lists. They could be defined in other ways, as long as the same operations could still be done using them.
  |||,
  test_cases: [
    bool_lib.test_true('f: f 0'),
    bool_lib.test_false('f: f (SUC 0)'),
    bool_lib.test_false('f: f 3'),
    bool_lib.test_false('f: f 5'),
  ],
  provides_constant: true,
  solutions: ['n: n (x: FALSE) TRUE'],
  wrong_solutions: ['n: n FALSE TRUE', 'n: n', 'SND'],
}
