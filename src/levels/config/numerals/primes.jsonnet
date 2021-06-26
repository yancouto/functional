local lib = import 'lib.libsonnet';
{
  name: 'primes',
  description: |||
    Write function PRIM that, given a number X, outputs its decomposition in increasing primes. That is, it should output a list of increasing primes, such that the multiplication of all its elements is X. It is guaranteed such decomposition always exists and is unique.
    
    A prime number is a number greater than 1 which is only divisible by 1 and itself.
    
    For example:
    - PRIM 12 -> [2, 2, 3]
    - PRIM 7 -> [7]
    - PRIM 15 -> [3, 5]
    
    X is at least 2 and at most 50.
  |||,
  test_cases: [
    lib.test_list_num('f: f 2', [2]),
    lib.test_list_num('f: f 7', [7]),
    lib.test_list_num('f: f 12', [2, 2, 3]),
    lib.test_list_num('f: f 13', [13]),
    lib.test_list_num('f: f 9', [3, 3]),
  ],
  solutions: [
    |||
      x: Y (f: p: x:
      p
        (h:t:d:
          ZERO (x (y: ZERO y (PRE h) (PRE y)) h)
            (PUSH h (f p (DIV x h)))
            (f t x)
        )
        (ZERO (PRE x) FALSE (PUSH x FALSE))
      )
      (PUSH 2 (PUSH 3 FALSE)) x
    |||,
  ],
}
