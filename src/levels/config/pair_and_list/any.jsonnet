local bool_lib = import '../boolean/lib.libsonnet';
local bool = ['FALSE', 'TRUE'];
local cases = [
  [bool[x] for x in case]
  for case in [[0, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 1], [0, 1, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1]]
];

{
  name: 'any',
  description: |||
    Write a function that, given a list of three booleans, reduces to TRUE if at least one of the booleans is TRUE, and to FALSE if all are FALSE.
  |||,
  test_cases: [
    local term = 'f: f (PAIR %s (PAIR %s (PAIR %s FALSE)))' % [c[0], c[1], c[2]];
    if c[3] == 'TRUE' then
      bool_lib.test_true(term)
    else
      bool_lib.test_false(term)
    for c in cases
  ],
  solutions: ['l: (OR (FST (SND (SND l))) (OR (FST l) (FST (SND l))))', 'l: l (a:b: (OR a (b (x:y: OR x (y TRUE)))))'],
  wrong_solutions: ['OR OR'],
}
