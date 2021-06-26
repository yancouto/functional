{
  name: 'if',
  description: |||
    We can define booleans as follows:
    - TRUE  = a: b: a
    - FALSE = a: b: b
    
    That is, TRUE is a two argument function that returns the first element, and FALSE returns the second.
    
    Write function IF, that is, a three-parameter function that is given a boolean, and two other values (say, A and B). If the boolean is TRUE, then it reduces to A, otherwise, it reduces to B. 
    
    For example:
    - IF TRUE a b -> a
    - IF FALSE a b -> b
    
    As always, input is well formed, so the first argument is always a boolean.
  |||,
  extra_info: |||
    You can use constants from levels you have previously completed. See below for a full list of allowed constants. For TRUE and FALSE you don't need to solve any level.
    
    In a single section, you can solve levels in any order, and the order you chose will affect the constants you can use to solve each of them.
    
    Levels are not ordered by difficulty! Choose wisely.
  |||,
  test_cases: [
    ['f: f TRUE A B', 'A'],
    ['f: f FALSE A B', 'B'],
    ['f: f FALSE C D', 'D'],
    ['f: f TRUE  C D', 'C'],
  ],
  before_level_constants: [
    ['TRUE', 'a:b: a'],
    ['FALSE', 'a:b: b'],
  ],
  provides_constant: true,
  solutions: ['x: x', 'b:x:y: b x y'],
}
