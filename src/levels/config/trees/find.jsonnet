local bool = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
local test_in(list, x) =
  local f = 'f: f %s %d' % [lib.bst(list), x];
  if std.find(x, list) == [] then
    bool.test_false(f)
  else
    bool.test_true(f);
{
  name: 'find',
  description: |||
    Write a function FIND that takes a binary search tree B and a value V and returns TRUE if V is in any node of B or FALSE otherwise.

    Examples:
    - FIND ((1) 2 (3)) 1 -> TRUE
    - FIND ((1) 2 (3)) 5 -> FALSE
  |||,
  test_cases: [
    test_in([2, 1, 3], 1),
    test_in([3, 2], 5),
    test_in([], 0),
    test_in([5, 3, 4, 6], 4),
    test_in([5, 2, 3, 6], 4),
  ],
  provides_constant: true,
  solutions: [
    'Y (f: b:x: b (l:v:r:z: ZERO (SUB x v) (ZERO (SUB v x) TRUE (f l x)) (f r x)) FALSE)',
    // Less optimised but prettier
    'Y (f: b:x: b (l:v:r:z: (EQ v x) TRUE (ZERO (SUB x v) (f l x) (f r x))) FALSE)',
    // Inefficient but ok
    'Y (f: b:x: b (l:v:r:z: (EQ v x) TRUE (OR (f l x) (f r x))) FALSE)',
  ],
  wrong_solutions: [
    'b:x: TRUE',
    'b:x: b (l:v:r: EQ v x)',
  ],
}
