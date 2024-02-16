local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  # Doesn't return FALSE if num is not equal, just some trash
  num_is_eq_func(x)::
    '(n: n SND %s)' % [pl.list_no_sentinel(std.repeat(['Z'], x) + ['TRUE'])],
  num_is_eq(term, x):::
    '%s SND %s' % [term, pl.list_no_sentinel(std.repeat(['Z'], x) + ['TRUE'])],
  test_num(term, x)::
    bool.test_true(self.num_is_eq(term, x)),
  # post applied
  inner_list_eq(list):::
    if std.length(list) == 0 then
      error "Can't be empty"
    else if std.length(list) == 1 then
      '(h:t: AND (%s h) (NOT t))' % [self.num_is_eq_func(list[0])]
    else
      '(h:t: AND (%s h) (t %s))' % [self.num_is_eq_func(list[0]), self.inner_list_eq(list[1:])],
  list_num_eq_func(list)::
    if std.length(list) == 0 then
      'EMPTY'
    else
      '(l: l %s)' % [self.inner_list_eq(list)],
  list_num_eq(term, list):::
    if std.length(list) == 0 then
      '%s (h:t:x: FALSE) TRUE' % [term]
    else
      '%s (h:t:x: AND (%s) (%s)) FALSE' % [term, self.num_is_eq('h', list[0]), self.list_num_eq('t', list[1:])]
  ,
  test_list_num(term, list)::
    bool.test_true(self.list_num_eq(term, list)),
}
