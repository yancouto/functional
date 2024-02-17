local bool = import '../boolean/lib.libsonnet';
local num = import '../numerals/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
local lib = import 'lib.libsonnet';
local test_6(list) =
  num.test_list_num('f: f %s' % [lib.bst(list)], [x for x in std.range(1, 6) if std.find(x, list) == []]);
{
  name: 'inverse 6',
  description: |||
    Write a function F that, given a binary search tree with distinct values between 1 and 6, returns an ordered array with all 1-6 numbers that are NOT in the tree.

    For example:
    - F ((FALSE 1 FALSE) 3 (FALSE 4 FALSE)) -> [2, 5, 6]
    - F FALSE -> [1, 2, 3, 4, 5, 6]
  |||,
  test_cases: [
    test_6([4, 2, 6]),
    test_6([3, 1, 2, 5, 4, 6]),
    test_6([3, 1, 5]),
    test_6([]),
  ],
  provides_constant: false,
  solutions: [
    'b: FILTER %(list6)s (x: %(notcontains)s b x)' % {
      list6: '(PUSH 1 (PUSH 2 (PUSH 3 (PUSH 4 (PUSH 5 (PUSH 6 FALSE))))))',
      notcontains: '(Y (f: b:x: b (l:v:r:t: (EQ v x) FALSE ((ZERO (SUB v x)) (f r x) (f l x))) TRUE))',
    },
    't: Y (f: t:a:b:p: t (l:v:r:z: f l a (PRE v) (f r v b p)) ((ZERO (SUB b a)) p (f FALSE a (PRE b) (PUSH b p)))) t 0 6 FALSE',
  ],
  wrong_solutions: [
    'x: x',
    // Recursing in the wrong order
    't: Y (f: t:a:b:p: t (l:v:r:z: (f r v b (f l a (PRE v) p)) ((ZERO (SUB b a)) p (f FALSE a (PRE b) (PUSH b p)))) t 0 6 FALSE',
  ],
}
