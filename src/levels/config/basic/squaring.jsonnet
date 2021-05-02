{
  name: 'squaring',
  description: |||
    Always remember that functions are also values.
    
    Write a function SQR that receives a function and a value, and applies the function to the value twice.
    
    Examples:
    - SQR (x: x) y -> y
    - SQR (x: x x) y -> (y y) (y y)
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
    ['f: f (x: x x x) (x: x)', '(x: x)'],
  ],
  solutions: ['f:x: f (f x)'],
}
