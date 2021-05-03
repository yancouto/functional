local bool_lib = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'add',
  description: |||
    Write function ADD that, given two numerals X and Y, reduces to X+Y.
    
    The input is well formed, that is, it's always two numerals.
  |||,
  extra_info: |||
    It's possible to write this function using only SUC, PRE and ZERO, and it would work for any numeral definition.
    
    However, you can use the specifics of this definition to make it easier if you wish.
  |||,
  test_cases: [
    lib.test_num('f: f 3 2', 5),
    lib.test_num('f: f 2 3', 5),
    bool_lib.test_true('f: ZERO (f 0 0)'),
    lib.test_num('f: f 2 0', 2),
    lib.test_num('f: f 0 2', 2),
    lib.test_num('f: f 5 5', 10),
  ],
  provides_constant: true,
  solutions: ['Y (f: a:b: (ZERO a) b (SUC (f (PRE a) b)))'],
}
