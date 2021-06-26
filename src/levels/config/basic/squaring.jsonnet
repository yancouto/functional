{
  name: 'squaring',
  description: |||
    Always remember that functions are also values.
    
    Write a two argument function SQR that receives a function and a value, and applies the function to the value twice.
    
    Examples:
    - SQR (x: x) y -> (x: x) ((x: x) y) -> (x: x) y -> y
    - SQR (x: x x) (y: y) -> ... -> (y: y)
    
    Remember that terms are left associative, that is:
    - f f x = (f f) x
    
    Which is not what we want in this problem, but instead "f (f x)", which needs explicit parenthesization.
  |||,
  extra_info: |||
    The main objective of each level is to just write a term that solves the problem.
    
    However, you can also compare your stats to those of other players. Two stats are collected:
    
    - Reductions: How many steps your solution takes to solve the test cases, in average.
    
    - Functions: How many functions are used in your term. Note that this will be equal to the number of ':'.
  |||,
  test_cases: [
    ['f: f (x: x) A', 'A'],
    ['f: f (x: x x) Z', '(Z Z) ((x: x x) Z)'],
    ['f: f (x: x x) (x: x)', '(x: x)'],
    ['f: f (x: x x x) (x: x)', '(x: x)'],
  ],
  solutions: ['f:x: f (f x)'],
}
