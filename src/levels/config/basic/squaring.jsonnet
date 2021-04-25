{
  name: 'squaring',
  description: |||
    Always remember that functions are also values.
    
    Write a function SQR that receives a function and a value, and applies the function to the value twice.
    
    Examples:
    - SQR (x: x) y -> y
    - SQR (x: x x) y -> (y y) (y y)
  |||,
  test_cases: [
    ['f: f (x: x) A', 'A'],
    ['f: f (x: x x) Z', '(Z Z) ((x: x x) Z)'],
    ['f: f (x: x x x) (x: x)', '(x: x)'],
  ],
  solutions: ['f:x: f (f x)'],
}
