local lib = import 'lib.libsonnet';
{
  name: 'and',
  description: |||
    Write function AND, that is, a function that is given two booleans, and only reduces to TRUE if both values are TRUE.
    
    Examples:
    - AND TRUE TRUE  -> TRUE
    - AND FALSE TRUE -> FALSE
    
    As always, the input is well formed, both arguments are booleans.
  |||,
  test_cases: [
    lib.test_true('f: f TRUE TRUE'),
    lib.test_false('f: f FALSE TRUE'),
    lib.test_false('f: f TRUE FALSE'),
    lib.test_false('f: f FALSE FALSE'),
  ],
  provides_constant: true,
  solutions: ['a:b: x:y: a (b x y) y'],
}
