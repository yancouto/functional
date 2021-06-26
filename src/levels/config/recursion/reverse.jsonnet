local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'reverse',
  description: |||
    Write a function REVERSE that takes a list, and reduces to the reversed list, that is, where the first element is now the last, the second is the second to last, and so on.
    
    Examples:
    - REVERSE [a, b, c] -> [c, b, a]
    - REVERSE [x] -> [x]
    
    The input is always valid.
  |||,
  test_cases: [
    ['f: f %s TRUE' % [pl.list(['A', 'B', 'C'])], 'C'],
    ['f: f %s FALSE TRUE' % [pl.list(['A', 'B', 'C'])], 'B'],
    ['f: f %s FALSE FALSE TRUE' % [pl.list(['A', 'B', 'C'])], 'A'],
    bool.test_true('f: EMPTY (f %s)' % [pl.list([])]),
    ['f: f %s TRUE' % [pl.list(['Z'])], 'Z'],
  ],
  provides_constant: true,
  solutions: ['l: Y (f: l: r: l (h:t:d: f t (PUSH h r)) r) l FALSE'],
}
