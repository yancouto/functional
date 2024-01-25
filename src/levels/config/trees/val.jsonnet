{
  name: 'val',
  description: |||
    Write function VAL that receives a node and returns the middle element.
    
    For example:
    - VAL (L V R) -> V
  |||,
  extra_info: |||
    We call it VAL as a shorthand for value, as on the next problems it will be used to retrieve the value stored in the node.
  |||,
  test_cases: [
    ['f: f (NODE L V R)', 'V'],
    ['f: f (NODE A (x:x) B)', 'x:x'],
  ],
  provides_constant: true,
  solutions: ['n: n (l:v:r: v)'],
  wrong_solutions: ['n: V', 'l:v:r: v'],
}
