{
  name: 'pop',
  description: |||
    Write function POP, that given a list L with at least two elements, reduces to its tail (that is, the list of all elements except the first).
    
    For example:
    - POP (A (B, (C, (D, E)))) -> (B, (C, (D, E)))
  |||,
  test_cases: [
    ['f: f (PAIR A B)', 'B'],
    ['f: f (PAIR (PAIR A B) (PAIR X Y))', 'PAIR X Y'],
    ['f: f (f (PAIR (PAIR A B) (PAIR X Y)))', 'Y'],
  ],
  provides_constant: true,
  solutions: ['SND', 'l: l FALSE', 'l: l (h:t: t)'],
}
