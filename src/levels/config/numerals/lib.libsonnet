local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  test_num(term, x)::
    bool.test_true('%s SND %s TRUE' % [term, pl.list(std.repeat(['FALSE'], x) + ['TRUE'])]),
}
