local lib = import 'lib.libsonnet';
{
  name: 'suc',
  description: |||
    We can define numerals as follows:
    Number x is represented by a list of x FALSE's followed by a single identity (a: a).
    
    For example:
    - 0 = (x: x)
    - 2 = (FALSE, (FALSE, I)) = a: a (x:y: y) (b: b (x:y: y) (i: i))
    
    Write function SUC that, given a numeral X, reduces to X+1.
    
    The input is well formed, that is, it's always a numeral.
  |||,
  extra_info: |||
    Notice that our numerals are all non-negative.
  |||,
  test_cases: [
    ['f: f (x: x)', 'PAIR FALSE (x:x)'],
    lib.test_num('f: f 2', 3),
    lib.test_num('f: f 9', 10),
  ],
  provides_constant: true,
  solutions: ['x: p: p (a:b:b) x', 'x: PAIR FALSE x'],
}
