{
  name: 'rig',
  description: |||
    Write function RIG that receives a node and returns the last element.
    
    For example:
    - RIG (L V R) -> R
  |||,
  extra_info: |||
    We call it RIG as a shorthand for right, as on the next problems it will be used to retrieve the right child of the node.
  |||,
  test_cases: [
    ['f: f (NODE L V R)', 'R'],
    ['f: f (NODE A B (x:x))', 'x:x'],
  ],
  provides_constant: true,
  solutions: ['n: n (l:v:r: r)'],
  wrong_solutions: ['n: R', 'l:v:r: r'],
}
