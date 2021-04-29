{
  list(lst)::
    if std.length(lst) == 1 then
      lst[0]
    else
      '(PAIR %s %s)' % [lst[0], self.list(lst[1:])],
}
