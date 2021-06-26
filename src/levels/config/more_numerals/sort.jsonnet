local lib = import '../numerals/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'sort',
  description: |||
    Write function SORT that, given a list of numbers, reduces to the same elements, ordered from smallest to largest.
    
    For example:
    - SORT [2] -> [2]
    - SORT [4, 3] -> [3, 4]
    - SORT [1, 3, 2] -> [1, 2, 3]
    
    The list has at least one element and at most 4. All elements are distinct.
  |||,
  test_cases: [
    lib.test_list_num('f: f %s' % [pl.list(v)], std.sort(v))
    for v in [[2], [4, 3], [1, 3, 2], [4, 2, 1, 3], [7, 5, 3, 2]]
  ],
  solutions: [
    |||
      Y (f: l:
      l
        (h:t:d:
          (CONCAT
            (f (FILTER t (x: (ZERO (SUB x h)))))
            (PUSH h (f (FILTER t (x: (ZERO (SUB h x))))))
        ))
        FALSE
      )
    |||,
    // TODO: Another approach? Get the minimum, put it first, and iterate.
  ],
  wrong_solutions: ['x: x', 'REVERSE'],
}
