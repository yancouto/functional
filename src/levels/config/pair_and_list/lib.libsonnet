{
  list(lst)::
    if std.length(lst) == 0 then
      'FALSE'
    else
      '(PAIR (%s) %s)' % [lst[0], self.list(lst[1:])],
}
