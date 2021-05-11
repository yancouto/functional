local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  test_num(term, x)::
    bool.test_true('%s SND %s' % [term, pl.list_no_sentinel(std.repeat(['Z'], x) + ['TRUE'])]),
}
