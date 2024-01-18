{
  name: 'push',
  description: |||
    Write function PUSH, that given a term M and a list L, reduces to a list with M as its first element (head) and L as the rest of the list (tail).
    
    For example:
    - PUSH A [B, C, D] -> [A, B, C, D] = (A, (B, (C, (D, FALSE))))
    
    Remember, input is well formed, you can always assume the second argument is a valid list.
  |||,
  test_cases: [
    ['f: f A FALSE', 'PAIR A FALSE'],
    ['f: f A (PAIR B FALSE)', 'PAIR A (PAIR B FALSE)'],
    ['f: f A (PAIR B (PAIR C FALSE))', 'PAIR A (PAIR B (PAIR C FALSE))'],
    ['f: (f A B) TRUE', 'A'],
  ],
  provides_constant: true,
  solutions: ['PAIR', 'a:b: f: f a b'],
}
