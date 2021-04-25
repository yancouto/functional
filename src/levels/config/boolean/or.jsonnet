local lib = import 'lib.libsonnet';
{
  name: 'or',
  description: |||
    Write function OR, that is, a function that is given two booleans, and only reduces to FALSE if both values are FALSE.
    
    Examples:
    - AND FALSE FALSE -> FALSE
    - AND FALSE TRUE  -> TRUE
    
    As always, the input is well formed, both arguments are booleans.
  |||,
  test_cases: [
    lib.test_true('f: f TRUE TRUE'),
    lib.test_true('f: f FALSE TRUE'),
    lib.test_true('f: f TRUE FALSE'),
    lib.test_false('f: f FALSE FALSE'),
  ],
  provides_constant: true,
  solutions: ['a:b: x:y: a x (b x y)', 'a:b: NOT (AND (NOT a) (NOT b))'],
}
