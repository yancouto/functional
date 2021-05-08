local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'none',
  description: |||
    Write a function NONE that takes a list of booleans, and reduces to TRUE if all booleans are FALSE, and FALSE otherwise.
    
    If the list is empty, it should reduce to TRUE, as in this case all the 0 booleans are FALSE.
    
    Examples:
    - NONE [FALSE, FALSE, FALSE] -> TRUE
    - NONE [TRUE, FALSE] -> FALSE
    - NONE [] -> TRUE
    
    The input is always valid.
  |||,
  test_cases: [
    bool.test_true('f: f %s' % [pl.list([])]),
    bool.test_true('f: f %s' % [pl.list(['FALSE', 'FALSE', 'FALSE'])]),
    bool.test_false('f: f %s' % [pl.list(['TRUE', 'FALSE'])]),
    bool.test_true('f: f %s' % [pl.list(['AND TRUE FALSE', 'OR FALSE FALSE'])]),
    bool.test_false('f: f %s' % [pl.list(['FALSE', 'FALSE', 'TRUE', 'FALSE'])]),
  ],
  provides_constant: true,
  solutions: ['Y (f: l: l (h:t:d: h FALSE (f t)) TRUE)', 'l: ALL (MAP l NOT)'],
}
