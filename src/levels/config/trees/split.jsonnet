local bool = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'split',
  description: |||
    Split bin tree
  |||,
  test_cases: [
    # TODO
  ],
  provides_constant: true,
  solutions: [],
  wrong_solutions: [
    'b: PAIR b b',
  ],
}
