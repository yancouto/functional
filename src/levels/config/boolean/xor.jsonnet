local lib = import 'lib.libsonnet';
{
  name: 'xor',
  description: |||
    Write function XOR, that is, a function that is given two booleans, reduces to TRUE when exactly one of them is true.
    
    Examples:
    - XOR FALSE FALSE -> FALSE
    - XOR TRUE TRUE   -> FALSE
    - XOR TRUE FALSE  -> TRUE
    
    As always, the input is well formed, both arguments are booleans.
  |||,
  test_cases: [
    lib.test_false('f: f TRUE TRUE'),
    lib.test_true('f: f FALSE TRUE'),
    lib.test_true('f: f TRUE FALSE'),
    lib.test_false('f: f FALSE FALSE'),
  ],
  provides_constant: true,
  solutions: ['a:b: x:y: a (b y x) (b x y)'],
}
