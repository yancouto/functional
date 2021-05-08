{
  list(lst)::
    if std.length(lst) == 0 then
      'FALSE'
    else
      '(PAIR (%s) %s)' % [lst[0], self.list(lst[1:])],
  applied(lst):::
    if std.length(lst) == 0 then
      'FALSE'
    else
      '%s %s' % [lst[0], self.applied(lst[1:])],
  test_list_rec(term, lst):::
    if std.length(lst) == 0 then
      term
    else
      // We can't use recursion (Y) here because it doesn't resolve fully
      '%s (h:t: PAIR h (%s))' % [term, self.test_list_rec('t', lst[1:])],
  test_list(term, lst)::
    [self.test_list_rec(term, lst), self.list(lst)],
}
