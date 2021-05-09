local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'eqblist',
  description: |||
    Write a function EQBLIST that takes two lists of booleans, and reduces to TRUE if they're equal, and FALSE otherwise.
    
    You can assume the lists have the same size.
    
    Examples:
    - EQBLIST [] [] -> TRUE
    - EQBLIST [TRUE, FALSE] [TRUE, FALSE] -> TRUE
    - EQBLIST [TRUE] [FALSE] -> FALSE
  |||,
  test_cases: [
    bool.test_true('f: f %s %s' % [pl.list([]), pl.list([])]),
    bool.test_true('f: f %s %s' % [pl.list(['TRUE', 'FALSE']), pl.list(['OR TRUE FALSE', 'AND TRUE FALSE'])]),
    bool.test_false('f: f %s %s' % [pl.list(['TRUE', 'FALSE']), pl.list(['TRUE', 'TRUE'])]),
    local l = pl.list(['TRUE', 'FALSE', 'TRUE', 'TRUE', 'FALSE']);
    bool.test_true('f: f %s %s' % [l, l]),
    local l1 = ['TRUE', 'FALSE', 'TRUE', 'TRUE', 'FALSE'];
    local l2 = ['TRUE', 'FALSE', 'TRUE', 'FALSE', 'FALSE'];
    bool.test_false('f: f %s %s' % [pl.list(l1), pl.list(l2)]),
  ],
  provides_constant: true,
  solutions: ['a:b: ACC (MAP (ZIP a b) (p: NOT (p XOR))) AND TRUE'],
}
