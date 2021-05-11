local bool = import '../boolean/lib.libsonnet';
{
  list(lst)::
    if std.length(lst) == 0 then
      'FALSE'
    else
      '(PAIR (%s) %s)' % [lst[0], self.list(lst[1:])],
  list_no_sentinel(lst)::
    if std.length(lst) == 0 then
      error 'empty list'
    else if std.length(lst) == 1 then
      lst[0]
    else
      '(PAIR (%s) %s)' % [lst[0], self.list_no_sentinel(lst[1:])],
}
