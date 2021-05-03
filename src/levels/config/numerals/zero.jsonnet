local bool_lib = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'zero',
  description: |||
    Write function ZERO that, given a numeral X, reduces to TRUE if it's zero, and FALSE if it's not.
    
    The input is well formed, that is, it's always a numeral.
  |||,
  extra_info: |||
    Any numeral definition that supports SUC, PRE and ZERO operations are equivalent, even though they may be very different.
  |||,
  test_cases: [
    bool_lib.test_true('f: f 0'),
    bool_lib.test_false('f: f (SUC 0)'),
    bool_lib.test_false('f: f 3'),
    bool_lib.test_true('f: f (PRE 1)'),
  ],
  provides_constant: true,
  solutions: ['x: x (a:b: a)', 'FST'],
}
