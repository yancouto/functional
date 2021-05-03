local lib = import 'lib.libsonnet';
{
  name: 'pre',
  description: |||
    Write function PRE that, given a non-zero numeral X, reduces to X-1.
    
    The input is well formed, that is, it's always a non-zero numeral.
  |||,
  extra_info: |||
    Notice that this definition of numeral is quite arbitrary.
    
    In fact, you can come up with other numeral definitions, and as long as you can define the same primitives on them (such as SUC and PRE), they're all useful.
  |||,
  test_cases: [
    ['f: f (PAIR FALSE (x: x))', 'a:a'],
    lib.test_num('f: f 3', 2),
    lib.test_num('f: f 10', 9),
  ],
  provides_constant: true,
  solutions: ['x: x (a:b: b)', 'POP'],
}
