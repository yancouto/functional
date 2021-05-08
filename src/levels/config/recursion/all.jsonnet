local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'map',
  description: |||
    Write a function ALL that takes a list of booleans, and reduces to true if all booleans are TRUE, and FALSE otherwise.
    
    If the list is empty, it should reduce to TRUE, as in this case all the 0 booleans are TRUE.
    
    Examples:
    - ALL [TRUE, TRUE, TRUE] -> TRUE
    - ALL [TRUE, FALSE] -> FALSE
    - ALL [] -> TRUE
    
    The input is always valid.
  |||,
  test_cases: [
    bool.test_true('f: f %s' % [pl.list([])]),
    bool.test_true('f: f %s' % [pl.list(['TRUE', 'TRUE', 'TRUE'])]),
    bool.test_false('f: f %s' % [pl.list(['TRUE', 'FALSE'])]),
    bool.test_true('f: f %s' % [pl.list(['XOR TRUE FALSE', 'OR FALSE TRUE'])]),
    bool.test_false('f: f %s' % [pl.list(['TRUE', 'TRUE', 'FALSE', 'TRUE'])]),
  ],
  provides_constant: true,
  solutions: ['Y (f: l: l (h:t:d: AND h (f t)) TRUE)', 'l: ACC l AND TRUE'],
}
