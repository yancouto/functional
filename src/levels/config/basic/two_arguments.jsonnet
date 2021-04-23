{
  name: 'two arguments',
  description: |||
    Functions in terms only accept a single parameter. However, you can simulate multiple arguments by having multiple chained functions.
    
    Example:
    - (x: y: x) a b -> (y: a) b -> a
    - (x: y: x y) a b -> (y: a y) b -> a b
    
    Write a function with two arguments that swaps the order of their terms.
  |||,
  extra_info: |||
    Notice that terms are left associative, that is:
    - a b c = ((a b) c)
    
    And that's why you can call "multi parameter functions" like this:
    - FUNC x y z = (((FUNC x) y) z)
  |||,
  test_cases: [
    ['f: f A B', 'B A'],
    ['f: f X (x: x)', 'X'],
    ['f: f (x: x) A', 'A (y: y)'],
  ],
  solutions: ['a:b: b a'],
}
