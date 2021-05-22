{
  name: 'pair',
  description: |||
    Let M and N be terms, we can define pairs as follows:
    - (M, N) = f: f M N

    That is, a pair of two elements is a function that receives one argument and applies both elements to it.

    Write function PAIR, a function that receives two arguments and returns a pair made of both elements.

    For example:
    - PAIR A B -> (A, B) = f: f A B
  |||,
  extra_info: |||
    Remember you can solve levels from the same section in any order, and use constants from levels you've previously solved.
  |||,
  test_cases: [
    ['p: p A B', 'f: f A B'],
    ['p: (p A B) (a:b: a)', 'A'],
    ['p: (p A B) (a:b: b)', 'B'],
  ],
  provides_constant: true,
  solutions: ['a:b: f: f a b'],
}
