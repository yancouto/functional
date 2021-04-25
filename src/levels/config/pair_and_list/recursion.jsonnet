{
  name: 'recursion',
  description: |||
    Something
  |||,
  test_cases: [
    ['y: y (f: x: x) A', 'A'],
  ],
  solutions: ['f: (x: f (x x)) (x: f (x x))'],
}
