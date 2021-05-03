{
  name: 'simple list',
  description: |||
    A simple list can be create using pairs. It can be defined recursively as:
    - An empty list is represented by FALSE.
    - A list with one or more terms, is represented as (M, rest), where M is the first element and rest is the rest of the list.
    
    So, for example:
    - [a, b] = (a, (b, FALSE)) = f: f a (g: g b (x:y: y)) is a list of two elements.
    
    Write a function that, given a list of five elements, reduces to the third.
  |||,
  test_cases: [
    ['f: f (PAIR A (PAIR B (PAIR C (PAIR D (PAIR E FALSE)))))', 'C'],
    ['f: f (PAIR X (PAIR Y (PAIR Z (PAIR W (PAIR A FALSE)))))', 'Z'],
  ],
  solutions: ['f: f FALSE FALSE TRUE'],
}
