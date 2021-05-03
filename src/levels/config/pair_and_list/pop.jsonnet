local lib = import 'lib.libsonnet';
{
  name: 'pop',
  description: |||
    Write function POP, that given a list L with at least one element, reduces to its tail (that is, the list of all elements except the first).
    
    For example:
    - POP [A, B, C, D, E] -> [B, C, D, E]
  |||,
  test_cases: [
    ['f: f %s' % [lib.list(['A'])], lib.list([])],
    ['f: f %s' % [lib.list(['PAIR A B', 'X'])], lib.list(['X'])],
    ['f: f (f %s)' % [lib.list(['A', 'B', 'C', 'D'])], lib.list(['C', 'D'])],
  ],
  provides_constant: true,
  solutions: ['SND', 'l: l FALSE', 'l: l (h:t: t)'],
}
