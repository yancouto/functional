local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  num_is_eq(term, x):::
    '%s SND %s' % [term, pl.list_no_sentinel(std.repeat(['Z'], x) + ['TRUE'])],
  test_num(term, x)::
    bool.test_true(self.num_is_eq(term, x)),
  list_num_eq(term, list):::
    if std.length(list) == 0 then
      '%s (h:t:x: FALSE) TRUE' % [term]
    else
      '%s (h:t:x: AND (%s) (%s)) FALSE' % [term, self.num_is_eq('h', list[0]), self.list_num_eq('t', list[1:])]
  ,
  test_list_num(term, list)::
    bool.test_true(self.list_num_eq(term, list)),
}
