{
  name: 'suc',
  description: |||
    We can define numerals as follows:
    Number x is represented by a list of x FALSE's followed by a single identity (x: x).
    
    For example:
    - 0 = (x: x)
    - 2 = (FALSE, (FALSE, I)) = a: a (x:y: y) (b: b (x:y: y) (i: i))
    
    Write function SUC that, given a numeral X, reduces to X+1.
    
    The input is well formed, that is, it's always a numberal.
  |||,
  extra_info: |||
    Notice that our numerals are all non-negative.
  |||,
  test_cases: [
    ['f: f (x: x)', 'PAIR FALSE (x:x)'],
  ],
  provides_constant: true,
  solutions: ['x: PAIR FALSE x'],
}
