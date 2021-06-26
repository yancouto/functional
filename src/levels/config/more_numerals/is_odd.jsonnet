local bool = import '../boolean/lib.libsonnet';
{
  name: 'is odd',
  description: |||
    Write a function ISODD that takes one number N and reduces to TRUE if it is odd, or FALSE if it is even.
    
    Examples:
    - F 0 -> FALSE
    - F 1 -> TRUE
    - F 32 -> FALSE
  |||,
  test_cases: [
    bool.test_false('f: f 0'),
    bool.test_true('f: f 1'),
    bool.test_false('f: f 2'),
    bool.test_true('f: f 9'),
    bool.test_false('f: f 12'),
  ],
  solutions: ['n: n NOT FALSE'],
  wrong_solutions: ['n: n NOT'],
}
