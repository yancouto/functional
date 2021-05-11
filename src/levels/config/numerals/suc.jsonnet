local lib = import 'lib.libsonnet';
{
  name: 'suc',
  description: |||
    We can define numerals as follows:
    Number N is (f:x: f^N x)
    
    That is, it receives a function f and a value x, and applies f N times to x.
    
    For example:
    - 0 = (f:x: x)
    - 2 = (f:x: f (f x))
    
    Write function SUC that, given a numeral N, reduces to N+1.
    
    The input is well formed, that is, it's always a numeral.
  |||,
  extra_info: |||
    Notice that our numerals are all non-negative.
  |||,
  test_cases: [
    ['f: f 0 (x: A) B', 'A'],
    lib.test_num('f: f 0', 1),
    lib.test_num('f: f 2', 3),
    lib.test_num('f: f 9', 10),
  ],
  provides_constant: true,
  solutions: ['n: (f:x: f (n f x))'],
  wrong_solutions: ['n: (f:x: f n)'],
}
