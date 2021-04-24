{
  name: 'push',
  description: |||
    Write function PUSH, that given a list L and a term M, reduces to a list with M as its first element (head) and L as the rest of the list (tail).
    
    For example:
    - PUSH A (B, (C, (D, E))) -> (A, (B, (C, (D, E))))
    
    Remember, input is always formatted, you can always assume the first argument is a valid list.
  |||,
  test_cases: [
    ['f: f A B', 'PAIR A B'],
    ['f: f A (PAIR B C)', 'PAIR A (PAIR B C)'],
    ['f: (f A B) TRUE', 'A'],
  ],
  provides_constant: true,
  solutions: ['PAIR', 'a:b: f: f a b'],
}
