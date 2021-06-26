local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'filter',
  description: |||
    Write a function FILTER that takes a list L and a function F, and reduces to a list with all elements of L that when applied to F reduce to TRUE.
    
    Examples:
    - FILTER [TRUE, FALSE] NOT -> [FALSE]
    - FILTER [(TRUE, TRUE), (TRUE, FALSE), (FALSE, TRUE)] FST -> [(TRUE, TRUE), (TRUE, FALSE)]
    
    The input is valid. F is a function that always reduces to a boolean when applied to an element of list L.
  |||,
  test_cases: [
    bool.test_false('f: f %s (x: TRUE)' % [pl.list([])]),
    bool.test_false('f: f %s (x: x)' % [pl.list(['FALSE', 'AND TRUE FALSE', 'FALSE'])]),
    bool.test_false('f: f (PUSH (PAIR TRUE TRUE) (PUSH (PAIR FALSE FALSE) (PUSH (PAIR FALSE TRUE) FALSE))) (p: p XOR) FALSE'),
    ['f: f (PUSH (PAIR TRUE TRUE) (PUSH (PAIR FALSE FALSE) (PUSH (PAIR FALSE TRUE) FALSE))) (p: p XOR) TRUE (x:y: y (x B A) B)', 'A'],
  ],
  provides_constant: true,
  solutions: ['l:f: Y (g: l: l (h:t:d: (f h) (PUSH h (g t)) (g t)) FALSE) l'],
}
