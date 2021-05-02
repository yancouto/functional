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
  extra_info: |||
    Notice that, when using constants, you don't "pay" for their inner functions in your stats.
    
    So writing (TRUE a b) and ((x:y: x) a b) both have the same effect, but the second one counts two extra functions in your stats.
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
