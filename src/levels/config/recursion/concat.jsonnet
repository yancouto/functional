local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'concat',
  description: |||
    Write a function CONCAT that takes two lists and reduces to the concatenation of the two lists.
    
    That is, a list with all the elements from the first list, then all elements from the second list.
    
    Examples:
    - CONCAT [A, B] [C] -> [A, B, C]
    - CONCAT [] [X] -> [X]
  |||,
  test_cases: [
    pl.test_list('f: f %s %s' % [pl.list([]), pl.list(['X'])], ['X']),
    ['f: f %s %s' % [pl.list(['X']), pl.list([])], pl.list(['X'])],
    ['f: f %s %s' % [pl.list(['A', 'B']), pl.list(['C'])], pl.list(['A', 'B', 'C'])],
    ['f: f %s (f %s %s)' % [pl.list(['A', 'B']), pl.list(['C', 'D']), pl.list(['E', 'F'])], pl.list(['A', 'B', 'C', 'D', 'E', 'F'])],
  ],
  provides_constant: true,
  solutions: ['Y (f: a:b: (EMPTY a) b (PUSH (a TRUE) (f (a FALSE) b)))'],
}
