local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'map',
  description: |||
    Write a function MAP that takes a list L and a function F, and reduces to a list with F applied to every element of L.
    
    Examples:
    - MAP [TRUE, FALSE] NOT -> [FALSE, TRUE]
    - MAP [(A, B), (C, D)] FST -> [A, C]
  |||,
  test_cases: [
    ['f: (f %s FST) TRUE' % [pl.list(['PAIR A B', 'PAIR C D'])], 'A'],
    ['f: (f %s SND) FALSE TRUE' % [pl.list(['PAIR E F', 'PAIR G H'])], 'H'],
    bool.test_true('f: ACC (f %s NOT) AND TRUE' % [pl.list(['FALSE', 'FALSE', 'FALSE'])]),
  ],
  provides_constant: true,
  solutions: ['l:f: Y (g: l: l (h:t:d: PUSH (f h) (g t)) FALSE) l'],
}
