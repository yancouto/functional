{
  name: 'snd',
  description: |||
    Write function SND, a function that receives a pair and returns the second element.
    
    For example:
    - SND (A, B) -> B
  |||,
  test_cases: [
    ['f: f (PAIR A B)', 'B'],
    ['f: f (PAIR X (x:x))', 'x: x'],
    ['f: f (PAIR X (x:x)) Z', 'Z'],
  ],
  provides_constant: true,
  solutions: ['p: p (a:b: b)', 'p: p FALSE'],
}
