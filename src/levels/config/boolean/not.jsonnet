local lib = import 'lib.libsonnet';
{
  name: 'not',
  description: |||
    Write function NOT, that is, a function that if given TRUE returns FALSE, and if given FALSE returns TRUE.
    
    TRUE and FALSE are the same as defined in the previous level.
    
    For example:
    - (NOT TRUE) a b -> FALSE a b -> b
    
    Note that you can assume that the used values are always booleans! Input is always well formed.
  |||,
  extra_info: |||
    Notice that you don't NEED to use the constants. And using them as little as possible means solving the problem using the least reductions.
    
    When we say NOT TRUE reduces to FALSE, we're being somewhat loose with terminology. It reduces to a term that _behaves like_ TRUE, for example it could be "a:b: (x:y: x) a b", which is not technically TRUE, but behaves like it.
  |||,
  test_cases: [
    lib.test_false('f: f TRUE'),
    lib.test_true('f: f FALSE'),
  ],
  provides_constant: true,
  solutions: ['b: x:y: b y x', 'b: IF b FALSE TRUE'],
  wrong_solutions: ['f: f'],
}
