local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'zip',
  description: |||
    Write a function ZIP that takes two lists, and reduces to the list of pairs with elements from the first and second list.
    
    You can assume both lists have exactly the same number of elements.
    
    Examples:
    - ZIP [A, B] [C, D] -> [(A, C), (B, D)]
    - ZIP [] [] -> []
    
    The input is always valid.
  |||,
  test_cases: [
    ['f: f %s %s TRUE FALSE' % [pl.list(['A']), pl.list(['B'])], 'B'],
    bool.test_false('f: f %s %s FALSE' % [pl.list(['A']), pl.list(['B'])]),
    bool.test_false('f: f %s %s' % [pl.list([]), pl.list([])]),
    ['f: f %s %s FALSE TRUE TRUE' % [pl.list(['A', 'B']), pl.list(['C', 'D'])], 'B'],
    bool.test_true('f: ACC (MAP (f %s %s) (p: p OR)) AND TRUE' % [pl.list(['TRUE', 'TRUE', 'FALSE']), pl.list(['FALSE', 'TRUE', 'TRUE'])]),
  ],
  provides_constant: true,
  solutions: ['Y (f: a:b: a (h:t:d: PUSH (PAIR h (b TRUE)) (f t (b FALSE))) FALSE)'],
}
