local lib = import '../numerals/lib.libsonnet';
{
  name: 'decompositon',
  description: |||
    Write function DEC that, given a number X, outputs its decomposition in increasing powers of two. That is, it should output a list of unique increasing powers of two, such that its sum is X. It is guaranteed such decomposition always exists and is unique.
    
    For example:
    - DEC 12 -> [4, 8]
    - DEC 8 -> [8]
    - DEC 15 -> [1, 2, 4, 8]
    
    X is at most 15.
  |||,
  test_cases: [
    lib.test_list_num('f: f 5', [1, 4]),
    lib.test_list_num('f: f 8', [8]),
    lib.test_list_num('f: f 13', [1, 4, 8]),
    lib.test_list_num('f: f 15', [1, 2, 4, 8]),
    lib.test_list_num('f: f 10', [2, 8]),
    lib.test_list_num('f: f 0', []),
    lib.test_list_num('f: f 1', [1]),
  ],
  solutions: [
    'x: REVERSE (Y (f: x: p: p (h:t:d: ZERO (SUB x (SUB h 1)) (f x t) (PUSH h (f (SUB x h) t))) FALSE) x (PAIR 8 (PAIR 4 (PAIR 2 (PAIR 1 FALSE)))))',
  ],
}
