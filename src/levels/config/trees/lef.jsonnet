{
  name: 'lef',
  description: |||
    Write function LEF that receives a node and returns the first element.
    
    For example:
    - LEF (L V R) -> L
  |||,
  extra_info: |||
    We call it LEF as a shorthand for left, as on the next problems it will be used to retrieve the left child of the node.
  |||,
  test_cases: [
    ['f: f (NODE L V R)', 'L'],
    ['f: f (NODE (x:x) A B)', 'x:x'],
  ],
  provides_constant: true,
  solutions: ['n: n (l:v:r: l)'],
  wrong_solutions: ['n: L', 'l:v:r: l', 'n: (l: l)'],
}
