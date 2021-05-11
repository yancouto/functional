local bool = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'pre',
  description: |||
    Write function PRE that, given a numeral N, reduces to N-1.
    
    If N is zero, then PRE N should reduce to zero.
    
    The input is well formed, that is, it's always a numeral.
  |||,
  extra_info: |||
    Any numeral definition that supports SUC, PRE and ZERO operations are equivalent, even though they may be very different.
  |||,
  test_cases: [
    bool.test_true('f: ZERO (f 1)'),
    bool.test_false('f: ZERO (f 2)'),
    bool.test_false('f: f FALSE'),
    lib.test_num('f: f 0', 0),
    lib.test_num('f: f 3', 2),
    lib.test_num('f: f 10', 9),
    lib.test_num('f: f 15', 14),
  ],
  provides_constant: true,
  solutions: [
    'n: f:x: n (g:h: h (g f)) (u: x) (u: u)',
    'n: f:x: n (p: p (h:t: h (PAIR FALSE x) (PAIR FALSE (f t)))) (PAIR TRUE x) FALSE',
    'n: (n (a:f: f A a) FALSE) (h:t:d: f:x: Y (g: l: l (h:t:d: f (g t)) x) t ) FALSE',
  ],
  wrong_solutions: [
    'n: f:x: n f (i: i) x',
  ],
}
