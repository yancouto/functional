{
  name: 'simple list',
  description: |||
    A simple list can be create using pairs. It can be defined recursively as:
    - A list with a two terms A and B is the pair (A, B).
    - A list with two or more terms, is represented as (M, rest), where rest is the rest of the list.
    
    So, for example:
    - (a, (b, c)) = f: f a (g: g b c) is a list of three elements.
    
    Write a function that, given a list of five elements, reduces to the third.
  |||,
  extra_info: 'Note that, by definition, all lists have size at least two.',
  test_cases: [
    ['f: f (PAIR A (PAIR B (PAIR C (PAIR D E))))', 'C'],
    ['f: f (PAIR X (PAIR Y (PAIR Z (PAIR W A))))', 'Z'],
  ],
  solutions: ['f: f FALSE FALSE TRUE'],
}
