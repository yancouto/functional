local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
local F = 'FALSE';
local T = 'TRUE';
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
    bool.test_true('f: EQBLIST %s (f %s %s)' % [pl.list(l) for l in [case[0] + case[1], case[0], case[1]]])
    for case in [
      [[], [F]],
      [[T], []],
      [[T, F], [F]],
      [[F, F], [F, F, T]],
    ]
  ] + [
    bool.test_true('f: EQBLIST %s (f %s (f %s %s))' % [pl.list(l) for l in [[T, F, F, F, T, T], [T, F], [F, F], [T, T]]]),
    bool.test_false('f: (f %s %s)' % [pl.list(l) for l in [[], []]]),
    ['f: (f %s %s) FALSE TRUE' % [pl.list(l) for l in [['A'], ['B', 'C']]], 'B'],
  ],
  provides_constant: true,
  solutions: ['Y (f: a:b: a (h:t:d: PUSH h (f t b)) b)', 'Y (f: a:b: (EMPTY a) b (PUSH (a TRUE) (f (a FALSE) b)))'],
}
